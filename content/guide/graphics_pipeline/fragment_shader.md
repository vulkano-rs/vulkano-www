# Fragment shader

After the vertex shader has run on each vertex, the next step that the GPU performs is to determine
which pixels of the target image are within the shape of the triangle. Only these pixels will be
modified on the final image.

> **Note**: More precisely, it is only if the center of a pixel is within the triangle that the
> GPU considers that the whole pixel is inside.

<center>
    <object data='/guide-fragment-shader-1.svg'
            alt='Illustration of which pixels are inside the triangle'>
    </object>
</center>

The GPU then takes each of these pixels one by one (the ones in red in the image above) and runs
another type of shader named a **fragment shader** which we also need to provide in order to start
our draw operation.

Here is what an example fragment shader looks like:

```glsl
#version 460

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
```

The `layout(location = 0) out vec4 f_color;` line declares an output named `f_color`. Vulkan gives
you the possibility to draw to multiple images at once, which is why we need to declare each output
and its type. Drawing to multiple images at once is an advanced topic that isn't covered here.

The `main()` function is executed once for each pixel covered by the triangle and must write in
`f_color` the value that we want to write to the target image. As explained in [a previous
section](/guide/image-clear) these values are normalized, in other words the value `1.0` will in
reality write `255` in memory. In this example since our target image contains colors, we write the
color red.

Next: [Render passes and framebuffers](render-pass-framebuffer)
