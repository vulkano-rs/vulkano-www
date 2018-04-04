# Exporting the content of an image

In [the previous section](/guide/image-clear) we filled the image with a color.

But you may now wonder how to see the result of this operation. As explained previously, images
are opaque structures whose actual layout is implementation-specific. So how do we read their
content?

The answer to this question is that we have to create a buffer and ask the GPU to copy the content
of the image to the buffer.

## Copying from the image to the buffer

The first step is to create the buffer, as we have already covered in previous sections. The buffer
has to be large enough, otherwise the copy will result in an error. Each pixel of the image
contains four unsigned 8-bit values, and the image dimensions are 1024 by 1024 pixels. Hence why
the number of elements in the buffer is `1024 * 1024 * 4`.

```rust
let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                         (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                         .expect("failed to create buffer");
```

And let's modify the command buffer we created in the previous section to add the copy operation:

```rust
let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
    .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
    .copy_image_to_buffer(image.clone(), buf.clone()).unwrap()
    .build().unwrap();
```

Since this is a memory transfer operation, this time the image values are *not* interpreted as
floating-point values. The memory content of the image (unsigned 8-bit values) is directly copied
to the buffer.

Let's not forget to execute the command buffer and block until the operation is finished:

```rust
let finished = command_buffer.execute(queue.clone()).unwrap();
finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();
```

## Turning the image into a PNG

Now that we have a buffer that contains our image data, we will visualize it by saving it as a PNG
file. The Rust ecosystem has a crate named `image` that can do this.
Let's add it to our Cargo.toml:

```toml
image = "0.14"
```

And to our crate root:

```rust
extern crate image;
```

In this library the main type that represents an image is the `ImageBuffer`. It can be created
from a slice:

```rust
use image::{ImageBuffer, Rgba};

let buffer_content = buf.read().unwrap();
let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
```

The `ImageBuffer` can then be saved into a PNG file:

```rust
image.save("image.png").unwrap();
```

And that's it! When running your program, a blue image named `image.png` should appear.

<center>
![](/guide-image-export-1.png)

*Here it is.*
</center>

This might look stupid, but think about the fact that it's the GPU that wrote the content of
the image. In the next sections we will do more than just fill an image with blue, but we will
continue to retreive the image's content and write it to a PNG file.
