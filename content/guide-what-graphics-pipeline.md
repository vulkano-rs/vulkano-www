# Graphics pipeline introduction

Up until now, we have created command buffers that perform two kind of operations:

- Memory transfers (copying data between buffers and images, clearing an image).
- Compute operations (dispatching a compute shader).

While these two kind of operations are sufficient in order to use the power of the GPU for
parallel calculations, there is a third kind of operations: graphical operations.

Before they were used for general-purpose calculations, GPU were used for graphics (hence their
name). To do so, GPUs provide to developers a specialized well-optimized serie of steps called
***the graphics pipeline***.

> **Note**: There is nothing that the graphics pipeline can do that a compute pipeline couldn't do.
> However the graphics pipeline is much more specialized and therefore much more optimized. For
> example, some parts of the graphics pipeline can be handled by dedicated chips on the hardware.

The graphics pipeline can look complex if you haven't done any graphics programming before, but it
is essential to understand it if you want to draw images in an efficient way.

## Introduction

The purpose of the graphics pipeline is to draw an shape on an image. This shape can be as simple
as a single triangle, or can be very complex like a human body for example.

In order to start a graphics operation (ie. an operation that uses the graphics pipeline), you will
need the following elements:

- A **graphics pipeline object** that describes the way the GPU should behavior, similar to the
  compute pipeline object we created in [a previous section](/guide/compute-pipeline).
- One or multiple buffers containing the shape of the object we want to draw.
- A ***framebuffer*** object, which is a collection of images to write to.
- Just like compute pipelines, we can also pass descriptor sets and push constants.

*To be finished*
