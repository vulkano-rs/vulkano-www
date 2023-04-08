# Clearing an image

Contrary to buffers, images have an opaque implementation-specific memory layout. What this means
is that you can't modify an image by directly writing to its memory. There is no such thing as a
`CpuAccessibleImage`.

<!-- todo: Does vulkano support it now?-->
> **Note**: In reality Vulkan also allows you to create *linear* images, which can be modified but
> are much slower and are supposed to be used only in some limited situations. Vulkano doesn't
> support them yet.

Therefore the only way to read or write to an image is to ask the GPU to do it. This is exactly
what we are going to do by asking the GPU to fill our image with a specific color. This is called
*clearing* an image.

```rust
use vulkano::command_buffer::ClearColorImageInfo;
use vulkano::format::ClearColorValue;

let mut builder = AutoCommandBufferBuilder::primary(
    device.clone(),
    queue.queue_family_index(),
    CommandBufferUsage::OneTimeSubmit,
)
.unwrap();

builder
    .clear_color_image(ClearColorImageInfo {
        clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
        ..ClearColorImageInfo::image(image.clone())
    })
    .unwrap();

let command_buffer = builder.build().unwrap();
```

> **Note**: The function is called clearing a *color* image, as opposed to depth and/or stencil
> images which we haven't covered yet.

## Normalized components

[The `ClearColorValue` enum](https://docs.rs/vulkano/0.33.0/vulkano/format/enum.ClearColorValue.html) indicates
which color to fill the image with. Depending on the format of the image, we have to use the right
enum variant of `ClearValue`.

Here we pass floating-point values because the image was created with the `R8G8B8A8_UNORM` format.
The `R8G8B8A8` part means that the four components are stored in 8 bits each, while the `UNORM`
suffix means "unsigned normalized". The coordinates being "normalized" means that their value in
memory (ranging between 0 and 255) is interpreted as floating point values. The in-memory value `0`
is interpreted as the floating-point `0.0`, and the in-memory value `255` is interpreted as the
floating-point `1.0`.

With any format whose suffix is `UNORM` (but also `SNORM` and `SRGB`), all the operations that are
performed on the image (with the exception of memory copies) treat the image as if it contained
floating-point values. This is the reason why we pass `[0.0, 0.0, 1.0, 1.0]`. The values `1.0` will
in fact be stored as `255` in memory.

Next: [Exporting the result](/guide/image-export)
