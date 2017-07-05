# Creating a device

In [the previous section](/guide/initialization) we created an instance and chosen a physical
device from this instance.

But initialization isn't finished yet. Before being able to do anything, we have to create a
***device***. A *device* is an object that represents an open channel of communication with a
*physical device*, and it is probably the most important object of the Vulkan API.

## About queues

Just like it is possible to use multiple threads in your program running on the CPU, it is also
possible to run multiple operations in parallel on the GPU of your graphics card. The Vulkan
equivalent of a CPU thread is a ***queue***. Queues are grouped by **queue families**.

The queue families of a physical device can be enumerated like this:

```rust
for family in physical.queue_families() {
    println!("Found a queue family with {:?} queue(s)", family.queues_count());
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
Vulkan implementation which queues we want to use. Let's choose a single queue that we will use for
all our operations.

```rust
let queue_family = physical.queue_families()
    .find(|&q| q.supports_graphics())
    .expect("couldn't find a graphical queue family");
```

Creating a device returns two things: the device itself, but also a list of *queue objects* that
will later allow us to submit operations.

```rust
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::instance::Features;

let (device, mut queues) = {
    Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
};
```

Just like creating an instance, creating a device takes additional parameters which we aren't going
to cover yet.

Once this function call succeeds we have an open channel of communication with a Vulkan device!

Since it is possible to request multiple queues, the `queues` variable returned by the function is
in fact an iterator. In this example code this iterator contains just one element, so let's
extract it:

```rust
let queue = queues.next().unwrap();
```

We now have our `device` and our `queue`, which means that we are ready to ask the GPU to perform
operations.
