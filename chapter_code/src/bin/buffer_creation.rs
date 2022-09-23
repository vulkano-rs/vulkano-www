// Copyright (c) 2017 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

//! This is the source code of the "Buffer Creation" chapter at http://vulkano.rs.
//!
//! It is not commented, as the explanations can be found in the guide itself.

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::sync::{self, GpuFuture};
use vulkano::VulkanLibrary;

fn main() {
    // Initialization
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    let physical = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    // Device creation
    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let queue = queues.next().unwrap();

    // Example operation
    let source_content = 0..64;
    let source = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage {
            transfer_src: true,
            ..Default::default()
        },
        false,
        source_content,
    )
    .expect("failed to create buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage {
            transfer_dst: true,
            ..Default::default()
        },
        false,
        destination_content,
    )
    .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    // Start the execution
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush() // same as signal fence, and then flush
        .unwrap();
    // Wait for the GPU to finish
    future.wait(None).unwrap();

    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();
    assert_eq!(&*src_content, &*destination_content);

    println!("Everything succeeded!");
}
