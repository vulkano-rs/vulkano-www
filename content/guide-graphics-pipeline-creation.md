# Putting it all together

In [the vertex input section](/guide/vertex-input) we created a buffer named `vertex_buffer` which
contains the shape of our triangle, and wrote the source code of a *vertex shader* that positions
vertices on the image.

In [the fragment shader section](/guide/fragment-shader) we wrote the source code of a
*fragment shader* that fills pixels with a color.

Finally in [the render passes section](/guide/render-pass-framebuffer) we create a *render pass*
and a *framebuffer* that contains the target image.

It is now time to put everything together and perform the draw operation!

> **Note**: You can find the [full source code of this section
> here](https://github.com/vulkano-rs/vulkano-www/blob/master/examples/guide-triangle.rs).

## Creating a graphics pipeline

Just like we had to create a compute pipeline in order to perform a compute operation, we have to
create a graphics pipeline before we perform a draw operation.

This is done by first creating the shaders, just like for a compute pipeline:

```rust
mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"
    }
}

mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}"
    }
}

let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");
```

Then we can create the graphics pipeline by using a builder.

```rust
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;

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

Now that we have all the ingredients, it is time to bind everything and insert a draw call inside of
our render pass.

To draw the triangle, we need to pass the viewport, the pipeline, the vertex_buffer and the actual
draw command:

```rust
use vulkano::pipeline::viewport::Viewport;

let mut builder = AutoCommandBufferBuilder::primary(
    device.clone(),
    queue.family(),
    CommandBufferUsage::OneTimeSubmit,
)
.unwrap();

let viewport = Viewport {
    origin: [0.0, 0.0],
    dimensions: [1024.0, 1024.0],
    depth_range: 0.0..1.0,
};

builder
    .begin_render_pass(
        framebuffer.clone(),
        SubpassContents::Inline,
        vec![[0.0, 0.0, 1.0, 1.0].into()],
    )
    .unwrap()

    // new stuff
    .set_viewport(0, [viewport])
    .bind_pipeline_graphics(pipeline.clone())
    .bind_vertex_buffers(0, vertex_buffer.clone())
    .draw(
        3, 1, 0, 0, // 3 is the number of vertices, 1 is the number of instances
    )
    
    .unwrap()
    .end_render_pass()
    .unwrap()

// (continued below)
```

The first parameter of the `.draw()` method is the number of vertices of our shape. All the other
constants are in the case of drawing on multiple viewports or drawing multiple objects with instancing
(we won't cover that here).
> **Note**: If you wanted to draw multiple objects, the most straight-forward method is to call
> `draw()` multiple time in a row.

Once we have finished drawing, let's do the same thing as [in the mandelbrot
example](/guide/mandelbrot) and write the image to a PNG file.

```rust
    .copy_image_to_buffer(image, buf.clone())
    .unwrap();

let command_buffer = builder.build().unwrap();

let future = sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush()
    .unwrap();
future.wait(None).unwrap();

let buffer_content = buf.read().unwrap();
let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
image.save("image.png").unwrap();
```

And here is what you should get:

<center>
<img src="/guide-graphics-pipeline-creation-1.png" />
</center>

Next: [Windowing](/guide/window)
