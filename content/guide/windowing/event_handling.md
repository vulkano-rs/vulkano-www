# Window events handling

Now that everything is initialized, let's configure the main loop to actually draw something on the window.

First, let's match two additional events:

```rust
let mut window_resized = false;
let mut recreate_swapchain = false;

event_loop.run(move |event, _, control_flow| match event {
    Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
    } => {
        *control_flow = ControlFlow::Exit;
    }
    Event::WindowEvent {
        event: WindowEvent::Resized(_),
        ..
    } => {
        window_resized = true;
    }
    Event::MainEventsCleared => {}
    _ => (),
});
```

In some situations, like when the window is resized (as the images of the swapchain will no longer match the
window's) the swapchain will become invalid by itself. To continue rendering, we will need to recreate the
swapchain as well as all dependent setup. For that, we will use the `recreate_swapchain` variable, and handle
it before rendering.

The `WindowEvent::WindowResized` will be emitted when the window is, well, resized. When that happens,
we will need to recreate everything that depends on the dimensions of the window. Let's set that in the `window_resized`
variable, and handle it later.

As stated in the winit docs, the `MainEventsCleared` event "will be emitted when all input events have been processed
and redraw processing is about to begin". This essentially enables us to write functionality for each frame.

## Handling invalid swapchains and window resizes

Before starting to use our swapchain, let's write the logic to recreate it
in case of it becoming invalid:

```rust
use vulkano::swapchain::SwapchainCreationError;

Event::RedrawEventsCleared => {
    if recreate_swapchain {
        recreate_swapchain = false;

        let new_dimensions = surface.window().inner_size();

        let (new_swapchain, new_images) = match swapchain
            .recreate()
            .dimensions(new_dimensions.into())
            .build()
        {
            Ok(r) => r,
            // This error tends to happen when the user is manually resizing the window.
            // Simply restarting the loop is the easiest way to fix this issue.
            Err(SwapchainCreationError::UnsupportedDimensions) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
        swapchain = new_swapchain;
        let new_framebuffers = get_framebuffers(&new_images, render_pass.clone());
    }
}
```

Here, as the framebuffers depend on the swapchain images, we will also need to recreate them (for future use).

Next, let's recreate everything that depends on window dimensions. Because the swapchain
will also become invalidated if that happens, let's add some logic for recreating it as well:

```rust
if window_resized || recreate_swapchain {
    recreate_swapchain = false;

    let new_dimensions = surface.window().inner_size();

    let (new_swapchain, new_images) = match swapchain
        .recreate()
        .dimensions(new_dimensions.into())
        .build()
    {
        Ok(r) => r,
        Err(SwapchainCreationError::UnsupportedDimensions) => return,
        Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
    };
    swapchain = new_swapchain;
    let new_framebuffers = get_framebuffers(&new_images, render_pass.clone());

    if window_resized {
        window_resized = false;

        viewport.dimensions = new_dimensions.into();
        let new_pipeline = get_pipeline(
            device.clone(),
            vs.clone(),
            fs.clone(),
            render_pass.clone(),
            viewport.clone(),
        );
        command_buffers = get_command_buffers(
            device.clone(),
            queue.clone(),
            new_pipeline,
            &new_framebuffers,
            vertex_buffer.clone(),
        );
    }
}
```

Here we will update the viewport to the new dimensions. Because we set the pipeline to have a fixed
viewport, we will have to recreate it. The command buffers will depend on the new pipeline and
on the previously recreated framebuffers, so they will need to be recreated as well.

## Acquiring and presenting

To actually start drawing, the first thing that we need to do is to *acquire* an image::

```rust
let (image_i, suboptimal, acquire_future) =
    match swapchain::acquire_next_image(swapchain.clone(), None) {
        Ok(r) => r,
        Err(AcquireError::OutOfDate) => {
            recreate_swapchain = true;
            return;
        }
        Err(e) => panic!("Failed to acquire next image: {:?}", e),
    };
```

The `acquire_next_image()` function returns the image index on which we are allowed to draw, as well as a *future* representing the
moment when the GPU will gain access to that image.

If no image is available (which happens if you submit draw commands too quickly), then the function will
block and wait until there is any. The second parameter is an optional timeout.

Sometimes the function may be suboptimal, were the swapchain image will still work, but may not get properly displayed.
If this happens, we will signal to recreate the swapchain:

```rust
if suboptimal {
    recreate_swapchain = true;
}
```

Previously, we just submitted one command to the gpu, and then waited for it to finish.
Submitting a command produces an object that implements the `GpuFuture` trait,
which holds the resources for as long as they are in use by the GPU.
Destroying an object with this trait blocks the thread until the GPU finishes executing it.

Because now things will happen in a loop, instead of destroying the future object right away, we will keep
it alive between frames. In this way, instead of blocking the CPU side, the GPU can continue working between frames.

To do that, we will start by creating a future representing *now*, and then storing it:

```rust
let mut previous_frame_end = Some(sync::now(device.clone()).boxed());

event_loop.run(move |event, _, control_flow| match event {
// crop
```

Next, let's write some code for the actual future that will be created in each frame:

```rust
let future = previous_frame_end
    .take()
    .unwrap()
    .join(acquire_future)
    .then_execute(queue.clone(), command_buffers[image_i].clone())
    .unwrap()
    .then_swapchain_present(queue.clone(), swapchain.clone(), image_i)
    .then_signal_fence_and_flush();
```

First, we join in with the previous frame (so that we don't need to synchronize again).
Then, execute the respective command buffer and then actually *present* the image to the swapchain.
All of this will happen on the GPU without CPU interaction. This means that the image will only be
presented after the GPU finishes executing the command buffer that will actually draw the triangle.

In the end, we signal a *fence* (a signal to the CPU that the GPU has finished) and flush, to actually
send the command.

If there are errors, we need to handle them right away. Let's do that:

```rust
match future {
    Ok(future) => {
        previous_frame_end = Some(future.boxed());
    }
    Err(FlushError::OutOfDate) => {
        recreate_swapchain = true;
        previous_frame_end = Some(sync::now(device.clone()).boxed());
    }
    Err(e) => {
        println!("Failed to flush future: {:?}", e);
        previous_frame_end = Some(sync::now(device.clone()).boxed());
    }
}
```

Here, we save the future in a case of success, or synchronize and create a new one
in case of failure.

Because we joining with the future from the previous frame, some of the resources that get created
won't be automatically destroyed. We can manually free them by calling a special function:

```rust
Event::RedrawEventsCleared => {
    previous_frame_end.as_mut().unwrap().cleanup_finished();
    // crop
```

You can call it from time to time, but here we are just going to call every frame.

Your program is finally complete! If you run it, it should display a nice triangle on the screen.
If you have any problems, take a look at
[all the code](https://github.com/vulkano-rs/vulkano-www/blob/master/examples/windowing.rs),
and see if you have missed anything.

Next: Going 3D (coming soon).
