# Graphics pipeline introduction

Up until now, we have created command buffers that perform two kind of operations:

- Memory transfers (copying data between buffers and images, clearing an image).
- Compute operations (dispatching a compute shader).

While these two kind of operations are sufficient in order to use the power of the GPU for
parallel calculations (as seen in [the Mandelbrot example](/guide/mandelbrot)), there is a third
kind of operations: graphical operations.

Before they were used for general-purpose calculations, GPUs were used for graphics (hence their
name). To benefit from this, GPUs provide to developers a specialized well-optimized series of 
steps called ***the graphics pipeline***. Using the graphics pipeline is more restrictive than 
using compute operations, but it is also much faster.

> **Note**: There is nothing that the graphics pipeline can do that a compute pipeline couldn't do.
> However the graphics pipeline is much more specialized and therefore much more optimized. Some
> parts of the graphics pipeline are generally handled by dedicated chips on the hardware.

Using the graphics pipeline can look complex if you haven't done any graphics programming before,
but it is essential to understand it if you want to render images in an efficient way.

## Quick introduction

The purpose of the graphics pipeline is to draw a certain shape on an image. This shape can be as
simple as a single triangle, or as complex as a mountain range.

In order to start a graphical operation (i.e. an operation that uses the graphics pipeline), you
will need the following elements:

- A **graphics pipeline object** that describes the way the GPU should behave, similar to the
  way [a compute pipeline object](/guide/compute-pipeline) describes a compute operation.
- One or multiple buffers containing the shape of the object we want to draw.
- A ***framebuffer*** object, which is a collection of images to write to.
- Just like compute pipelines, we can also pass descriptor sets (and push constants, which we
  haven't covered yet).

When you start a graphics operation, the GPU will start by executing a ***vertex shader*** (that
is part of the graphics pipeline object) on each vertex of the shape that you want to draw. This
first step will allow you to position the shape on the screen.

Then the GPU finds out which pixels of the target image are covered by the shape, and executes a
***fragment shader*** (also part of the graphics pipeline object) on each of these pixels. This
shader is used to determine what is the color of the shape for the given pixel is. Finally the
GPU will merge this color with the color that already exists at this location.

The ***graphics pipeline object*** contains the vertex shader, the fragment shader, plus various
options that allows one to further configure the behavior of the graphics card.

> **Note**: This explanation only covers the fundamentals of graphics pipelines. Graphics pipelines
> have tons of configurable options, plus additional optional shader stages.

The next sections will be dedicated to covering graphics pipeline in more details.

Next: [Vertex input](/guide/vertex-input)
