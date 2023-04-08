# Device creation

In [the previous section](/guide/initialization) we created an instance and chose a physical
device from this instance.

But initialization isn't finished yet. Before being able to do anything, we have to create a
***device***. A *device* is an object that represents an open channel of communication with a
*physical device*, and it is probably the most important object of the Vulkan API.

## About queues

Just like how it's possible to use multiple threads in your program running on the CPU, it's also
possible to run multiple operations in parallel on the GPU of your graphics card. The Vulkan
equivalent of a CPU thread is a ***queue***. Queues are grouped by **queue families**.

The queue families of a physical device can be enumerated like this:

```rust
for family in physical_device.queue_family_properties() {
    println!("Found a queue family with {:?} queue(s)", family.queue_count);
}
```

While some implementations only provide one family with one queue, some others have three or four
families with up to sixteen queues in some of these families.

> **Note**: If you want to get a more precise idea of the queue families provided by the various
> Vulkan implementations, you can go to [vulkan.gpuinfo.org](http://vulkan.gpuinfo.org), click on
> the report you want, and open the "Queue families" tab.

Whenever we want the device to perform an operation, we have to submit this operation to a specific
queue. Some queues support only graphical operations, some others support only compute operations,
and some others support both.

## Creating a device

The reason why queues are relevant right now is in order to create a *device*, we have to tell the
Vulkan implementation which type of queues we want to use. Queues are grouped into *queue families*,
which describe their capabilities. Let's locate a queue family that supports graphical operations:

```rust
use vulkano::device::QueueFlags;

let queue_family_index = physical_device
    .queue_family_properties()
    .iter()
    .enumerate()
    .position(|(_queue_family_index, queue_family_properties)| {
        queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
    })
    .expect("couldn't find a graphical queue family") as u32;
```

Once we have the index of a viable queue family, we can use it to create the device:

```rust
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};

let (device, mut queues) = Device::new(
    physical_device,
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
```

Creating a device returns two things: the device itself, but also a list of *queue objects* that
will later allow us to submit operations.

Once this function call succeeds we have an open channel of communication with a Vulkan device!

Since it is possible to request multiple queues, the `queues` variable returned by the function is
in fact an iterator. In this example code this iterator contains just one element, so let's
extract it:

```rust
let queue = queues.next().unwrap();
```

We now have our `device` and our `queue`, which means that we are ready to ask the GPU to perform
operations.

Next: [Creating a buffer](/guide/buffer-creation)
