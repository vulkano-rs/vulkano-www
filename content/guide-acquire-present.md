# Usage a swapchain

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

*To be finished*

## Clearing the image

*To be finished*

## Advanced : present modes
