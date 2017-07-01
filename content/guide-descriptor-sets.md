# Descriptor sets

In the GLSL code of the previous section, the .

    layout(set = 0, binding = 0) buffer Data {
        uint data[];
    } data;

The buffers and images that a compute pipeline can access are bound to what is called
*descriptor*s. The code above declares such a descriptor.

Descriptors are grouped by *descriptor set*s. The `layout(set = 0, binding = 0)` indicates
that this descriptor is the descriptor 0 in the set 0. Descriptors and sets are 0-indexed.

What we declared in the GLSL code is actually just a slot for a descriptor set. In order to invoke
our compute pipeline, we first need to bind an actual descriptor set.

![](/guide-descriptor-sets-1.svg)

## Creating a descriptor set

Creating a descriptor set can be done with the `simple_descriptor_set!` macro:

    let set = Arc::new(simple_descriptor_set!(pipeline.clone(), 0, {
        data: data_buffer.clone()
    }));

> **Note**: `data_buffer` was created in [the introduction](/guide/compute-intro).

> **Note**: The `simple_descriptor_set!` will be replaced with something else in the future.

## Advanced: about descriptor sets

*To be written*.
