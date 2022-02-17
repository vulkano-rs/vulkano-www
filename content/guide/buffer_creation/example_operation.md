# Example operation

Now that we are familiar with devices, queues, and buffers, we are going to see how to ask the GPU
to actually do something.

What we are going to ask in this example is very simple: we will ask it to copy data from one
buffer to another.

## Creating the buffers

The first step is to create two `CpuAccessibleBuffer`s: the source and the destination. This
was covered in [the previous section](/guide/buffer-creation).

```rust
let source_content = 0..64;
let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
    .expect("failed to create buffer");

let destination_content = (0..64).map(|_| 0);
let destination = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, destination_content)
    .expect("failed to create buffer");
```

The iterators might look a bit tricky. The `source_content` iterator produces 64 values ranging
from 0 to 63. The `dest_content` iterator produces 64 values that are all equal to 0.
In other words, once created the source buffer contains sixty-four values ranging from 0 to 63
while the destination buffer contains sixty-four 0s.

## Command buffers

In order to ask the GPU to perform an operation, we need to create a type of object that we
haven't covered yet: ***command buffer***.

With Vulkan and vulkano you can't just execute commands one by one, as it would be too inefficient.
Instead, we need to build a *command buffer* that contains a list of commands that we want to
execute.

> **Note**: Submitting a command to the GPU can take up to several hundred microseconds, which is
> why we submit as many things as we can at once.

> **Note**: OpenGL (Vulkan's predecessor) allows you to execute commands one by one, but in reality
> implementations buffer commands internally into command buffers. In other words, OpenGL
> automatically does what Vulkan requires us to do manually. In practice OpenGL's automatic
> buffering often causes more harm than good in performance-critical applications.

Here is how you create a command buffer:

```rust
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};

let mut builder = AutoCommandBufferBuilder::primary(
    device.clone(),
    queue.family(),
    CommandBufferUsage::OneTimeSubmit,
)
.unwrap();

builder.copy_buffer(source.clone(), dest.clone()).unwrap();

let command_buffer = builder.build().unwrap();
```

As you can see, it is very straight-forward. We create a *builder*, add a copy command to it with
`copy_buffer`, then turn that builder into an actual command buffer with `build`. Like we saw in
[the buffers creation section](/guide/buffer-creation), we call `clone()` multiple times but we
 only clone `Arc`s.

<!-- todo: Explain more about secondary command buffers -->
Vulkan supports primary and secondary command buffers. Secondary command buffers allow you to
store functionality that you can reuse in the primary command buffer, but they can't be sent to
the gpu directly. We won't cover them here, but you can read
[more about them](https://docs.rs/vulkano/0.28.0/vulkano/command_buffer/index.html).

One thing to notice is that the `AutoCommandBufferBuilder::primary()` method takes as
parameter a queue family. This must be the queue family that the command buffer is going to run on.
In this example we don't have much choice anyway (as we only use one queue and thus one queue
family), but when you design a real program you have to be aware of this requirement.

## Submission and synchronization

And "now" we submit the command buffer so that it gets executed:

```rust
use vulkano::sync;
use vulkano::sync::GpuFuture;

sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap();
```

The `.then_execute()` method returns an object that represents the execution of the command buffer.

After submitting the command buffer, we might be tempted to try to read the content of the
`destination` buffer as demonstrated in [the previous section](/guide/buffer-creation). However
calling `destination.read()` now may sometimes return an error, because the buffer could
still be being written by the GPU.

Submitting an operation doesn't wait for the operation to be complete. Instead it just sends some
kind of signal to the GPU to instruct it that it must start processing the command buffer, and the
actual processing is performed asynchronously.

In order to read the content of `destination` and make sure that our copy succeeded, we need to
wait until the operation is complete.

First, we need to tell the gpu that it should signal when it's finished, by using a special
object called a *fence*:

```rust
let future = sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush()  // signal to the cpu and start executing
    .unwrap();
```

We will better cover what a *fence* is in a later section of the guide and what signalling it means.
The "future" object will store the information about the execution.

Next, we wait for the GPU to finish executing:

```rust
future.wait(None).unwrap();
```

The `None` parameter is an optional timeout.
> **Note**: We can only do this because we called `.then_signal_fence_and_flush()` earlier. If we
> didn't do that, the .wait() method wouldn't exist.

Only after this is done can we safely call `destination.read()` and check that our copy succeeded.

```rust
let src_content = source.read().unwrap();
let destination_content = destination.read().unwrap();
assert_eq!(&*src_content, &*destination_content);
```

Next: [Introduction to compute operations](/guide/compute-intro)
