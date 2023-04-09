# Introduction to swapchains

Since we are going to draw to a window which is ultimately on the screen, things are a bit special.
If you were going to write directly to the window's surface, you would introduce tearing and other
strange artifacts, because you would be updating an image that's already visible on a screen.
To ensure that only complete images are shown, Vulkan uses what is called a *swapchain*.

> **Note**: See also [the wikipedia article for a swap 
> chain](https://en.wikipedia.org/wiki/Swap_Chain).

A swapchain is a group of one or multiple images, sometimes two images but most commonly three. If
you have ever heard terms such as *double buffering* or *triple buffering*, it refers to having
respectively two or three swapchain images.

The idea behind a swapchain is to draw to one of its images while another one of these images is
being shown on the screen. When we are done drawing we ask the swapchain to show the image we have
just drawn to, and in return the swapchain gives us drawing access to another of its images.

## (Optional) Checking for swapchain support

As you may recall, previously we just selected the first physical device available:

```rust
let physical = instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");
```

However, some devices may not support swapchain creation or wouldn't be the best option. So, in 
this optional sub-chapter, we are going to write a simple function to filter devices for specific 
Vulkan extension support and select the best device. In a real application, this could be your 
"default" or "recommended" device, and the user could choose any other if needed.

The first step is to select all the extensions needed for your application:

```rust
use vulkano::device::DeviceExtensions;

let device_extensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};
```

Next, we are going to enumerate all the devices and filter them by supported extensions:

```rust
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};

instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .filter(|p| p.supported_extensions().contains(&device_extensions))
    // continues bellow
```

Some devices that passed the test may not have the needed queue family(ies) to present images to 
the surface or even support graphical operations. So, we are going to filter them and at the same 
time select the first queue family that is suitable:

```rust
    .filter_map(|p| {
        p.queue_family_properties()
            .iter()
            .enumerate()
            // Find the first first queue family that is suitable.
            // If none is found, `None` is returned to `filter_map`,
            // which disqualifies this physical device.
            .position(|(i, q)| {
                q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
            })
            .map(|q| (p, q as u32))
    })
    // continues bellow
```

All the physical devices that pass the filters above are suitable for the application.
However, not every device is equal, some are preferred over others. Now, we assign each
physical device a score, and pick the device with the lowest ("best") score.

```rust
    .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,

        // Note that there exists `PhysicalDeviceType::Other`, however,
        // `PhysicalDeviceType` is a non-exhaustive enum. Thus, one should
        // match wildcard `_` to catch all unknown device types.
        _ => 4,
    })
    .expect("no device available");
```

In the end, your new function for selecting the best physical device should look like this:

```rust
use std::sync::Arc;

// crop
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::DeviceExtensions;
use vulkano::swapchain::Surface;
use winit::window::Window;

fn select_physical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
    device_extensions: &DeviceExtensions,
) -> (Arc<PhysicalDevice>, u32) {
    instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                // Find the first first queue family that is suitable.
                // If none is found, `None` is returned to `filter_map`,
                // which disqualifies this physical device.
                .position(|(i, q)| {
                    q.queue_flags.graphics && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,

            // Note that there exists `PhysicalDeviceType::Other`, however,
            // `PhysicalDeviceType` is a non-exhaustive enum. Thus, one should
            // match wildcard `_` to catch all unknown device types.
            _ => 4,
        })
        .expect("no device available")
}

fn main() {
    // crop

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let (physical_device, queue_family_index) = select_physical_device(
        &instance, 
        &surface, 
        &device_extensions,
    );

    // crop
}
```

## Updating logical device creation

Now that we have our desired physical device, the next step is to create a logical device that can 
support the swapchain.

To do that, we need to pass all the previously required extensions:

```rust
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

let (device, mut queues) = Device::new(
    physical_device.clone(),
    DeviceCreateInfo {
        queue_create_infos: vec![QueueCreateInfo {
            queue_family_index,
            ..Default::default()
        }],
        enabled_extensions: device_extensions,
        ..Default::default()
    },
)
.expect("failed to create device");

let queue = queues.next().unwrap();
```

## Creating the swapchain

Swapchains have a lot of properties: the format and dimensions of their images, an optional
transformation, a presentation mode, and so on. We have to specify a value for each of these
parameters when we create the swapchain. Therefore, we have to query the
capabilities of the surface.

```rust
let caps = physical_device
    .surface_capabilities(&surface, Default::default())
    .expect("failed to get surface capabilities");
```

Of all of these properties, we only care about some of them, mainly the dimensions of the image 
(which have to be constrained between a minimum and a maximum), the behavior when it comes to 
transparency (composite alpha), and the format of the images.

```rust
let dimensions = surface.window().inner_size();
let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
let image_format = Some(
    physical_device
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0,
);
```

Combining everything, we can create the swapchain:

```rust
use vulkano::image::ImageUsage;
use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};

let (swapchain, images) = Swapchain::new(
    device.clone(),
    surface.clone(),
    SwapchainCreateInfo {
        min_image_count: caps.min_image_count + 1, // How many buffers to use in the swapchain
        image_format,
        image_extent: dimensions.into(),
        image_usage: ImageUsage {
            color_attachment: true,  // What the images are going to be used for
            ..Default::default()
        },
        composite_alpha,
        ..Default::default()
    },
)
.unwrap();
```

It's good to have `min_image_count` be at least one more than the minimal, to give a bit more 
freedom to the image queue.

For additional information, check the
[swapchain documentation](https://docs.rs/vulkano/0.33.0/vulkano/swapchain/index.html#swapchains).

Next: [Other initialization](/guide/windowing/other-initialization)
