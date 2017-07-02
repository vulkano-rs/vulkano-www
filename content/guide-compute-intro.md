# Introduction to compute operations

Before going further, let's understand the difference between a CPU and a GPU.

Both the CPU and the GPU execute instructions one by one. The instructions available for regular
programs that run on the CPU include for example modifying a register, modifying a value in
memory, or performing some mathematical operation.

The instructions that a GPU can execute are the same, except that they can operate on a lot of
data at once. You can for example instruct the GPU to multiply at once thirty-two values by a
constant, in approximately the same time that a CPU would take to multiply one value by that
constant.

This is what makes GPUs very good at parallel computations which require executing the same
sequence of operation on multiple values. While a CPU would perform this sequence on each value one
by one, a GPU can perform it on multiple values at once.

> **Note**: See also [SIMD](https://en.wikipedia.org/wiki/SIMD).

> **Note**: In [a previous section](/guide/device-creation) we talked about *queues*. These queues
> are usually foremost *software* queues, and not actual hardware constructs. One exception is
> transfer-only queues (that can't run any operation except data transfers), which are going to
> cover later.

## Usability

Vulkan (or any other API) doesn't let you directly control the threading aspect of the GPU.
In order to perform an operation of multiple values at once, you will only need to indicate the
list of operations to perform on **one** value. The Vulkan implementation will automatically make
the necessary adjustements to make your operation run on multiple values at once.

This makes using a GPU much easier than if you had to manually control everywhere. However you
still need to be aware that your program will run multiple times in parallel.

## Example in this guide

For the purpose of this guide, we are going to do something very simple: we are going to multiply
65536 values by the constant 12. Even though this doesn't serve any purpose, it is a good starting
point example. Most real-world usages of the GPU involve complex mathematical algorithms, and thus
are not really appropriate for a tutorial.

As explained above, all we have to do is write the operation that is performed on *one* value. 
Therefore our operation here is simply (in pseudo-code):

```glsl
value = value * 12;
```

The Vulkan implementation will automatically handle all the details that make it possible to run
this in parallel for each of the 65536 values.

Another preliminary thing we want to do is create the buffer that will contain the values.
*To be finished*.

[The next section of the guide](/guide/compute-pipeline) will indicate how to actually code this
operation.
