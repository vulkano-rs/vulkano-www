# Descriptor sets

In the GLSL code of the previous section, the buffer accessed by the shader was declared like
this:

```glsl
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;
```

In Vulkan, the buffers that a compute pipeline needs to access must be bound to what are called
*descriptor*s. The code above declares such a descriptor.

> **Note**: A descriptor can contain a buffer, but also other types that we haven't covered yet:
> a buffer view, an image, a sampled image, etc. One or more descriptors of the same type can form
> an array.

A descriptor or array of descriptors is assigned to a *binding*, and bindings are grouped into
*descriptor set*s. The `layout(set = 0, binding = 0)` attribute in the
GLSL code indicates that this descriptor is assigned to binding 0 in the set 0. Binding indices
and set indices are 0-based.

What we declared in the GLSL code is actually not a descriptor set, but only a slot for a
descriptor set. Before we can invoke the compute pipeline, we first need to bind an actual
descriptor set to that slot.

<div style="text-align: center;"><object data="/guide-descriptor-sets-1.svg"></object></div>

## Creating a descriptor set

Just like for buffers and command buffers, we also need an allocator for descriptor sets.

For our application, we are going to use a `PersistentDescriptorSet`. When creating this descriptor
set, we attach to it the result buffer wrapped in a `WriteDescriptorSet`. This object will describe
how will the buffer be written:

```rust
use vulkano::pipeline::Pipeline;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;

let descriptor_set_allocator = StandardDescriptorSetAllocator::new(device.clone());
let pipeline_layout = compute_pipeline.layout();
let descriptor_set_layouts = pipeline_layout.set_layouts();

let descriptor_set_layout_index = 0;
let descriptor_set_layout = descriptor_set_layouts
    .get(descriptor_set_layout_index)
    .unwrap();
let descriptor_set = PersistentDescriptorSet::new(
    &descriptor_set_allocator,
    descriptor_set_layout.clone(),
    [WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
)
.unwrap();
```

In order to create a descriptor set, you'll need to know the layout that it is targeting. We do 
this by using the "Pipeline" trait and calling `.layout()` on our pipeline to obtain the pipeline's 
layout. Next we'll fetch the layout specific to the pass that we want to target by using 
`.set_layouts().get(0)` where zero indicates the first index of the pass that we are targeting.

Once you have created a descriptor set, you may also use it with other pipelines, as long as the
bindings' types match those the pipelines' shaders expect. But Vulkan requires that you provide a
pipeline whenever you create a descriptor set; you cannot create one independently of any
particular pipeline.

We then bind each descriptor one by one in order, which here is just the `buf` variable. Just like
for `compute_pipeline`, cloning `data_buffer` only clones an `Arc` and isn't expensive.

> **Note**: `data_buffer` was created in [the introduction](/guide/compute-intro).

Now that we have a compute pipeline and a descriptor set to bind to it, we can start our operation.
This is covered in [the next section](/guide/dispatch).
