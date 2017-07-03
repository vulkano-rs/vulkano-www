# Dispatch

Now that we have all the needed ingredients, we can create the command buffer that will execute
our compute pipeline. This is called a *dispatch* operation.

Creating a command buffer is similar to [the example operation in a previous
section](/guide/example-operation).

```rust
let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
    .dispatch([1024, 1, 1], pipeline.clone(), set.clone(), ()).unwrap()
    .build().unwrap();
```

*To be finished ; explain 1024, 1, 1*

> **Note**: The last parameter contains the *push constants*, which we haven't covered yet.
> Push constants are a way to pass a small amount of data to a shader, as an alternative to
> putting this data in a buffer in a descriptor set.

Just like we already covered, we submit the command buffer:

```rust
let finished = command_buffer.execute(queue.clone()).unwrap();
```

This just schedules the operation for execution. We have to wait for it to be complete:

```rust
finished.then_signal_fence_and_flush().unwrap()
    .wait().unwrap();
```

Once complete we can check that the pipeline has been correctly executed:

```rust
let content = data_buffer.read().unwrap();
for (n, val) in content.iter().enumerate() {
    assert_eq!(*val, n as u32 * 12);
}
```
