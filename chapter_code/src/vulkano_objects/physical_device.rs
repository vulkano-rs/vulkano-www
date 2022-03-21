use std::sync::Arc;

use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::device::DeviceExtensions;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::window::Window;

pub fn select_physical_device<'a>(
  instance: &'a Arc<Instance>,
  surface: Arc<Surface<Window>>,
  device_extensions: &DeviceExtensions,
) -> (PhysicalDevice<'a>, QueueFamily<'a>) {
  let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
    .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
    .filter_map(|p| {
      p.queue_families()
        .find(|&q| q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false))
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
