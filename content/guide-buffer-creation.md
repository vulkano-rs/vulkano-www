# Creating a buffer

When using Vulkan, you will very often need for the GPU to read or write data in memory. In fact
there isnt's much point in using the GPU otherwise, as you will need to ask it to write the results
of its calculations somewhere.

In order for the GPU to be able to access some data (either for reading, writing or both), we
first need to create a ***buffer*** and put the data in it.

## Several kinds of buffers

Vulkano doesn't provide a `Buffer` object which you could create with `Buffer::new`. Instead it
provides several different structs that all represent buffers, each of these structs being optimal
for a certain kind of usage. For example if you want to continuously upload data you should use a
`CpuBufferPool`, while on the other hand if you have some data that you are never going to modify
you should use an `ImmutableBuffer`.

The most simple kind of buffer that exists is the `CpuAccessibleBuffer`, which can be created
like this:

*To be finished*
