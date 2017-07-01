# Dispatch

Now that we have all the needed ingredients, we can create the command buffer that will execute
our compute pipeline.

This is similar to [the example operation in a previous guide](/guide/example-operation).

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .dispatch([1024, 1, 1], pipeline.clone(), set.clone(), ()).unwrap()
        .build().unwrap();

> **Note**: The last parameter contains the *push constants*, which we haven't covered yet.

*To be written*
