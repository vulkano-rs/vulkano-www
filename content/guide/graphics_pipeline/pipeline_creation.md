# Putting it all together

In [the vertex input section](/guide/vertex-input) we created a buffer named `vertex_buffer` which
contains the shape of our triangle, and wrote the source code of a *vertex shader* that positions
vertices on the image.

In [the fragment shader section](/guide/fragment-shader) we wrote the source code of a
*fragment shader* that fills pixels with a color.

Finally in [the render passes section](/guide/render-pass-framebuffer) we create a *render pass*
and a *framebuffer* that contains the target image.

It is now time to put everything together and perform the draw operation!

> **Note**: You can find the [full source code of this chapter
> here](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/src/bin/graphics_pipeline.rs).

## Creating a graphics pipeline

Just like we had to create a compute pipeline in order to perform a compute operation, we have to
create a graphics pipeline before we perform a draw operation.

This is done by first creating the shaders, just like for a compute pipeline:

```rust
mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
            #version 460

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
    }
}

let vs = vs::load(device.clone()).expect("failed to create shader module");
let fs = fs::load(device.clone()).expect("failed to create shader module");
```

Then we can create the graphics pipeline by using a builder.

```rust
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;

// More on this latter
let viewport = Viewport {
    origin: [0.0, 0.0],
    dimensions: [1024.0, 1024.0],
    depth_range: 0.0..1.0,
};

let pipeline = GraphicsPipeline::start()
    // Describes the layout of the vertex input and how should it behave
    .vertex_input_state(MyVertex::per_vertex())
    // A Vulkan shader can in theory contain multiple entry points, so we have to specify
    // which one.
    .vertex_shader(vs.entry_point("main").unwrap(), ())
    // Indicate the type of the primitives (the default is a list of triangles)
    .input_assembly_state(InputAssemblyState::new())
    // Set the fixed viewport
    .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
    // Same as the vertex input, but this for the fragment input
    .fragment_shader(fs.entry_point("main").unwrap(), ())
    // This graphics pipeline object concerns the first pass of the render pass.
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    // Now that everything is specified, we call `build`.
    .build(device.clone())
    .unwrap();
```

When we draw, we have the possibility to draw only to a specific rectangle of the screen called a
***viewport***. The borders of the viewport will map to the `-1.0` and `1.0` logical coordinates 
that we covered in [the vertex input section of the guide](/guide/vertex-input). Any part of the 
shape that ends up outside of this rectangle will be discarded.

The state `ViewportState::viewport_fixed_scissor_irrelevant()` configures the builder so that we 
use one specific viewport, and that the state of this viewport is *fixed*. This makes it not 
possible to change the viewport for each draw command, but adds more performance. Because we are 
drawing only one image and not changing the viewport between draws, this is the optimal approach. 
If you wanted to draw to another image of a different size, you would have to create a new pipeline 
object. Another approach would be to use a dynamic viewport, where you would pass your viewport in 
the command buffer instead.

> **Note**: If you configure multiple viewports, you can use geometry shaders to choose which
> viewport the shape is going to be drawn to. This topic isn't covered here.

## Drawing

Now that we have all the ingredients, it is time to bind everything and insert a draw call inside 
of our render pass.

To draw the triangle, we need to pass the pipeline, the vertex_buffer and the actual draw command:

```rust
let mut builder = AutoCommandBufferBuilder::primary(
    device.clone(),
    queue.queue_family_index(),
    CommandBufferUsage::OneTimeSubmit,
)
.unwrap();

builder
    .begin_render_pass(
        RenderPassBeginInfo {
            clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
            ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
        },
        SubpassContents::Inline,
    )
    .unwrap()

    // new stuff
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
constants are in the case of drawing on multiple viewports or drawing multiple objects with 
instancing (we won't cover that here).

> **Note**: If you wanted to draw multiple objects, the most straight-forward method is to call
> `draw()` multiple time in a row.

Once we have finished drawing, let's do the same thing as [in the mandelbrot
example](/guide/mandelbrot) and write the image to a PNG file.

To do that, as before, let's first create the buffer:

```rust
// crop

let buf = Buffer::from_iter(
    &memory_allocator,
    BufferCreateInfo {
        usage: BuferUsage::TRANSFER_DST,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Download,
        ..Default::default()
    },
    (0..1024 * 1024 * 4).map(|_| 0u8),
)
.expect("failed to create buffer");

// crop
```

And then write the rest of the operations:

```rust
    .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, buf.clone()))
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

println!("Everything succeeded!");
```

And here is what you should get:

<center>
<img src="/guide-graphics-pipeline-creation-1.png" />
</center>

Next: [Windowing](/guide/windowing/introduction)
