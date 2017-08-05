# Putting it all together

In [the vertex input section](/guide/vertex-input) we created a buffer named `vertex_buffer` which
contains the shape of our triangle, and wrote the source code of a *vertex shader* that positions
vertices on the image.

In [the fragment shader section](/guide/fragment-shader) we wrote the source code of a
*fragment shader* that fills pixels with a color.

Finally in [the render passes section](/guide/render-pass-framebuffer) we create a *render pass*
and a *framebuffer* that contains the target image.

It is now time to put everything together and perform the draw operation!

## Creating a graphics pipeline

Just like we had to create a compute pipeline in order to perform a compute operation, we have to
create a graphics pipeline before we perform a draw operation.

This is done by first creating the shaders, just like for a compute pipeline:

```rust
mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"]
    struct Dummy;
}

mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"]
    struct Dummy;
}

let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");
```

Then we can create the graphics pipeline by using a builder.

```rust
let pipeline = Arc::new(GraphicsPipeline::start()
    // Defines what kind of vertex input is expected.
    .vertex_input_single_buffer::<Vertex>()
    // The vertex shader.
    .vertex_shader(vs.main_entry_point(), ())
    // Defines the viewport (explanations below).
    .viewports_dynamic_scissors_irrelevant(1)
    // The fragment shader.
    .fragment_shader(fs.main_entry_point(), ())
    // The render pass.
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    // Now that everything is specified, we call `build`.
    .build(device.clone())
    .unwrap());
```

# Drawing

```rust
let cb = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
    .begin_render_pass(framebuffers.as_ref().unwrap()[image_num].clone(), false,
                        vec![[0.0, 0.0, 1.0, 1.0].into()])
    .unwrap()

    .draw(pipeline.clone(),
            DynamicState {
                line_width: None,
                // TODO: Find a way to do this without having to dynamically allocate a Vec every frame.
                viewports: Some(vec![Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                    depth_range: 0.0 .. 1.0,
                }]),
                scissors: None,
            },
            vertex_buffer.clone(), (), ())
    .unwrap()

    .end_render_pass()
    .unwrap()

// (continued below)
```

```rust
    .copy_image_to_buffer(image.clone(), buf.clone())
    .unwrap()

    .build()
    .unwrap();
```
