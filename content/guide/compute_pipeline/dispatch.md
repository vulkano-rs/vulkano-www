# Dispatch

Now that we have all the needed ingredients, we can create the command buffer that will execute
our compute pipeline. This is called a *dispatch* operation.

Creating a command buffer is similar to [the example operation in a previous
section](/guide/example-operation).

```rust
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::pipeline::PipelineBindPoint;

let command_buffer_allocator = StandardCommandBufferAllocator::new(
    device.clone(),
    StandardCommandBufferAllocatorCreateInfo::default(),
);

let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
    &command_buffer_allocator,
    queue.queue_family_index(),
    CommandBufferUsage::OneTimeSubmit,
)
.unwrap();

let work_group_counts = [1024, 1, 1];

command_buffer_builder
    .bind_pipeline_compute(compute_pipeline.clone())
    .bind_descriptor_sets(
        PipelineBindPoint::Compute,
        compute_pipeline.layout().clone(),
        descriptor_set_layout_index as u32,
        descriptor_set,
    )
    .dispatch(work_group_counts)
    .unwrap();

let command_buffer = command_buffer_builder.build().unwrap();
```

First, we bind the pipeline and then the *descriptor set*s, indicating the type of set, the layout
and the descriptor sets we are going to use. Here the number of sets could have actually been many, 
in which case we would indicate our desired one with an index. Because we only have one, the index 
is 0.

As explained in [the compute pipeline section](/guide/compute-pipeline), we want to spawn 1024
*work groups*. This value is indicated by the actual `.dispatch()` method.

Just like we already covered, we submit the command buffer:

```rust
let future = sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush()
    .unwrap();
```

This just schedules the operation for execution and tells the GPU to signal when finished.
We have to wait for it to complete:

```rust
future.wait(None).unwrap();
```

Once complete, we can check that the pipeline has been correctly executed:

```rust
let content = data_buffer.read().unwrap();
for (n, val) in content.iter().enumerate() {
    assert_eq!(*val, n as u32 * 12);
}

println!("Everything succeeded!");
```

Next: [Creating an image](/guide/image-creation)
