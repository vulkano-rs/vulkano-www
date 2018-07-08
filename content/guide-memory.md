## Introduction to memory

When you write some program (either in Rust or any other programming language) and run it, the
program's instructions are executed by the CPU and the program's variables are typically stored in
RAM. The CPU and RAM are physically close to each other on the motherboard.

Similarly, your video card has its own microprocessor called the *GPU* or the *graphics processor*,
and its own RAM which we usually call the *video RAM*, the *VRAM*, or the *video memory*. It can
be seen more or less as a secondary machine within your main machine.

The CPU can read and write very quickly to the RAM (as they are close to each other), and the GPU
can read and write very quickly to the video memory (as they are close to each other as well). It
is also possible for the GPU to access RAM, and usually also possible for the CPU to access the
video memory, but the read and write accesses are going to be much slower as they have to go
through the PCI Express bus that connects your video card to your motherboard.

<center>
<object data="/guide-buffer-creation-1.svg"></object>
</center>

> **Note**: All of this is true only for desktop machines with video cards, which is what we
> are going to focus on. Mobile machines usually share the same memory for both the CPU and the
> graphics processor. In addition to this, it is possible to have for example a software
> implementation of Vulkan that doesn't even use a GPU.
