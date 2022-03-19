use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, Queue};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::swapchain::{Surface, Swapchain};
use winit::window::Window;

pub fn create_swapchain(
  physical_device: &PhysicalDevice,
  device: Arc<Device>,
  surface: Arc<Surface<Window>>,
  queue: Arc<Queue>,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
  let caps = surface
    .capabilities(*physical_device)
    .expect("failed to get surface capabilities");

  let dimensions: [u32; 2] = surface.window().inner_size().into();
  let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
  let format = caps.supported_formats[0].0;

  Swapchain::start(device, surface)
    .num_images(caps.min_image_count)
    .format(format)
    .dimensions(dimensions)
    .usage(ImageUsage::color_attachment())
    .sharing_mode(&queue)
    .composite_alpha(composite_alpha)
    .build()
    .expect("failed to create swapchain")
}

pub fn create_framebuffers_from_swapchain_images(
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
