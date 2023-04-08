use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo};
use winit::window::Window;

pub fn create_swapchain(
    physical_device: &Arc<PhysicalDevice>,
    device: Arc<Device>,
    surface: Arc<Surface>,
) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
    let caps = physical_device
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = Some(
        physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
    );

    Swapchain::new(
        device,
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count,
            image_format,
            image_extent: surface
                .object()
                .unwrap()
                .clone()
                .downcast::<Window>()
                .unwrap()
                .inner_size()
                .into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    )
    .unwrap()
}

pub fn create_framebuffers_from_swapchain_images(
    images: &[Arc<SwapchainImage>],
    render_pass: Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}
