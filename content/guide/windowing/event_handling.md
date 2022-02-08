# Window events handling

Now that everything is initialized, let's configure the main loop to actually draw something on the window.

We are going to match for 2 additional events:

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
        recreate_swapchain = true;
    }
    Event::RedrawEventsCleared => {}
    _ => (),
});
```

In some situations, like when the window is resized (as the images of the swapchain will no longer match the
window's) the swapchain will become invalid by itself. To continue rendering, we will need to recreate the
swapchain as well as all dependent setup. We will remember to do this for the next loop iteration.

Everything that comes after the `RedrawEventsCleared` event will be drawn each frame. Let's start by
recreating the swapchain when needed:

```rust
use vulkano::swapchain::SwapchainCreationError;

Event::RedrawEventsCleared => {
    if recreate_swapchain {
        recreate_swapchain = false;
    
        let new_dimensions: [u32; 2] = surface.window().inner_size().into();
    
        let (new_swapchain, new_images) = match swapchain
            .recreate()
            .dimensions(new_dimensions)
            .build()
        {
            Ok(r) => r,
            // This error tends to happen when the user is manually resizing the window.
            // Simply restarting the loop is the easiest way to fix this issue.
            Err(SwapchainCreationError::UnsupportedDimensions) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
    
        swapchain = new_swapchain;
        framebuffers = get_framebuffers(&new_images, render_pass.clone());
    }
}
```



<!-- todo -->

In order to use the swapchain, we have to start by *acquiring* an image. This is done with the
`swapchain::acquire_next_image()` function.

```rust
let (image_num, acquire_future) = swapchain::acquire_next_image(swapchain.clone(), None).unwrap();
```

This function call returns a tuple. The first element is a `usize` corresponding to the index of
the image within the `images` array of the image which is now available to us. The second element
of the tuple is a *future* that represents the moment when the image will be acquired by the GPU.

The `acquire_next_image` function will block until an image is available. This can happen depending
on the present mode.

*To be finished* - check the [Triangle example](https://github.com/vulkano-rs/vulkano-examples/blob/master/src/bin/triangle.rs) for now.

## Clearing the image

*To be finished*

## Advanced : present modes
