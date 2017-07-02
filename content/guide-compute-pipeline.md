# Compute pipelines

In order to ask the GPU to perform an operation, we have to program it.

This is done in two steps:

- First we write the source code of the program in a programming language called *GLSL*. Vulkano
  will compile the GLSL code at compile-time into an intermediate representation called *SPIR-V*.
- At runtime we pass this *SPIR-V* to the Vulkan implementation.

![](/guide-compute-pipeline-1.svg)

## The GLSL code

This is the GLSL code:

```glsl
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} data;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    data.data[idx] *= 12;
}
```

You have to use a hack:

```rust
mod cs {
    #[derive(VulkanoShader)]
    #[ty = "compute"]
    #[src = "
#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} data;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    data.data[idx] *= 12;
}"
    ]
    struct Dummy;
}
```

> **Note**: This is going to change in the future.

## Creating a compute pipeline


```rust
let shader = cs::Shader::load(&device)
    .expect("failed to create shader module");
ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
    .expect("failed to create compute pipeline")
```
