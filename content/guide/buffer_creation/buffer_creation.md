# Creating a memory allocator

Before you can create buffers in memory, you have to request (allocate) some memory first.
It turns out [allocating memory](https://docs.rs/vulkano/0.33.0/vulkano/memory/allocator/index.html) 
efficiently and dynamically is challenging. Luckily, in vulkano, we have several kinds of memory 
allocators that we can pick from depending on our use case. Since we don't have any special needs, 
we can use the [`StandardMemoryAllocator`](https://docs.rs/vulkano/0.33.0/vulkano/memory/allocator/type.StandardMemoryAllocator.html) 
with default settings, that kind of allocator is general-purpose and will be your go-to option in 
most cases.

```rust
use vulkano::memory::allocator::StandardMemoryAllocator;

let memory_allocator = StandardMemoryAllocator::new_default(device.clone());
```

Since `device` is actually an `Arc<Device>`, the call to `.clone()` only clones the `Arc`
which isn't expensive. You should get used to passing the device as a parameter, as you will
need to do so for most of the Vulkan objects that you create.

# Creating a buffer

When using Vulkan, you will very often need the GPU to read or write data in memory. In fact
there isn't much point in using the GPU otherwise, as there is nothing you can do with the results
of its work except write them to memory.

In order for the GPU to be able to access some data (either for reading, writing or both), we
first need to create a ***buffer*** object and put the data in it.

## Memory usage

A Vulkan implementation might (and most often does) have multiple *memory types*, each being best
suited to certain tasks. There are many possible arrangements of memory types a Vulkan 
implementation might have, and picking the right one is important to ensure most optimal performance.

When allocating memory for a buffer in vulkano, you have to provide a *memory usage*, which tells
the memory allocator which memory types it should prefer, and which ones it should avoid, when 
picking the right one. For example, if you want to continuously upload data to the GPU, you should
use `MemoryUsage::Upload`; on the other hand, if you have some data that will largely remain 
visible only to the GPU, using `MemoryUsage::DeviceOnly` brings increased performance at the cost
of more complicated data access from the CPU.

The simplest way to create a buffer is to create it in CPU-accessible memory, by using 
`MemoryUsage::Upload` or `MemoryUsage::Download`:

```rust
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::AllocationCreateInfo;

let data: i32 = 12;
let buffer = Buffer::from_data(
    &memory_allocator,
    BufferCreateInfo {
        usage: BufferUsage::UNIFORM_BUFFER,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Upload,
        ..Default::default()
    },
    data,
)
.expect("failed to create buffer");
```

We have to indicate several things when creating the buffer. The first parameter is the memory 
allocator to use. 

The second parameter is the create info for the buffer. The only field that you have to override
is [the usage for which we are creating the
buffer](https://docs.rs/vulkano/0.33.0/vulkano/buffer/struct.BufferUsage.html) for, which can help 
the implementation perform some optimizations. Trying to use a buffer in a way that wasn't 
indicated when creating it will result in an error. For the sake of the example, we just create a 
buffer that supports being used as a uniform buffer.

The third parameter is the create info for the allocation. The field of interest is similarly
[the usage for which we are creating the 
allocation](https://docs.rs/vulkano/latest/vulkano/memory/allocator/enum.MemoryUsage.html). When
creating a CPU-accessible buffer, you will most commonly use `MemoryUsage::Upload`, but in cases 
where the application is writing data through this buffer continuously, using 
`MemoryUsage::Download` is preferred as it may yield some performance gain. Using 
`MemoryUsage::DeviceOnly` will get you a buffer that is inaccessible from the CPU when such a 
memory type exists. Therefore, you can't use this memory usage together with `Buffer::from_data` 
directly, and instead have to create a *staging buffer* whose content is then copied to the 
device-local buffer.

Finally, the fourth parameter is the content of the buffer. Here we create a buffer that contains 
a single integer with the value `12`.

> **Note**: In a real application you shouldn't create buffers with only 4 bytes of data. Although
> buffers aren't expensive, you should try to group as much related data as you can in the same 
> buffer.

## From_data and from_iter

In the example above we create a buffer that contains the value `12`, which is of type `i32`.
but you can put any type you want in a buffer, there is no restriction. In order to give our
arbitrary types a representation that can be used in a generic way, we use the crate `bytemuck`
and its "plain old data" trait, `AnyBitPattern`. Thus, any crate which exposes types with
`bytemuck` support can be used in a buffer. You can also derive `AnyBitPattern` for you own types,
or use the vulkano-provided `BufferContents` derive macro:

```rust
use vulkano::buffer::BufferContents;

#[derive(BufferContents)]
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u32,
}

let data = MyStruct { a: 5, b: 69 };

let buffer = Buffer::from_data(
    &memory_allocator,
    BufferCreateInfo {
        usage: BufferUsage::UNIFORM_BUFFER,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Upload,
        ..Default::default()
    },
    data,
)
.unwrap();
```

While it is sometimes useful to use a buffer that contains a single struct, in practice it is very
common to put an array of values inside a buffer. You can, for example, put an array of fifty
`i32`s in a buffer with the `Buffer::from_data` function.

However, in practice it is also very common to not know the size of the array at compile-time. In
order to handle this, `Buffer` provides a `from_iter` constructor that takes an iterator to the 
data as the last parameter, instead of the data itself.

In the example below, we create a buffer that contains the value `5` of type `u8`, 128 times. The
type of the content of the buffer is `[u8]`, which, in Rust, represents an array of `u8`s whose size
is only known at runtime.

```rust
let iter = (0..128).map(|_| 5u8);
let buffer = Buffer::from_iter(
    &memory_allocator,
    BufferCreateInfo {
        usage: BufferUsage::UNIFORM_BUFFER,
        ..Default::default()
    },
    AllocationCreateInfo {
        usage: MemoryUsage::Upload,
        ..Default::default()
    },
    iter,
)
.unwrap();
```

## Reading and writing the contents of a buffer

Once a CPU-accessible buffer is created, you can access its content with the `read()` or `write()`
methods. Using `read()` will grant you shared access to the content of the buffer, and using
`write()` will grant you exclusive access. This is similar to using a `RwLock`.

For example if `buffer` contains a `MyStruct` (see above):

```rust
let mut content = buffer.write().unwrap();
// `content` implements `DerefMut` whose target is of type `MyStruct` (the content of the buffer)
content.a *= 2;
content.b = 9;
```

Alternatively, suppose that the content of `buffer` is of type `[u8]` (like with the example that
uses `from_iter`):

```rust
let mut content = buffer.write().unwrap();
// this time `content` derefs to `[u8]`
content[12] = 83;
content[7] = 3;
```

Just like the constructors, keep in mind that being able to read/write the content of the buffer
like this is specific to buffer allocated in CPU-accessible memory. Device-local buffers cannot
be accessed in this way.

Next: [Example operation](/guide/example-operation)
