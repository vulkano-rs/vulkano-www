# Vertex input

## Vertex buffer

The first part of drawing an object with the graphics pipeline is to describe the shape of this
object. When you think "shape", you may think of squares, circles, etc., but in graphics
programming the most common shapes that one will need to work with are triangles.

> **Note**: Tessellation shaders and alternative `PrimitiveTopology` values unlock the possibility 
> to use other polygons, but this is a more advanced topic.

Each triangle is made of three vertices, and the shape of an object is just a collection of
vertices linked together to form triangles. For the purpose of this guide, we are only going to
draw a single triangle first.

The first step to describe a shape with vulkano is to create a struct named `MyVertex` (the actual
name doesn't matter) whose purpose is to describe the properties of a single vertex. Once this is
done, the shape of our triangle is going to be a buffer whose content is an array of three
`MyVertex` objects.

```rust
use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}
```

Our struct contains a `position` field which we will use to store the position of the vertex on the 
image we are drawing to. Being a vectorial renderer, Vulkan doesn't use coordinates in pixels. 
Instead it considers that the image has a width and a height of 2 units (-1.0 to 1.0), and that the
origin is at the center of the image.

<center><object data="/guide-vertex-input-1.svg"></object></center>

When we give positions to Vulkan, we need to use its coordinate system.

In this guide we are going to draw only a single triangle for now. Let's pick a shape for it,
for example this one:

<center><object data="/guide-vertex-input-2.svg"></object></center>

Which translates into this code:

```rust
let vertex1 = MyVertex { position: [-0.5, -0.5] };
let vertex2 = MyVertex { position: [ 0.0,  0.5] };
let vertex3 = MyVertex { position: [ 0.5, -0.25] };
```

> **Note**: The field that contains the position is named `position`, but note that this name is
> arbitrary. We will see below how to actually pass that position to the GPU.

Now all we have to do is create a buffer that contains these three vertices. This buffer
will be passed as a parameter when we start the drawing operation.

```rust
let vertex_buffer = Buffer::from_iter(
    &memory_allocator,
    BufferCreateInfo {
        usage: BufferUsage::VERTEX_BUFFER,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Upload,
        ..Default::default()
    },
    vec![vertex1, vertex2, vertex3],
)
.unwrap();
```

A buffer that contains a collection of vertices is commonly named a *vertex buffer*. Because we
know the specific use of this buffer is for storing vertices, we specify the usage flag 
`VERTEX_BUFFER`.

> **Note**: Vertex buffers are not special in any way. The term *vertex buffer* indicates the way 
> the programmer intends to use the buffer, and it is not a property of the buffer.

## Vertex shader

At the start of the drawing operation, the GPU is going to pick each element from this buffer one
by one and call a ***vertex shader*** on them.

Here is what the source code of a vertex shader looks like:

```glsl
#version 460

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
```

The line `layout(location = 0) in vec2 position;` declares that each vertex has an *attribute* 
named `position` and of type `vec2`. This corresponds to the definition of the `MyVertex` struct we 
created.

> **Note**: Calling the `impl_vertex!` macro is what makes it possible for vulkano to build the
> link between the content of the buffer and the input of the vertex shader.

The `main` function is called once for each vertex, and sets the value of the `gl_Position`
variable to a `vec4` whose first two components are the position of the vertex.

`gl_Position` is a special "magic" global variable that exists only in the context of a vertex
shader and whose value must be set to the position of the vertex on the surface. This is how the
GPU knows how to position our shape.

## After the vertex shader

After the vertex shader has run on each vertex, the GPU knows where our shape is located on the
screen. It then proceeds to call [the fragment shader](/guide/fragment-shader).
