# Introduction

Welcome to the vulkano guide! This guide will cover the basics of Vulkan and vulkano, and will help 
you get started with interfacing with a GPU.

It will teach you the basics of graphics programming in the sense that you will know how to draw 
objects on the screen. However, this guide doesn't cover actual graphics programming techniques, 
such as loading a 3D model or adding realistic lighting to a scene. At this point, the examples in 
the guide are very basic, but we will be adding more comprehensive tutorials in the future.

We will assume that you are familiar with the Rust programming language. If you are not, you should 
definitely [learn it first](https://www.rust-lang.org/documentation.html)!

An excellent way to learn is to read examples. On top of this guide, you should familiarize 
yourself with [vulkano examples](https://github.com/vulkano-rs/vulkano/tree/master/examples). To 
view the current release's examples you'll need to switch to a version tag. On the branch dropdown 
click the tags tab. There you'll find all released versions. Master branch will contain examples 
that are updated continuously to match changes that are unreleased. You should only use the master 
branch if you are using vulkano as a git dependency.

To contribute to this guide, you can create a pull request at 
[vulkano-www](https://github.com/vulkano-rs/vulkano-www) repository.

## Quick glossary

When you create a program (either in Rust or any other programming language) and run it, the
program's instructions are executed by the ***CPU*** (Central Processing Unit).

But some computers also usually have a ***video card*** plugged in them. This video card has its
own microprocessor called the ***GPU*** (Graphics Processing Unit) or the ***graphics processor***.
It can be seen more or less as a secondary machine within your main machine. Your monitor is
generally plugged in to your video card if you have one.

***Vulkan*** is a standard API whose version 1.0 was released in 2016 that lets you interface with
the video card and the GPU of the machine your program is running on. ***Vulkano*** is a Rust
library on top of Vulkan that makes it much easier and safer to use. After you have learned to
use Vulkan/vulkano, you will be able to ask your GPU to perform operations and either write the
result into memory (which you can then read from your Rust program), or to write the result to your
monitor for you to physically see.

## Setup

You may first need to set up some external dependencies as documented in the [vulkano 
readme](https://github.com/vulkano-rs/vulkano/blob/master/README.md#setup-and-troubleshooting) to 
avoid build failures in the future.

To get started with vulkano, add it as a project dependency to your Cargo.toml:

```toml
[dependencies]
vulkano = "0.33.0"
```

You may want to consider adding the following minimum optimization level to your `Cargo.toml` as 
well. The purpose of this is to reduce performance artifacts resulting from the default debug 
optimization.

```toml
[profile.dev]
opt-level = 1 
```

Note: If you run into any issues with this guide, please [open an 
issue](https://github.com/vulkano-rs/vulkano-www/issues). If you have issues with vulkano itself, 
please also [open an issue](https://github.com/vulkano-rs/vulkano/issues).

You are now ready to [get started](/guide/initialization)!
