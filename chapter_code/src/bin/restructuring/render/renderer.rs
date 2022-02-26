use std::sync::Arc;

use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
  self, AcquireError, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture,
  SwapchainCreationError,
};
use vulkano::sync::{self, FenceSignalFuture, FlushError, GpuFuture, JoinFuture, NowFuture};
use vulkano_win::VkSurfaceBuild;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use chapter_code::shaders::static_triangle;
use chapter_code::vulkano_objects;
use chapter_code::vulkano_objects::buffers::Vertex2dBuffer;

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
  vertex_buffer: Arc<Vertex2dBuffer>,
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

    let device_extensions = DeviceExtensions {
      khr_swapchain: true,
      ..DeviceExtensions::none()
    };

    let (physical_device, queue_family) =
      vulkano_objects::physical_device::select_physical_device(&instance, surface.clone(), &device_extensions);

    let (device, mut queues) = {
      Device::new(
        physical_device,
        &Features::none(),
        &physical_device
          .required_extensions()
          .union(&device_extensions),
        [(queue_family, 0.5)].iter().cloned(),
      )
      .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let (swapchain, images) = vulkano_objects::swapchain::create_swapchain(
      &physical_device,
      device.clone(),
      surface.clone(),
      queue.clone(),
    );

    let render_pass = vulkano_objects::render_pass::create_render_pass(device.clone(), swapchain.clone());
    let framebuffers = vulkano_objects::swapchain::create_framebuffers_from_swapchain_images(
      &images,
      render_pass.clone(),
    );

    let vertex_shader =
      static_triangle::vs::load(device.clone()).expect("failed to create shader module");
    let fragment_shader =
      static_triangle::fs::load(device.clone()).expect("failed to create shader module");

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

    let vertex_buffer =
      vulkano_objects::buffers::create_static_triangle_vertex_buffer(device.clone());

    let command_buffers = vulkano_objects::command_buffers::create_only_vertex_command_buffers(
      device.clone(),
      queue.clone(),
      pipeline.clone(),
      &framebuffers,
      vertex_buffer.clone(),
    );

    Self {
      surface,
      device,
      queue,
      swapchain,
      images,
      render_pass,
      framebuffers,
      vertex_buffer,
      vertex_shader,
      fragment_shader,
      viewport,
      pipeline,
      command_buffers,
      _instance: instance,
    }
  }

  pub fn handle_swapchain_recreation(&mut self) {
    let (new_swapchain, new_images) = match self
      .swapchain
      .recreate()
      .dimensions(self.surface.window().inner_size().into())
      .build()
    {
      Ok(r) => r,
      Err(SwapchainCreationError::UnsupportedDimensions) => return,
      Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
    };

    self.swapchain = new_swapchain;
    self.framebuffers = vulkano_objects::swapchain::create_framebuffers_from_swapchain_images(
      &new_images,
      self.render_pass.clone(),
    );
  }

  pub fn handle_window_resize(&mut self) {
    self.handle_swapchain_recreation();
    self.viewport.dimensions = self.surface.window().inner_size().into();

    self.pipeline = vulkano_objects::pipeline::create_pipeline(
      self.device.clone(),
      self.vertex_shader.clone(),
      self.fragment_shader.clone(),
      self.render_pass.clone(),
      self.viewport.clone(),
    );

    self.command_buffers = vulkano_objects::command_buffers::create_only_vertex_command_buffers(
      self.device.clone(),
      self.queue.clone(),
      self.pipeline.clone(),
      &self.framebuffers,
      self.vertex_buffer.clone(),
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
}
