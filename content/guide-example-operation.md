# Running an operation

Now that we are familiar with devices, queues and buffers, we are finally going to ask the GPU
to do something.

What we are going to ask is very simple: we will ask the GPU to copy data from a buffer to another.

## Creating the buffers

To do so, we are going to create two `CpuAccessibleBuffer`s: the source and the destination. This
was already covered in a previous section.

```rust
let source_content = 0 .. 64;
let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), Some(queue.family()),
                                            source_content).expect("failed to create buffer");

let dest_content = 0 .. 64.map(|_| 0);
let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), Some(queue.family()),
                                          dest_content).expect("failed to create buffer");
```

As you can see, the source buffer contains sixty-four values ranging from 0 to 63. The destination
buffer contains sixty-four 0s.

## Command buffers

In order to ask the GPU to perform an operation, we need to create a type of object that we
haven't talked about yet: ***command buffer***.

With Vulkan and vulkano, you can't just execute commands one by one as it would be too inefficient
(submitting a command to the GPU can take up to several hundred microseconds). Instead we
need to build a *command buffer* that contains the list of commands that we want to execute.

> **Note**: OpenGL allows you to execute commands one by one, but in reality implementations buffer
> commands internally into command buffers. In other words, OpenGL automatically does what Vulkan
> requires us to do manually. This is a good thing though, as OpenGL's automatic buffering often
> causes more harm than good.

Here is how you create a command buffer:

```rust
let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
    .copy_buffers(source.clone(), destination.clone()).unwrap()
    .build().unwrap();
```

## Submission and synchronization

And now we submit it:



After submitting the command buffer to the GPU, we might be tempted to try read the content of the
`destination` buffer as demonstrated in [the previous section](/guide/buffer-creation). However
calling `destination.read()` now would return an error, because the buffer is currently being
written by the GPU! Submitting an operation to the GPU doesn't wait for the operation to be
complete. Instead it just sends some kind of signal to the GPU to instruct it that it must start
processing the command buffer, and the actual processing is performed asynchronously.

In order to read the content of `destination` and make sure that our copy succeeded, we need to
wait until the operation is complete. This is done like this:



Only after can we call `destination.read()`:

```rust
let src_content = source.read().unwrap();
let dest_content = destination.read().unwrap();
assert_eq!(&*src_content, &*dest_content);
```

*To be finished*
