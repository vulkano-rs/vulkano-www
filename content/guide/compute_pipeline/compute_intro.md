# Introduction to compute operations

Before we go further, we need to understand the difference between a CPU and a GPU. As a reminder,
the CPU is what executes your Rust program, while the GPU is what we are trying to interface with.

Both the CPU and the GPU execute instructions one by one. The instructions available for regular
programs that run on the CPU include, for example, modifying a value in memory, or performing some
mathematical operation.

The instructions that a GPU can execute are often limited, but they can operate on a lot of
data at once. You can, for example, instruct the GPU to multiply thirty-two values by a constant,
in approximately the same time that a CPU would take to multiply a single value by that constant
(ignoring the overhead of transferring data between the two devices).

This is what makes GPUs very good at parallel computations which require executing the same
sequence of operation on multiple values. While a CPU would perform this sequence on each value one
by one, a GPU can perform it on multiple values at once.

> **Note**: See also [SIMD](https://en.wikipedia.org/wiki/SIMD).

> **Note**: In [a previous section](/guide/device-creation) we talked about *queues*. These queues
> are usually foremost *software* queues, and not actual hardware constructs.

> **Note**: You can find the [full source code of this chapter
> here](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/src/bin/compute_pipeline.rs).

## Usability

Vulkan (or any other API) doesn't let you directly control the threading aspect of the GPU.
In order to perform an operation with multiple values at once, you will only need to indicate the
list of operations to perform on **one** value. The Vulkan implementation will automatically make
the necessary adjustments to make your operation run on multiple values at once.

This makes using a GPU much easier than if you had to manually control everything. However, you
still need to be aware that your program will run multiple times in parallel, because it has
consequences on what you can do without causing data races.

## Example in this guide

For the purpose of this guide, we are going to do something very simple: we are going to multiply
65536 values by the constant 12. Even though this doesn't serve any purpose, it is a good starting
point example. Most real-world usages of the GPU involve complex mathematical algorithms, and thus
are not really appropriate for a tutorial.

As explained above, you don't need to use any `for` loop or anything similar of that sort. All we
have to do is write the operation that is performed on *one* value, and ask the GPU to execute
it 65536 times. Our operation here is therefore simply (in pseudo-code):

```glsl
// `index` will range from 0 to 65536
buffer_content[index] *= 12;
```

While it may look like this code multiplies a single value by 12, in reality the Vulkan
implementation will automatically handle all the details that make it possible to run this in
parallel multiple times in the most optimized way.

As a preliminary action we are going to create the buffer that will contain the values. This is
similar to what we already did twice:

```rust
let data_iter = 0..65536u32;
let data_buffer = Buffer::from_iter(
    &memory_allocator,
    BufferCreateInfo {
        usage: BufferUsage::STORAGE_BUFFER,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Upload,
        ..Default::default()
    },
    data_iter,
)
.expect("failed to create buffer");
```

The `data_buffer` buffer now contains the data before the transformation, and we are going to
perform the calculation on each element.
Although notice that we're using `STORAGE_BUFFER` usage this time, since the buffer will be used
in the compute shader.

[The next section of the guide](/guide/compute-pipeline) will indicate how to actually code this
operation.
