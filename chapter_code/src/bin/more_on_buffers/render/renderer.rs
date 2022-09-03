use std::sync::Arc;

use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo};
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::{GraphicsPipeline, Pipeline};
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    self, AcquireError, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture,
    SwapchainCreateInfo, SwapchainCreationError,
};
use vulkano::sync::{self, FenceSignalFuture, FlushError, GpuFuture, JoinFuture, NowFuture};
use vulkano_win::VkSurfaceBuild;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use chapter_code::game_objects::Square;
use chapter_code::models::SquareModel;
use chapter_code::shaders::movable_square;
use chapter_code::vulkano_objects;
use chapter_code::vulkano_objects::buffers::ImmutableBuffers;
use chapter_code::Vertex2d;

pub type Fence = FenceSignalFuture<
    PresentFuture<
        CommandBufferExecFuture<
            JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture<Window>>,
            Arc<PrimaryAutoCommandBuffer>,
        >,
        Window,
    >,
>;

pub struct Renderer {
    surface: Arc<Surface<Window>>,
    _instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<winit::window::Window>>>,
    render_pass: Arc<RenderPass>,
    framebuffers: Vec<Arc<Framebuffer>>,
    buffers: ImmutableBuffers<Vertex2d, movable_square::vs::ty::Data>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    viewport: Viewport,
    pipeline: Arc<GraphicsPipeline>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
}

impl<'a> Renderer {
    pub fn initialize(event_loop: &EventLoop<()>) -> Self {
        let instance = vulkano_objects::instance::get_instance();

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        {
            // window configuration
            let window = surface.window();
            window.set_title("Movable Square");
            window.set_inner_size(LogicalSize::new(600.0f32, 600.0));
        }

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };

        let (physical_device, queue_family) =
            vulkano_objects::physical_device::select_physical_device(
                &instance,
                surface.clone(),
                &device_extensions,
            );

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
                enabled_extensions: device_extensions, // new
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let queue = queues.next().unwrap();

        let (swapchain, images) = vulkano_objects::swapchain::create_swapchain(
            &physical_device,
            device.clone(),
            surface.clone(),
        );

        let render_pass =
            vulkano_objects::render_pass::create_render_pass(device.clone(), swapchain.clone());
        let framebuffers = vulkano_objects::swapchain::create_framebuffers_from_swapchain_images(
            &images,
            render_pass.clone(),
        );

        let vertex_shader =
            movable_square::vs::load(device.clone()).expect("failed to create shader module");
        let fragment_shader =
            movable_square::fs::load(device.clone()).expect("failed to create shader module");

        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: surface.window().inner_size().into(),
            depth_range: 0.0..1.0,
        };

        let pipeline = vulkano_objects::pipeline::create_pipeline(
            device.clone(),
            vertex_shader.clone(),
            fragment_shader.clone(),
            render_pass.clone(),
            viewport.clone(),
        );

        let buffers = ImmutableBuffers::initialize::<SquareModel>(
            device.clone(),
            pipeline.layout().set_layouts().get(0).unwrap().clone(),
            images.len(),
            queue.clone(),
        );

        let command_buffers = vulkano_objects::command_buffers::create_simple_command_buffers(
            device.clone(),
            queue.clone(),
            pipeline.clone(),
            &framebuffers,
            &buffers,
        );

        Self {
            surface,
            device,
            queue,
            swapchain,
            images,
            render_pass,
            framebuffers,
            buffers,
            vertex_shader,
            fragment_shader,
            viewport,
            pipeline,
            command_buffers,
            _instance: instance,
        }
    }

    pub fn recreate_swapchain(&mut self) {
        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: self.surface.window().inner_size().into(),
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        self.swapchain = new_swapchain;
        self.framebuffers = vulkano_objects::swapchain::create_framebuffers_from_swapchain_images(
            &new_images,
            self.render_pass.clone(),
        );
    }

    pub fn handle_window_resize(&mut self) {
        self.recreate_swapchain();
        self.viewport.dimensions = self.surface.window().inner_size().into();

        self.pipeline = vulkano_objects::pipeline::create_pipeline(
            self.device.clone(),
            self.vertex_shader.clone(),
            self.fragment_shader.clone(),
            self.render_pass.clone(),
            self.viewport.clone(),
        );

        self.command_buffers = vulkano_objects::command_buffers::create_simple_command_buffers(
            self.device.clone(),
            self.queue.clone(),
            self.pipeline.clone(),
            &self.framebuffers,
            &self.buffers,
        );
    }

    pub fn get_image_count(&self) -> usize {
        self.images.len()
    }

    pub fn acquire_swapchain_image(
        &self,
    ) -> Result<(usize, bool, SwapchainAcquireFuture<Window>), AcquireError> {
        swapchain::acquire_next_image(self.swapchain.clone(), None)
    }

    pub fn synchronize(&self) -> NowFuture {
        let mut now = sync::now(self.device.clone());
        now.cleanup_finished();

        now
    }

    pub fn flush_next_future(
        &self,
        previous_future: Box<dyn GpuFuture>,
        swapchain_acquire_future: SwapchainAcquireFuture<Window>,
        image_i: usize,
    ) -> Result<Fence, FlushError> {
        previous_future
            .join(swapchain_acquire_future)
            .then_execute(self.queue.clone(), self.command_buffers[image_i].clone())
            .unwrap()
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_i)
            .then_signal_fence_and_flush()
    }

    pub fn update_uniform(&self, index: usize, square: &Square) {
        let mut uniform_content = self.buffers.uniforms[index]
            .0
            .write()
            .unwrap_or_else(|e| panic!("Failed to write to uniform buffer\n{}", e));

        uniform_content.color = square.color;
        uniform_content.position = square.position;
    }
}
