use std::sync::Arc;

use vulkano::device::Device;
use vulkano::render_pass::RenderPass;
use vulkano::swapchain::Swapchain;
use winit::window::Window;

pub fn create_render_pass(
  device: Arc<Device>,
  swapchain: Arc<Swapchain<Window>>,
) -> Arc<RenderPass> {
  vulkano::single_pass_renderpass!(
      device.clone(),
      attachments: {
          color: {
              load: Clear,
              store: Store,
              format: swapchain.image_format(),
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
