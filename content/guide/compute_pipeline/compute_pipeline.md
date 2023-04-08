# Compute pipelines

In order to ask the GPU to perform an operation, we have to write some kind of code for it, like we
would for a regular program. A program that runs on the GPU is called a ***shader***.

This is done in two steps:

- First we write the source code of the program in a programming language called *GLSL*. Vulkano
  will compile the GLSL code at compile-time into an intermediate representation called *SPIR-V*.
- At runtime, we pass this *SPIR-V* to the Vulkan implementation (GPU driver), which in turn 
  converts it into its own implementation-specific format.

<div style="text-align: center;"><object data="/guide-compute-pipeline-1.svg"></object></div>

> **Note**: In the very far future it may be possible to write shaders in Rust, or in a
> domain specific language that resembles Rust.

## The GLSL code

But first, we need to write the source code of the operation. The GLSL language looks a lot like
the C programming language, but has some differences.

This guide is not going to cover teaching you GLSL, as it is an entire programming language. As 
with many programming languages, the easiest way to learn GLSL is by looking at examples.

Let's take a look at some GLSL that takes each element of a buffer and multiplies it by 12:

```glsl
#version 460

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}
```

Let's break it down a bit.

```glsl
#version 460
```

The first line indicates which version of GLSL to use. Since GLSL was already the shading language
of the OpenGL API (Vulkan's predecessor), we are in fact already at the version 4.60 of the
language. You should always include this line at the start of every shader.

> **Note**: You can use an older version for compatibility with older GPUs and Vulkan 
> implementations.

```glsl
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
```

We want to invoke the compute shader 65536 times in total, once for each element in the buffer.
But in practice we are going to ask the GPU to spawn 1024 ***work groups***, where each work group
has a ***local size*** of 64. This line of code declares what the *local size* is. Each element of
the local size corresponds to one invocation of the shader, which gives us 1024 * 64 = 65536
invocations.

You should always try to aim for a local size of at least 32 to 64. While we could ask to spawn
65536 work groups with a local size of 1, in practice this has been benchmarked to be slower than
using a larger local size.

For convenience, the number of work groups and the local size are three-dimensional. You can use
the `y` and `z` coordinates if you access a two-dimensional or a three-dimensional data structure.
The shader will be called once for each possible combination of values for `x`, `y` and `z`.

```glsl
layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;
```

This declares a *descriptor*, which we are going to cover in more details [in the next
section](/guide/descriptor-sets). In particular, we declare a buffer whose name is `buf` and that
we are going to access in our code.

The content of the buffer is an unsized array of `uint`s. A `uint` is always similar to a `u32`
in Rust. In other words the line `uint data[];` is equivalent in Rust to a variable named `data`
of type `[u32]`.

```glsl
void main() {
```

Every GLSL code has an entry point named `main`, similar to C or Rust. This is the function that
is going to be invoked 65536 times.

```glsl
uint idx = gl_GlobalInvocationID.x;
```

As explained above we are going to spawn 1024 work groups, each having a local size of 64. Compute
shaders in GLSL have access to several read-only static variables that let us know the index of
the invocation we are currently in. The `gl_GlobalInvocationID.x` value here will contain a value
that ranges between 0 and 65535. We are going to use it to determine which element of the array
to modify.

```glsl
buf.data[idx] *= 12;
```

The content of the buffer is accessed with `buf.data`. We multiply the value at the given index
with 12.

> **Note**: You can easily trigger a data race by calling, for example, `buf.data[0] *= 12;`, as all
> of the shader invocations will access the buffer simultaneously. This is a safety problem that
> vulkano doesn't detect or handle yet. Doing so will lead to an undefined result, but not in an
> undefined behavior.

## Embedding the GLSL code in the Rust code

Now that we've written the shader in GLSL, we're going to be compiling the shaders *at
application compile-time*. We'll accomplish this using `vulkano-shaders`
which is a procedural macro that manages the compile-time compilation of GLSL into SPIR-V
and generation of associated rust code.

To use `vulkano-shaders`, we first have to add a dependency:

```toml
# Notice that it uses the same version as vulkano
vulkano-shaders = "0.33.0"
```
> **Note**: `vulkano-shaders` uses the crate `shaderc-sys` for the actual GLSL compilation step. 
> When you build your project, an attempt will be made to automatically install shaderc if you 
> don't already have it. shaderc also comes in [the Vulkan 
> SDK](https://www.vulkan.org/tools#download-these-essential-development-tools)). See [shaderc-sys 
> crate](https://lib.rs/crates/shaderc-sys) for installation instructions should the automatic 
> system fail. 

Here is the syntax:

```rust
mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src: r"
            #version 460

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

            layout(set = 0, binding = 0) buffer Data {
                uint data[];
            } buf;

            void main() {
                uint idx = gl_GlobalInvocationID.x;
                buf.data[idx] *= 12;
            }
        ",
    }
}
```

As you can see, we specify some "fields" in the `vulkano_shaders::shader!` macro to specify our 
shader. The macro will then compile the GLSL code (outputting compilation errors if any) and 
generate several structs and methods, including one named `load`. This is the method that we have 
to use next:

```rust
let shader = cs::load(device.clone())
    .expect("failed to create shader module");
```

This feeds the shader to the Vulkan implementation. The last step to perform at runtime is to
create a ***compute pipeline*** object from that shader. This is the object that actually describes
the compute operation that we are going to perform. We won't cover the last three parameters, but
you can search about them
[here](https://docs.rs/vulkano/0.33.0/vulkano/pipeline/compute/struct.ComputePipeline.html).

```rust
use vulkano::pipeline::ComputePipeline;

let compute_pipeline = ComputePipeline::new(
    device.clone(),
    shader.entry_point("main").unwrap(),
    &(),
    None,
    |_| {},
)
.expect("failed to create compute pipeline");
```

Before invoking that compute pipeline, we need to bind a buffer to it. This is covered by [the
next section](/guide/descriptor-sets).
