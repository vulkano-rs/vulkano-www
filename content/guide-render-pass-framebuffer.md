# Render passes

In order to fully optimize and parallelize commands execution, we can't just ask the GPU
to draw a shape whenever we want. Instead we first have to enter "rendering mode" by entering
what is called a ***render pass***, then draw, and then leave the render pass.

## What is a render pass?

The term "render pass" describes two things:

- It designates the "rendering mode" we have to enter before we can add drawing commands to
  a command buffer.

- It also designates a kind of object that describes this rendering mode.

Entering a render pass (as in "the rendering mode") requires passing a render pass object.

## Creating a render pass

For the moment, the only thing we want to do is draw some color to an image that corresponds to
our window. This is the most simple case possible, and we only need to provide two informations
to a render pass: the format of the images of our swapchain, and the fact that we don't use
multisampling (which is an advanced anti-aliasing technique).

However complex games can use render passes in very complex ways, with multiple subpasses and
multiple attachments, and with various micro-optimizations. In order to accomodate for these
complex usages, vulkano's API to create a render pass is a bit particular.

TODO: provide a simpler way in vulkano to do that?

```rust
let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
    attachments: {
        color: {
            load: Clear,
            store: Store,
            format: swapchain.format(),
            samples: 1,
        }
    },
    pass: {
        color: [color],
        depth_stencil: {}
    }
).unwrap());
```

## Entering the render pass

A render pass only describes the format and the way we load and store the image we are going to
draw upon. It is enough to initialize all the objects we need.

But before we can draw, we also need to indicate the actual list of attachments. This is done
by creating a *framebuffer*.

Creating a framebuffer is typically done as part of the rendering process. It is not a
bad idea to keep the framebuffer objects alive between frames, but it won't kill your
performances to create and destroy a few framebuffer objects during each frame.

```rust
let framebuffer = {
    let image = &images[image_num];
    let dimensions = [image.dimensions()[0], image.dimensions()[1], 1];
    Framebuffer::new(&render_pass, dimensions, render_pass::AList {
        color: image
    }).unwrap()
};
```

We are now ready the enter drawing mode!

This is done by calling the `begin_render_pass` function on the command buffer builder.
This function takes as parameter the framebuffer, TODO, and a `Vec` that
contains the colors to fill the attachments with.

Clearing our attachment has exactly the same effect as the `clear_color_image` function we covered
previously, except that this time it is done by the rendering engine.

For the sake of the example, let's just enter a render pass and leave it immediately after:

```rust
AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
    .begin_render_pass(framebuffers.as_ref().unwrap()[image_num].clone(), false,
                        vec![[0.0, 0.0, 1.0, 1.0].into()])
    .unwrap()
    .end_render_pass()
    .unwrap()
```

The next section will introduce the `draw` command, which we will put between `begin_render_pass`
and `end_render_pass`.
