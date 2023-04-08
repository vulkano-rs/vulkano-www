# Render passes

In order to fully optimize and parallelize command execution, we can't just ask the GPU
to draw a shape whenever we want. Instead we first have to enter a special "rendering mode" by
*entering* what is called a ***render pass***. It is only once we have entered a render pass that
you can draw.

## What is a render pass?

The term "render pass" describes two things:

- It designates the "rendering mode" we have to enter before we can add drawing commands to
  a command buffer.

- It also designates a kind of object that describes this rendering mode.

Entering a render pass (as in "the rendering mode") requires passing a render pass object.

## Creating a render pass

For the moment, the only thing we want to do is draw some color to a single image. This is the most
simple case possible, and we only need to provide two things to a render pass: the format of
the image, and the fact that we don't use multisampling (which is an anti-aliasing technique).

More complex games can use render passes in very complex ways, with multiple subpasses and
multiple attachments, and with various micro-optimizations. Vulkano's API is suitable for both the
simple cases and the complex usages, which is why it may look complex at first.

```rust
let render_pass = vulkano::single_pass_renderpass!(
    device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            format: Format::R8G8B8A8_UNORM,
            samples: 1,
        },
    },
    pass: {
        color: [color],
        depth_stencil: {},
    },
)
.unwrap();
```

A render pass is made of **attachments** and **passes**. Here we declare one attachment whose name
is `color` (the name is arbitrary), and one pass that will use `color` as its single output.

The `load: Clear` line indicates that we want the GPU to *clear* the image when entering the render
pass (i.e. fill it with a single color), while `store: Store` indicates that we want the GPU to
actually store the output of our draw commands to the image.

> **Note**: It is possible to create temporary images whose content is only relevant inside of a
> render pass, in which case it is optimal to use `store: DontCare` instead of `store: Store`.

## Entering the render pass

A render pass only describes the format and the way we load and store the image we are going to
draw upon. It is enough to initialize all the objects we need.

But before we can draw, we also need to indicate the actual list of attachments. This is done
by creating a *framebuffer*.

Creating a framebuffer is typically done as part of the rendering process. It is not a
bad idea to keep the framebuffer objects alive between frames, but it won't kill your
performance to create and destroy a few framebuffer objects during some frames.

```rust
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};

let view = ImageView::new_default(image.clone()).unwrap();
let framebuffer = Framebuffer::new(
    render_pass.clone(),
    FramebufferCreateInfo {
        attachments: vec![view],
        ..Default::default()
    },
)
.unwrap();
```

We are now ready the enter drawing mode!

This is done by calling the `begin_render_pass` function on the command buffer builder.
This function takes as parameter the framebuffer, a enum, and a `Vec` that contains the colors
to fill the attachments with. Since we have only one single attachment, this `Vec` contains only
one element.

Clearing our attachment has exactly the same effect as the `clear_color_image` function we covered
previously, except that this time it is done by the rendering engine.

The enum passed as second parameter describes whether we are going to directly invoke draw
commands or use secondary command buffers instead. Secondary command buffers are a more advanced
topic. Be we are using only direct commands, we will leave it as `::Inline`

As a demonstration, let's just enter a render pass and leave it immediately after:

```rust
use vulkano::command_buffer::{RenderPassBeginInfo, SubpassContents};

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
    .end_render_pass()
    .unwrap();
```

The [next section](/guide/graphics-pipeline-creation) will introduce the `draw` command, which will
be inserted between `begin_render_pass` and `end_render_pass`.
