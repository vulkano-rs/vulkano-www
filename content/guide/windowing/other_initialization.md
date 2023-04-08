# Other initialization

Now that we have a swapchain to work with, let's add all the missing Vulkan objects (same as in the 
previous chapter example) and modify them as needed. Let's also, for clarity, move some of them to 
separate functions.

In the render pass, let's configure it to always use the same format as the swapchain, to avoid any 
invalid format errors:

```rust
use vulkano::render_pass::RenderPass;

fn get_render_pass(device: Arc<Device>, swapchain: &Arc<Swapchain<Window>>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device,
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(), // set the format the same as the swapchain
                samples: 1,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    )
    .unwrap()
}

// main()
let render_pass = get_render_pass(device.clone(), &swapchain);
```

When we only had one image, we only needed to create one framebuffer for it. However, we now need 
to create a different framebuffer for each of the images:

```rust
use vulkano::image::view::ImageView;
use vulkano::image::SwapchainImage;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};

fn get_framebuffers(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: &Arc<RenderPass>,
) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

// main()
let framebuffers = get_framebuffers(&images, &render_pass);
```

We don't need to modify anything in the shaders and the vertex buffer (we are using the same 
triangle), so let's just leave everything as it is, only changing the structure a bit:

```rust
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage};
use vulkano::memory::allocator::AllocationCreateInfo;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 460

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 460

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
    }
}
```

```rust
fn main() {
    // crop

    let vertex1 = MyVertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = MyVertex {
        position: [0.0, 0.5],
    };
    let vertex3 = MyVertex {
        position: [0.5, -0.25],
    };
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

    let vs = vs::load(device.clone()).expect("failed to create shader module");
    let fs = fs::load(device.clone()).expect("failed to create shader module");
    
    // crop
}
```

As for the pipeline, let's initialize the viewport with our window dimensions:

```rust
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Subpass;
use vulkano::shader::ShaderModule;

fn get_pipeline(
    device: Arc<Device>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass, 0).unwrap())
        .build(device)
        .unwrap()
}

fn main() {
    // crop

    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: surface.window().inner_size().into(),
        depth_range: 0.0..1.0,
    };

    let pipeline = get_pipeline(
        device.clone(),
        vs.clone(),
        fs.clone(),
        render_pass.clone(),
        viewport.clone(),
    );

    // crop
}
```

Currently the viewport state is set to `fixed_scissor_irrelevant`, meaning that it will be only 
using one fixed viewport. Because of this, we will need to recreate the pipeline every time the 
window gets resized (the viewport changes). If you expect the window to be resized many times, you 
can set the pipeline viewport to a dynamic state, using
`ViewportState::viewport_dynamic_scissor_irrelevant()`, at a cost of a bit of performance.

Let's move now to the command buffers. In this example we are going to draw the same triangle over 
and over, so we can create a command buffer and call it multiple times. However, because we now 
also have multiple framebuffers, we will have multiple command buffers as well, one for each 
framebuffer. Let's put everything nicely into a function:

```rust
use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
    RenderPassBeginInfo,
};

use vulkano::device::Queue;

fn get_command_buffers(
    device: &Arc<Device>,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Arc<CpuAccessibleBuffer<[MyVertex]>>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .map(|framebuffer| {
            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit, // don't forget to write the correct buffer usage
            )
            .unwrap();

            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.1, 0.1, 0.1, 1.0].into())],
                        ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                    },
                    SubpassContents::Inline,
                )
                .unwrap()
                .bind_pipeline_graphics(pipeline.clone())
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .draw(vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap()
                .end_render_pass()
                .unwrap();

            Arc::new(builder.build().unwrap())
        })
        .collect()
}

// main()
let mut command_buffers = get_command_buffers(
    &device,
    &queue,
    &pipeline,
    &framebuffers,
    &vertex_buffer,
);
```

If you have set your pipeline to use a dynamic viewport, don't forget to then set the viewport in 
the command buffers, by using `.set_viewport(0, [viewport.clone()])`.

In the end, the structure of your `main` function should look something like this:

```rust
fn main() {
    // instance

    // surface

    // physical device
    // logical device
    // queue creation

    // swapchain

    // render pass
    // framebuffers
    // vertex buffer
    // shaders
    // viewport
    // pipeline
    // command buffers

    // event loop
}
```

If you feel lost in all the code, feel free to take a look at [full source code 
here](https://github.com/vulkano-rs/vulkano-www/blob/master/chapter_code/src/bin/windowing.rs).

The initialization is finally complete! Next, we will start working on the event loop and 
programming the functionality of each frame.

Next: [Event handling](/guide/windowing/event-handling)
