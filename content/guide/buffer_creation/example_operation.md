# Example operation

Now that we are familiar with devices, queues, and buffers, we are going to see how to ask the GPU
to actually do something.

What we are going to ask in this example is very simple: we will ask it to copy data from one
buffer to another.

> **Note**: You can find the [full source code of this section
> here](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/buffer_creation.rs).

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
haven't covered yet, the *command buffer*.

With Vulkan and Vulkano you can't just execute commands one by one, as it would be too inefficient.
Instead, we need to build a command buffer that contains a list of commands that we want to
execute.

You can create many command buffers and use them different times during the program. They can have
different uses and can do many different things. In this case, we are just going to create for the
operation we are trying to achieve.

Vulkan supports primary and secondary command buffers. Primary command buffers can be sent directly to the GPU
while secondary command buffers allow you to store functionality that you can reuse multiple times in primary command buffers.
We won't cover secondary command buffers here, but you can read
[more about them](https://docs.rs/vulkano/0.28.0/vulkano/command_buffer/index.html).

> **Note**: Submitting a command to the GPU can take up to several hundred microseconds, which is
> why we submit as many things as we can at once.
> OpenGL (Vulkan's predecessor) allows you to execute commands one by one, but in reality
> implementations buffer commands internally into command buffers. In other words, OpenGL
> automatically does what Vulkan requires us to do manually. In practice, OpenGL's automatic
> buffering often causes more harm than good in performance-critical applications.

We are going to submit the commands to the GPU, so let's create a primary command buffer:

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
`copy_buffer`, then turn that builder into an actual command buffer with `.build()`. Like we saw in
[the buffers creation section](/guide/buffer-creation), we call `.clone()` multiple times but we
only clone `Arc`s.

One thing to notice is that the `AutoCommandBufferBuilder::primary()` method takes as
parameter a queue family. This must be the queue family that the command buffer is going to run on.
In this example we don't have much choice anyway (as we only use one queue and thus one queue
family), but when you design a real program you have to be aware of this requirement.

## Submission and synchronization

The last step is to actually send the command buffer and execute it in the GPU.
We can do that by synchronizing with the GPU, then executing the command buffer:

```rust
use vulkano::sync;
use vulkano::sync::GpuFuture;

sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .flush()
    .unwrap();
```

No function in Vulkano immediately sends an operation to the GPU
(with the exception of some unsafe low-level functions). Instead, `sync::now()` creates a new
type of object called a *future*, that keeps alive all the resources that will be used by the GPU
and represents the execution in time of the actual operations.

The future returned by `sync::now()` is in a pending state, what makes it possible to append the execution of other command
buffers and operations. Only by calling `.flush()` that these operations are all submitted at once, and
they actually start executing on the GPU.

Using objects like this let's us build dependencies between operations and makes
it possible to make an operation start only after a previous operation is finished, while reducing the number slow communication
operations between the CPU and the GPU.

After submitting the command buffer, we might be tempted to try to read the content of the
`destination` buffer as demonstrated in [the previous section](/guide/buffer-creation).
However, because the CPU and GPU are now executing in parallel, calling `destination.read()`
now may sometimes return an error, because the buffer could still be being used by the GPU.

In order to read the content of `destination` and make sure that our copy succeeded, we need to
wait until the operation is complete. To do that, we need to program the GPU to send back a special
signal that will make us know it has finished. This kind of signal is called a *fence*, and it lets us
know whenever the GPU has reached a certain point of execution.

To do that, let's actually save the future and wait for the operations to finish:

```rust
let future = sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush() // same as signal fence, and then flush
    .unwrap();
```

Signaling a fence returns a future object called
[FenceSignalFuture](https://docs.rs/vulkano/0.28.0/vulkano/sync/struct.FenceSignalFuture.html),
that has a special method `.wait()`:

```rust
future.wait(None).unwrap();  // None is an optional timeout
```

Only after this is done can we safely call `destination.read()` and check that our copy succeeded.

```rust
let src_content = source.read().unwrap();
let destination_content = destination.read().unwrap();
assert_eq!(&*src_content, &*destination_content);
```

Next: [Introduction to compute operations](/guide/compute-intro)
