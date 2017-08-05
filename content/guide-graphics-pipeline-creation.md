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
use vulkano::pipeline::GraphicsPipeline;
use vulkano::framebuffer::Subpass;

let pipeline = Arc::new(GraphicsPipeline::start()
    // Defines what kind of vertex input is expected.
    .vertex_input_single_buffer::<Vertex>()
    // The vertex shader.
    .vertex_shader(vs.main_entry_point(), ())
    // Defines the viewport (explanations below).
    .viewports_dynamic_scissors_irrelevant(1)
    // The fragment shader.
    .fragment_shader(fs.main_entry_point(), ())
    // This graphics pipeline object concerns the first pass of the render pass.
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    // Now that everything is specified, we call `build`.
    .build(device.clone())
    .unwrap());
```

When we draw, we have the possibility to draw only to a specific rectangle of the screen called a
***viewport***. The borders of the viewport will map to the `-1` and `1` logical coordinates that
we covered in [the vertex input section of the guide](/guide/vertex-input). Any part of the shape
that ends up outside of this rectangle will be discarded.

The call to `viewports_dynamic_scissors_irrelevant(1)` configures the builder so that we use one
viewport, and that the state of this viewport is *dynamic*. This makes it possible to change the
viewport for each draw command. If the viewport state wasn't dynamic, then we would have to create
a new pipeline object if we wanted to draw to another image of a different size.

> **Note**: If you configure multiple viewports, you can use geometry shaders to choose which
> viewport the shape is going to be drawn to. This topic isn't covered here.

# Drawing

Now that we have all the ingredients, it is time to insert a call to `draw()` inside of our render
pass.

The `draw` method takes as parameter the pipeline object, the dynamic state that contains our
viewport, the buffer that contains our shape, and the descriptor sets and push constants. The
descriptor sets and push constants are the same thing as for compute shaders.

```rust
use vulkano::command_buffer::DynamicState;
use vulkano::pipeline::viewport::Viewport;

let dynamic_state = DynamicState {
    viewports: Some(vec![Viewport {
        origin: [0.0, 0.0],
        dimensions: [1024.0, 1024.0],
        depth_range: 0.0 .. 1.0,
    }]),
    .. DynamicState::none()
};

let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
    .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 1.0, 1.0].into()])
    .unwrap()

    .draw(pipeline.clone(), dynamic_state, vertex_buffer.clone(), (), ())
    .unwrap()

    .end_render_pass()
    .unwrap()

// (continued below)
```

Once we have finished drawing, let's do the same thing as [in the mandelbrot
example](/guide/mandelbrot) and write the image to a PNG file.

```rust
    .copy_image_to_buffer(image.clone(), buf.clone())
    .unwrap()

    .build()
    .unwrap();
    
let finished = command_buffer.execute(queue.clone()).unwrap();
finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();

let buffer_content = buf.read().unwrap();
let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
image.save("triangle.png").unwrap();
```

And here is what you should get:

<center>
![](/guide-graphics-pipeline-creation-1.png)
</center>

> **Note**: You can find the [full source code of this section
> here](https://github.com/vulkano-rs/vulkano/blob/master/examples/src/bin/guide-triangle.rs).
