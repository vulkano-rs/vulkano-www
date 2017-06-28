# Creating a device

Our short-term goal is to ask your video card to perform an operation. For this we have created an
instance and chosen a physical device from this instance.

Just like it is possible to use multiple threads in your program running on the CPU, it is also
possible to run multiple operations in parallel on the GPU of your graphics card. The Vulkan
equivalent of a CPU core is a *queue*. Queues are grouped by *queue families*.

The queue families of a physical device can be enumerated like this:

    for family in physical_device.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

While some implementations only have one family with one queue, some others have three or four
families with up to sixteen queues in some of these families.

When we ask the device to perform an operation, we have to submit the command to a specific queue.
Some queues support only graphical operations, some others support only compute operations, and
some others support both.

## Creating a device

But initialization isn't finished yet.

A `Device` object is an open channel of communication with a physical device. It is probably the
most important object of the Vulkan API.

    let (device, mut queues) = {
        Device::new(&physical, physical.supported_features(), &DeviceExtensions::none(), None,
                    [(queue, 0.5)].iter().cloned()).expect("failed to create device")
    };

We now have an open channel of communication with a Vulkan device!

In the rest of this article, we are going to ask the device to copy data from a buffer to
another. Copying data is an operation that you do very often in Vulkan, so let's get used
to it early.
