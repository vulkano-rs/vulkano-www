# Windowing

Up until now, we have only created applications that perform one quick action then exit. The thing
we are going to do next is create a window in order to draw graphics on it.

Strictly speaking, creating a window and handling events is **not** covered by vulkano. Vulkano,
however, is capable of rendering to windows.

## Winit

In order to create a window, we will use the `winit` crate. And while we're at it, we are also
going to add a dependency to the `vulkano-win` crate which is a link between vulkano and winit.

In your Cargo.toml:

```toml
vulkano_win = "0.4"
winit = "0.7"
```

And at the crate root:

```rust
extern crate vulkano_win;
extern crate winit;
```

I encourage you to browse a bit [the documentation of `winit`](https://docs.rs/winit/0.7).

```rust
use winit::EventsLoop;
let events_loop = EventsLoop::new();
```

This code creates a window with the default parameters, and also builds a Vulkan *surface* object
that represents the surface of that window whenever the Vulkan API is concerned.
Calling `window.window()` will return an object that allows you to manipulate the window, and
calling `window.surface()` will return a `Surface` object of `vulkano`.

However, if you try to run this code you will notice that the `build_vk_surface` returns an error.
The reason is that surfaces are actually not part of Vulkan itself, but of several *extension*s
to the Vulkan API. These extensions are disabled by default and need to be manually enabled when
creating the instance before one can use their capabilities.

To make this task easier, the `vulkano_win` provides a function named `required_extensions()` that
will return a list of the extensions that are needed on the current platform.

In order to make this work, we need to modify the way the instance is created:

```rust
let instance = {
    let extensions = vulkano_win::required_extensions();
    Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
};
```

After you made the change, running the program should now work and open then immediately close
a window.

*To be finished*

## Events handling
