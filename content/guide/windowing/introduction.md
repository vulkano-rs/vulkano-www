# Windowing introduction

Up until now, we have only created applications that perform one quick action and then exit. What
we are going to do next is create a window in order to draw graphics on it, and keep our
application running forever until the window is closed.

Strictly speaking, creating a window and handling events is **not** covered by vulkano. Vulkano,
however, is capable of rendering to window(s).

> **Note**: You can find the [full source code of this chapter
> here](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/src/bin/windowing.rs).

## Creating a window

In order to create a window, we will use the `winit` crate. And while we're at it, we are also
going to add a dependency to the `vulkano-win` crate which is a link between vulkano and winit.

Add to your `Cargo.toml` dependencies:

```toml
vulkano-win = "0.33.0"
winit = "0.28.3"
```

We encourage you to browse [the documentation of `winit`](https://docs.rs/winit).

Because the objects that come with creating a window are not part of Vulkan itself, the first thing 
that you will need to do is to enable all non-core extensions required to draw a window. 
`vulkano_win` automatically provides them for us, so the only thing left is to pass them on to the 
instance creation:

```rust
use vulkano::instance::{Instance, InstanceCreateInfo};

let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
let required_extensions = vulkano_win::required_extensions(&library);
let instance = Instance::new(
    library,
    InstanceCreateInfo {
        enabled_extensions: required_extensions,
        ..Default::default()
    },
)
.expect("failed to create instance");
```

Now, let's create the actual window:

```rust
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::{EventLoop};
use winit::window::{WindowBuilder};

let event_loop = EventLoop::new();  // ignore this for now
let surface = WindowBuilder::new()
    .build_vk_surface(&event_loop, instance.clone())
    .unwrap();
```

As you can see, we created a new object, called *surface*.

The *surface* is a cross-platform abstraction over the actual window object, that vulkano can use 
for rendering. As for the window itself, it can be retrieved by calling `surface.window()`, which 
you can use to manipulate and change its default properties.

After you made the change, running the program should now open a window, then immediately
close it when the `main` function exits.

## Events handling

In order to make our application run for as long as the window is alive, we need to handle the
window's events. This is typically done after initialization, and right before the end of the
`main` function. Using the `event_loop` object:

```rust
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;

event_loop.run(|event, _, control_flow| {
    match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        },
        _ => ()
    }
});
```

What this code does is block the main thread forever, and calls the closure whenever the events
loop (which we used to create our window) receives an event. These events include the events that 
are tied to our window, such as mouse movements.

When the user wants to close the window, a `WindowEvent::CloseRequested` event is received, which 
makes our closure set the `control_flow` to `ControlFlow::Exit` which signals to winit that we want 
an exit.

<!-- todo: is this correct? -->
<!-- > **Note**: Since there is nothing to stop it, the window will try to update as quickly as it can,
> likely using all the power it can get from one of your cores.
> We will change that, however, in the incoming chapters. -->

Right now, all we're doing is creating a window and keeping our program alive for as long as the
window isn't closed. The next section will show how to initialize what is called a *swapchain* on
the window's surface.

Next: [Swapchain creation](/guide/windowing/swapchain-creation)
