# Creating a buffer

When using Vulkan, you will very often need for the GPU to read or write data in memory. In fact
there isnt's much point in using the GPU otherwise, as you will need to ask it to write the results
of its calculations somewhere.

In order for the GPU to be able to access some data (either for reading, writing or both), we
first need to create a ***buffer*** and put the data in it.

## Several kinds of buffers

Vulkano doesn't provide a `Buffer` object which you could create with `Buffer::new`. Instead it
provides several different structs that all represent buffers, each of these structs being optimal
for a certain kind of usage. For example if you want to continuously upload data you should use a
`CpuBufferPool`, while on the other hand if you have some data that you are never going to modify
you should use an `ImmutableBuffer`.

The most simple kind of buffer that exists is the `CpuAccessibleBuffer`, which can be created
like this:

```rust
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                            Some(queue.family()), 12)
                                            .expect("failed to create buffer");
```

We have to indicate several things when creating the buffer. The first parameter is the device
to use. Since `device` is actually an `Arc<Device>`, the call to `.clone()` only clones the `Arc`
and should be cheap. You should get used to passing the device as parameter, as you will need
to do so for almost all Vulkan objects that you create.

The second parameter indicates for which purpose we are creating the buffer, which can help the
implementation perform some optimizations. For the sake of the example, we just create a
`BufferUsage` that corresponds to all possible usages.

The third parameter is the list of queue families that are going to access the buffer. Accessing it
from another family will trigger an error. Again, this is used by the Vulkan implementation to
perform some optimizations.

> **Note**: Vulkano may provide some shortcut functions in the future for the most common usages.

Finally, the last parameter is the content of the buffer. Here as you can see we create a buffer
that contains a single integer with the value `12`. In a real application you shouldn't create
such small buffers. Although buffers aren't be expensive, you should try to group multiple
values in the same buffer nonetheless.

## From_data and from_iter

In the example above we create a buffer that contains the value `12`, which is of type `i32`.
But you can put anything you want in a buffer, there is no restriction. You can for example write
this:

```rust
struct MyStruct {
    a: u32,
    b: bool,
}

let data = MyStruct { a: 5, b: true };

let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                            Some(queue.family()), data).unwrap();
```

The most common usage for buffers however is to create a buffer whose content is an unsized array.

```rust
let iter = (0 .. 128).map(|_| 5);
let buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                            Some(queue.family()), iter).unwrap();
```

*To be finished*

## Reading and writing the content of the buffer

Once the buffer is created, you can access its content with the `read()` or `write()` methods.
Using `read()` will grant you shared access to the content of the buffer, and `write()` will grant
you exclusive access. This is similar to using a `RwLock`.

For example if `buffer` contains a `MyStruct` (see above):

```rust
let mut content = buffer.write().unwrap();
// `content` implements `DerefMut` whose target is of type `MyStruct` (the content of the buffer)
content.a *= 2;
content.b = false;
```

Alternatively, supposing that the content of `buffer` is of type `[u8]`:

```rust
let mut content = buffer.write().unwrap();
// this time `content` derefs to `[u8]`
content[12] = 83;
content[7] = 3;
```

Keep in mind that being able to read/write the content of the buffer like this is specific to the
`CpuAccessibleBuffer`. Other kinds of buffers (for example the `ImmutableBuffer`) do not provide
such methods.
