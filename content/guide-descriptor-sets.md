# Descriptor sets

In the GLSL code of the previous section, the buffer accessed by the shader was declared like
this:

```glsl
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} data;
```

In Vulkan the buffers that a compute pipeline need to access must be bound to what are called
*descriptor*s. The code above declares such a descriptor.

> **Note**: A descriptor can contain a buffer, but also other types that we haven't covered yet:
> a buffer view, an image, a sampled image, etc.. Each descriptor can also be an array.

Descriptors are grouped by *descriptor set*s. The `layout(set = 0, binding = 0)` attribute in the
GLSL code indicates that this descriptor is the descriptor 0 in the set 0. Descriptors indices and
sets indices are 0-based.

What we declared in the GLSL code is actually not a descriptor set, but only a slot for a
descriptor set. Before we can invoke the compute pipeline, we first need to bind an actual
descriptor set to that slot.

<center>![](/guide-descriptor-sets-1.svg)</center>

## Creating a descriptor set

Creating a descriptor set can be done with the `simple_descriptor_set!` macro:

```rust
let set = Arc::new(simple_descriptor_set!(pipeline.clone(), 0, {
    data: data_buffer.clone()
}));
```

The first parameter of the macro is the pipeline for which we create this set, and the second
parameter is the index of the set in the pipeline. Since `pipeline` is an `Arc`, cloning it just
clones the `Arc` and is not an expensive operation. For once it is not the *device* that we pass
as first parameter, because the device is determined from `pipeline`.

Note that you are allowed to use a set for a different pipeline than the one it was created with,
but only if there is no conflict. However you can't create a descriptor set out of thin air, as
Vulkan doesn't allow it.

The third parameter is the content of the set, which here is just the `data` variable. Just like
for `pipeline`, cloning `data_buffer` only clones an `Arc` and isn't expensive.

> **Note**: `data_buffer` was created in [the introduction](/guide/compute-intro).

> **Note**: The `simple_descriptor_set!` macro will be replaced with something else in the future.

Now that we have a compute pipeline and a descriptor set to bind to it, we can start our operation.
This is covered in [the next section](/guide/dispatch).
