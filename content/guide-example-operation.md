# Running an operation

Now that we are familiar with devices, queues and buffers, we are going to see how to ask the GPU
to actually do something.

What we are going to ask in this example is very simple: we will ask it to copy data from one
buffer to another.

## Creating the buffers

The first step is to create two `CpuAccessibleBuffer`s: the source and the destination. This
was covered in [the previous section](/guide/buffer-creation).

```rust
let source_content = 0 .. 64;
let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), Some(queue.family()),
                                            source_content).expect("failed to create buffer");

let dest_content = (0 .. 64).map(|_| 0);
let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), Some(queue.family()),
                                          dest_content).expect("failed to create buffer");
```

The iterators might look a bit tricky. The `source_content` iterator produces 64 values ranging
from 0 to 63. The `dest_content` iterator produces 64 values that are all equal to 0.
In other words, once created the source buffer contains sixty-four values ranging from 0 to 63
while the destination buffer contains sixty-four 0s.

## Command buffers

In order to ask the GPU to perform an operation, we need to create a type of object that we
haven't covered yet: ***command buffer***.

With Vulkan and vulkano you can't just execute commands one by one, as it would be too inefficient.
Instead we need to build a *command buffer* that contains a list of commands that we want to
execute.

> **Note**: Submitting a command to the GPU can take up to several hundred microseconds, which is
> why we submit as many things as we can at once.

> **Note**: OpenGL (Vulkan's predecessor) allows you to execute commands one by one, but in reality
> implementations buffer commands internally into command buffers. In other words, OpenGL
> automatically does what Vulkan requires us to do manually. In practice OpenGL's automatic
> buffering often causes more harm than good in performance-critical applications.

Here is how you create a command buffer:

```rust
use vulkano::command_buffer::AutoCommandBufferBuilder;

let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
    .copy_buffer(source.clone(), dest.clone()).unwrap()
    .build().unwrap();
```

As you can see, it is very straight-forward. We create a *builder*, add a copy command to it with
`copy_buffers`, then turn that builder into an actual command buffer with `build`. Like we saw in
[the buffers creation section](/guide/buffer-creation), we call `clone()` multiple times but in
reality we only clone `Arc`s.

One thing to notice is that the `AutoCommandBufferBuilder::new()` method takes as
parameter a queue family. This must be the queue family that the command buffer is going to run on.
In this example we don't have much choice anyway (as we only use one queue and thus one queue
family), but when you design a real program you have to be aware of this specificity.

## Submission and synchronization

And now we submit the command buffer so that it gets executed:

```rust
use vulkano::command_buffer::CommandBuffer;
let finished = command_buffer.execute(queue.clone()).unwrap();
```

The `execute` function returns an object that represents the execution of the command buffer.

After submitting the command buffer, we might be tempted to try read the content of the
`destination` buffer as demonstrated in [the previous section](/guide/buffer-creation). However
calling `destination.read()` now would return an error, because the buffer is maybe currently being
written by the GPU.

Submitting an operation doesn't wait for the operation to be complete. Instead it just sends some
kind of signal to the GPU to instruct it that it must start processing the command buffer, and the
actual processing is performed asynchronously.

In order to read the content of `destination` and make sure that our copy succeeded, we need to
wait until the operation is complete. This is done by making use of the `finished` object that
was returned by `execute`:

```rust
use vulkano::sync::GpuFuture;

finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();
```

This may look a bit complicated, but we will cover what a *fence* is in a later section of the
guide and what signalling it means. The `wait()` function blocks the current thread until the GPU
has finished execution.

Only after this was done we can call `destination.read()` and check that our copy succeeded.

```rust
let src_content = source.read().unwrap();
let dest_content = dest.read().unwrap();
assert_eq!(&*src_content, &*dest_content);
```
