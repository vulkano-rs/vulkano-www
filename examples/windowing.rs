// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! This example contains the source code of the fourth part of the guide at http://vulkano.rs.
//!
//! It is not commented, as the explanations can be found in the guide itself.

use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{
  AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, Version};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, RenderPass, Subpass};
use vulkano::swapchain::{Surface, Swapchain};
use vulkano_win::VkSurfaceBuild;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

#[derive(Default, Copy, Clone)]
struct Vertex {
  position: [f32; 2],
}

mod vs {
  vulkano_shaders::shader! {
      ty: "vertex",
      src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
gl_Position = vec4(position, 0.0, 1.0);
}"
  }
}

mod fs {
  vulkano_shaders::shader! {
      ty: "fragment",
      src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
f_color = vec4(1.0, 0.0, 0.0, 1.0);
}"
  }
}

fn select_physical_device<'a>(
  instance: &'a Arc<Instance>,
  surface: Arc<Surface<Window>>,
  device_extensions: &DeviceExtensions,
) -> (PhysicalDevice<'a>, QueueFamily<'a>) {
  let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
    .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
    .filter_map(|p| {
      p.queue_families()
        .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        .map(|q| (p, q))
    })
    .min_by_key(|(p, _)| match p.properties().device_type {
      PhysicalDeviceType::DiscreteGpu => 0,
      PhysicalDeviceType::IntegratedGpu => 1,
      PhysicalDeviceType::VirtualGpu => 2,
      PhysicalDeviceType::Cpu => 3,
      PhysicalDeviceType::Other => 4,
    })
    .expect("no device available");

  (physical_device, queue_family)
}

fn get_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain<Window>>) -> Arc<RenderPass> {
  vulkano::single_pass_renderpass!(
      device.clone(),
      attachments: {
          color: {
              load: Clear,
              store: Store,
              format: swapchain.format(),  // set the format to use the same as the swapchain
              samples: 1,
          }
      },
      pass: {
          color: [color],
          depth_stencil: {}
      }
  )
  .unwrap()
}

fn get_framebuffers(
  images: &[Arc<SwapchainImage<Window>>],
  render_pass: Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
  images
    .iter()
    .map(|image| {
      let view = ImageView::new(image.clone()).unwrap();
      Framebuffer::start(render_pass.clone())
        .add(view)
        .unwrap()
        .build()
        .unwrap()
    })
    .collect::<Vec<_>>()
}

fn get_command_buffers(
  device: Arc<Device>,
  queue: Arc<Queue>,
  pipeline: Arc<GraphicsPipeline>,
  framebuffers: &Vec<Arc<Framebuffer>>,
  vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
  framebuffers
    .iter()
    .map(|framebuffer| {
      let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::MultipleSubmit,
      )
      .unwrap();

      builder
        .begin_render_pass(
          framebuffer.clone(),
          SubpassContents::Inline,
          vec![[0.0, 0.0, 1.0, 1.0].into()],
        )
        .unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .draw(vertex_buffer.len() as u32, 1, 0, 0)
        .unwrap()
        .end_render_pass()
        .unwrap();

      Arc::new(builder.build().unwrap())
    })
    .collect()
}

fn main() {
  let required_extensions = vulkano_win::required_extensions();
  let instance = Instance::new(None, Version::V1_1, &required_extensions, None).unwrap();

  let event_loop = EventLoop::new();
  let surface = WindowBuilder::new()
    .build_vk_surface(&event_loop, instance.clone())
    .unwrap();

  let device_extensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::none()
  };

  let (physical_device, queue_family) =
    select_physical_device(&instance, surface.clone(), &device_extensions);

  let (device, mut queues) = {
    Device::new(
      physical_device,
      &Features::none(),
      &physical_device
        .required_extensions()
        .union(&device_extensions), // new
      [(queue_family, 0.5)].iter().cloned(),
    )
    .expect("failed to create device")
  };

  let queue = queues.next().unwrap();

  let (swapchain, images) = {
    let caps = surface
      .capabilities(physical_device)
      .expect("failed to get surface capabilities");

    let dimensions: [u32; 2] = surface.window().inner_size().into();
    let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;
    Swapchain::start(device.clone(), surface.clone())
      .num_images(caps.min_image_count)
      .format(format)
      .dimensions(dimensions)
      .usage(ImageUsage::color_attachment())
      .sharing_mode(&queue)
      .composite_alpha(composite_alpha)
      .build()
      .expect("failed to create swapchain")
  };

  let render_pass = get_render_pass(device.clone(), swapchain.clone());
  let framebuffers = get_framebuffers(&images, render_pass.clone());

  vulkano::impl_vertex!(Vertex, position);

  let vertex1 = Vertex {
    position: [-0.5, -0.5],
  };
  let vertex2 = Vertex {
    position: [0.0, 0.5],
  };
  let vertex3 = Vertex {
    position: [0.5, -0.25],
  };
  let vertex_buffer = CpuAccessibleBuffer::from_iter(
    device.clone(),
    BufferUsage::all(),
    false,
    vec![vertex1, vertex2, vertex3].into_iter(),
  )
  .unwrap();

  let vs = vs::load(device.clone()).expect("failed to create shader module");
  let fs = fs::load(device.clone()).expect("failed to create shader module");

  let viewport = Viewport {
    origin: [0.0, 0.0],
    dimensions: surface.window().inner_size().into(),
    depth_range: 0.0..1.0,
  };

  let pipeline = GraphicsPipeline::start()
    .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
    .vertex_shader(vs.entry_point("main").unwrap(), ())
    .input_assembly_state(InputAssemblyState::new())
    .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
    .fragment_shader(fs.entry_point("main").unwrap(), ())
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    .build(device.clone())
    .unwrap();

  let command_buffers = get_command_buffers(
    device.clone(),
    queue.clone(),
    pipeline.clone(),
    &framebuffers,
    vertex_buffer.clone(),
  );

  event_loop.run(|event, _, control_flow| match event {
    Event::WindowEvent {
      event: WindowEvent::CloseRequested,
      ..
    } => {
      *control_flow = ControlFlow::Exit;
    }
    _ => (),
  });
}
