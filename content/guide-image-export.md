# Exporting the content of an imaage

In [the previous section](/guide/image-clear) we cleared an image in blue.

But you may now wonder how to see the result of this operation. As explained previously, images
are opaque structures whose actual layout is implementation-specific. So how do we read their
content?

The answer to this question is that we have to create a buffer and ask the GPU to copy the content
of the image to the buffer.

## Turning the image into a PNG
