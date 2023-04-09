# Window events handling

Now that everything is initialized, let's configure the main loop to actually draw something on the 
window.

First, let's match two additional events:

```rust
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

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

In some situations, like when the window is resized (as the images of the swapchain will no longer 
match the window's) the swapchain will become invalid by itself. To continue rendering, we will 
need to recreate the swapchain as well as all dependent setup. For that, we will use the 
`recreate_swapchain` variable, and handle it before rendering.

The `WindowEvent::WindowResized` will be emitted when the window is, well, resized. When that 
happens, we will need to recreate everything that depends on the dimensions of the window. Let's 
set that in the `window_resized` variable, and handle it later.

As stated in the winit docs, the `MainEventsCleared` event "will be emitted when all input events 
have been processed and redraw processing is about to begin". This essentially enables us to write 
functionality for each frame.

## Handling invalid swapchains and window resizes

Before starting to use our swapchain, let's write the logic to recreate it in case of it becoming 
invalid:

```rust
use vulkano::swapchain::{SwapchainCreateInfo, SwapchainCreationError};

Event::RedrawEventsCleared => {
    if recreate_swapchain {
        recreate_swapchain = false;

        let new_dimensions = surface.window().inner_size();

        let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
            image_extent: new_dimensions.into(), // here, "image_extend" will correspond to the window dimensions
            ..swapchain.create_info()
        }) {
            Ok(r) => r,
            // This error tends to happen when the user is manually resizing the window.
            // Simply restarting the loop is the easiest way to fix this issue.
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("failed to recreate swapchain: {e}"),
        };
        swapchain = new_swapchain;
        let new_framebuffers = get_framebuffers(&new_images, &render_pass);
    }
}
```

Here, as the framebuffers depend on the swapchain images, we will also need to recreate them (for 
future use).

Next, let's recreate everything that depends on window dimensions. Because the swapchain will also 
become invalidated if that happens, let's add some logic for recreating it as well:

```rust
if window_resized || recreate_swapchain {
    recreate_swapchain = false;

    let new_dimensions = surface.window().inner_size();

    let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
        image_extent: new_dimensions.into(),
        ..swapchain.create_info()
    }) {
        Ok(r) => r,
        Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
        Err(e) => panic!("failed to recreate swapchain: {e}"),
    };
    swapchain = new_swapchain;
    let new_framebuffers = get_framebuffers(&new_images, &render_pass);

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
            &device,
            &queue,
            &new_pipeline,
            &new_framebuffers,
            &vertex_buffer,
        );
    }
}
```

We will update the viewport to the new dimensions, and because we set the pipeline to have a fixed
viewport, we will have to recreate it. The command buffers will depend on the new pipeline and on 
the previously recreated framebuffers, so they will need to be recreated as well.

## Acquiring and presenting

To actually start drawing, the first thing that we need to do is to *acquire* an image to draw:

```rust
let (image_i, suboptimal, acquire_future) =
    match swapchain::acquire_next_image(swapchain.clone(), None) {
        Ok(r) => r,
        Err(AcquireError::OutOfDate) => {
            recreate_swapchain = true;
            return;
        }
        Err(e) => panic!("failed to acquire next image: {e}"),
    };
```

The `acquire_next_image()` function returns the image index on which we are allowed to draw, as 
well as a *future* representing the moment when the GPU will gain access to that image.

If no image is available (which happens if you submit draw commands too quickly), then the function 
will block and wait until there is. The second parameter is an optional timeout.

Sometimes the function may be suboptimal, were the swapchain image will still work, but may not get 
properly displayed. If this happens, we will signal to recreate the swapchain:

```rust
if suboptimal {
    recreate_swapchain = true;
}
```

The next step is to create the future that will be submitted to the GPU:

```rust
use vulkano::swapchain::PresentInfo;

let execution = sync::now(device.clone())
    .join(acquire_future)
    .then_execute(queue.clone(), command_buffers[image_i].clone())
    .unwrap()
    .then_swapchain_present(
        queue.clone(),
        PresentInfo {
            index: image_i,
            ..PresentInfo::swapchain(swapchain.clone())
        },
    )
    .then_signal_fence_and_flush();
```

Like we did in earlier chapters, we start by synchronizing. However, the command buffer can't be 
executed immediately, as it needs to wait for the image to actually become available. To do that,
we `.join()` with the other future that we got from `acquire_next_image()`, the two representing 
the moment where we have synchronized *and* actually acquired the said image. We can then instruct 
the GPU to execute our main command buffer as usual (we select it by using the image index).

In the end, we need to *present* the image to the swapchain, telling it that we have finished 
drawing and the image is ready for display. Don't forget to add a fence and flush the future.

We are now doing more than just executing a command buffer, so let's do a bit of error handling:

```rust
match execution {
    Ok(future) => {
        future.wait(None).unwrap();  // wait for the GPU to finish
    }
    Err(FlushError::OutOfDate) => {
        recreate_swapchain = true;
    }
    Err(e) => {
        println!("Failed to flush future: {e}");
    }
}
```

For now, we will just wait for the GPU to process all of its operations.

Finally, your triangle is complete! Well, almost, as you probably don't want for the CPU to just 
wait every frame for the GPU without actually doing anything. Anyways, if you execute your program 
now, you should see the window popup with a nice triangle, which you can resize without crashing.

## Frames in flight: executing instructions parallel to the GPU

Currently the CPU waits between frames for the GPU to finish, which is somewhat inefficient. What 
we are going to do now is to implement the functionality of *frames in flight*, allowing the CPU to 
start processing new frames while the GPU is working on older ones.

To do that, we need to save the created fences and reuse them later. Each stored fence will 
correspond to a new frame that is being processed in advance. You can do it with only one fence
(check vulkano's [triangle 
example](https://github.com/vulkano-rs/vulkano/blob/v0.33.0/examples/src/bin/triangle.rs) if you 
want to do something like that). However, here we will use multiple fences (likewise multiple 
frames in flight), which will make easier for you implement any other synchronization technique 
you want.

Because each fence belongs to a specific future, we will actually store the futures as we create 
them, which will automatically hold each of their specific resources. We won't need to synchronize 
each frame, as we can just join with the previous frames (as all of the operations should happen 
continuously, anyway).

> **Note**: Here we will use *fence* and *future* somewhat interchangeably, as each fence 
> corresponds to a future and vice versa. Each time we mention a fence, think of it as a future 
> that incorporates a fence.

In this example we will, for simplicity, correspond each of our fences to one image, making us able 
to use all of the existing command buffers at the same time without worrying much about what 
resources are used in each future. If you want something different, the key is to make sure each 
future uses resources that are not already in use (this includes images and command buffers).

Let's first create the vector that will store all of the fences:

```rust
use vulkano::sync::FenceSignalFuture;

let frames_in_flight = images.len();
let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
let mut previous_fence_i = 0;

event_loop.run(move |event, _, control_flow| match event {
    // crop
```

Because the fences don't exist at the start (or happen to stop existing because of an error), they 
are wrapped inside an Option. Each future containing the fence has the information of the previous 
one, of which the type is contained inside `_`. We will also be storing them in a `Arc`, which will 
automatically free them when all the references are dropped.

At the end of your main loop, remove all the previous future logic. Each frame, we will substitute 
the fence that corresponds to the image we have acquired. To make sure the new future and the old 
one will not be using the same image, we will wait for the old future to complete and free its 
resources:

```rust
// wait for the fence related to this image to finish
// normally this would be the oldest fence, that most likely have already finished
if let Some(image_fence) = &fences[image_i] {
    image_fence.wait(None).unwrap();
}
```

We will join with the future from the previous frame, so that we only need to synchronize if the 
future doesn't already exist:

```rust
let previous_future = match fences[previous_fence_i].clone() {
    // Create a NowFuture
    None => {
        let mut now = sync::now(device.clone());
        now.cleanup_finished();

        now.boxed()
    }
    // Use the existing FenceSignalFuture
    Some(fence) => fence.boxed(),
};
```

Here, we call `.boxed()` to our futures to store them in a heap, as they can have different sizes.
The `now.cleanup_finished();` function will manually free all not used resources (which could still 
be there because of an error).

Now that we have the `previous_future`, we can join and create a new one as usual:

```rust
let future = previous_future
    .join(acquire_future)
    .then_execute(queue.clone(), command_buffers[image_i].clone())
    .unwrap()
    .then_swapchain_present(
        queue.clone(),
        PresentInfo {
            index: image_i,
            ..PresentInfo::swapchain(swapchain.clone())
        },
    )
    .then_signal_fence_and_flush();
```

And then substitute the old (obsolete) fence in the error handling:

```rust
fences[image_i] = match future {
    Ok(value) => Some(Arc::new(value)),
    Err(FlushError::OutOfDate) => {
        recreate_swapchain = true;
        None
    }
    Err(e) => {
        println!("Failed to flush future: {e}");
        None
    }
};
```

Don't forget to set `previous_fence_i` for the next frame:

```rust
previous_fence_i = image_i;
```

In the end, we finally achieved a fully working triangle. The next step is to start moving it and 
changing it properties, but that's something for the next chapter.

If you have any problems, take a look at the [full source 
code](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/src/bin/windowing.rs), and 
see if you have missed anything.

Next: (coming soon).
