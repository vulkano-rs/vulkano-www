# Windowing

This guide is still a work in progress. For more thorough examples, take a look at the [examples](https://github.com/vulkano-rs/vulkano/tree/master/examples).

Up until now, we have only created applications that perform one quick action then exit. What
we are going to do next is to create a window in order to draw graphics on it, and keep our
application running forever until the window is closed.

Strictly speaking, creating a window and handling events is **not** covered by vulkano. Vulkano,
however, is capable of rendering to window(s).

## Creating a window

In order to create a window, we will use the `winit` crate. And while we're at it, we are also
going to add a dependency to the `vulkano-win` crate which is a link between vulkano and winit.

In your Cargo.toml add:

```toml
vulkano-win = "0.28.0"
winit = "0.26"
```

I encourage you to browse [the documentation of `winit`](https://docs.rs/winit).
Let's start by creating a window with it:

```rust
use vulkano::instance::Instance;
use vulkano::instance::Version;
use vulkano_win::create_vk_surface_from_handle;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let instance = Instance::new(
        None,
        Version::V1_1,
        &vulkano_win::required_extensions(),
        None,
    )
    .expect("failed to create Vulkan instance");

    let events_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&events_loop).unwrap();
    let _surface = create_vk_surface_from_handle(window, instance).unwrap();
}
```

This code creates a window with the default parameters, and also builds a Vulkan *surface* object
that represents the surface of that window whenever the Vulkan API is concerned.
Calling `surface.window()` will return an object that allows you to manipulate the window.

However, if you try to run this code you will notice that the `build_vk_surface` returns an error.
The reason is that surfaces are actually not part of Vulkan itself, but one of several
*extension*s to the Vulkan API. These extensions are disabled by default and need to be manually
enabled when creating the instance before one can use their capabilities.

To make this task easier, the `vulkano_win` provides a function named `required_extensions()` that
will return a list of the extensions that are needed on the current platform.

Therefore in order to make this work, we need to modify the way the instance is created:

```rust
let instance = {
    let extensions = vulkano_win::required_extensions();
    Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
};
```

After you made the change, running the program should now work and open a window, then immediately
close it when the `main` function exits.

## Events handling

In order to make our application run for as long as the window is alive, we need to handle the
window's events. This is typically done after initialization, and right before the end of the
`main` function.

```rust
events_loop.run(|event, _, control_flow| {
    match event {
        winit::Event::WindowEvent { event: winit::WindowEvent::CloseRequested, .. } => {
            *control_flow = ControlFlow::Exit;
        },
        _ => ()
    }
});
```

What this code does is block the main thread forever, and calls the closure whenever the events
loop (which we used to create our window) receives an event. These events include the events
that are tied to our window, such a mouse movements.

When the user wants to close the window, a `Closed` event is received, which makes our closure
set the `control_flow` to `ControlFlow::Exit` which signals to winit that we want an exit.

## Conclusion

Right now, all we're doing is creating a window and keeping our program alive for as long as the
window isn't closed. The [next section](/guide/swapchain-creation) will show how to initialize what is called a *swapchain* on
the window's surface.
