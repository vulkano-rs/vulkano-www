# Initialization

## Creating an instance

Before you can start utilizing the Vulkan API, the first thing to do is to create
an *instance*. An instance specifies the mapping between vulkano and the local Vulkan library.
As of vulkano version `0.31.0`, the library needs to be explicitly specified by passing a 
`VulkanLibrary` to the  `Instance` constructor.

For starters, our program will be very simple, so, for now, creating an instance won't need any
[additional parameters](https://docs.rs/vulkano/0.33.0/vulkano/instance/struct.InstanceCreateInfo.html),
so we can create it with default configurations:

```rust
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateInfo};

let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
let instance = Instance::new(library, InstanceCreateInfo::default())
    .expect("failed to create instance");
```

Like many other functions in vulkano, creating an instance returns a `Result`. If Vulkan is not
available on the system, this result will contain an error. For the sake of this example we call
`expect` on the `Result`, which prints a message to stderr and terminates the application if it
contains an error. In a real game or application you should handle that situation in a nicer way,
for example by opening a dialog box with an explanation. This is out of scope of this guide.

Before going further you can try your code by running:

```bash
cargo run
```

## Enumerating physical devices

The machine you run your program on may have multiple devices that support Vulkan. Before we can
ask a video card to perform some operations, we have to enumerate all the *physical device*s that
support Vulkan and choose which one we are going to use for this operation.

In reality a physical device can be a dedicated graphics card, but also an integrated graphics
processor or a software implementation. It can be basically anything that allows running Vulkan
operations.

As of the writing of this guide, it is not yet possible to use multiple devices simultaneously
in an efficient way (eg. SLI/Crossfire). You *can* use multiple devices simultaneously in the same
program, but there is not much point in doing so because you cannot share anything between them.
Consequently the best thing to do in practice is to choose one physical device which is going to 
run everything:

```rust
let physical_device = instance
    .enumerate_physical_devices()
    .expect("could not enumerate devices")
    .next()
    .expect("no devices available");
```

The `enumerate_physical_devices` function returns a `Result` of an iterator to the list of 
available physical devices. We call `next` on it to return the first device, if any. Note that the 
first device is not necessarily the best device. In a real program you probably want to leave the 
choice to the user (later we will see a better implementation of this).

Keep in mind that the list of physical devices can be empty. This happens if Vulkan is installed
on the system, but none of the physical devices of the machine are capable of supporting Vulkan. In
a real-world application you are encouraged to handle this situation properly as well.

Next: [Device creation](/guide/device-creation)
