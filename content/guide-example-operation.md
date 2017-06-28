# Running an operation

Now that you are familiar with devices, queues and buffers, we are finally going to ask the GPU
to do something.

What we are going to ask is very simple: we will ask the GPU to copy data from a buffer to another.

## Creating the buffers

To do so, we are going to create two `CpuAccessibleBuffer`s: the source and the destination. This
was already covered in the previous section.

    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), Some(queue.family()), [
        Vertex { position: [-0.5, -0.25] },
        Vertex { position: [0.0, 0.5] },
        Vertex { position: [0.25, -0.1] }
    ].iter().cloned()).expect("failed to create buffer");

As you can see, the source buffer contains sixty-four values ranging from 0 to 63. The destination
buffer contains sixty-four 0s.

## Command buffers

In order to ask the GPU to perform any operation, we need to create what is called a
*command buffer*.

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffers(source.clone(), destination.clone()).unwrap()
        .build().unwrap();

And now we submit it:
