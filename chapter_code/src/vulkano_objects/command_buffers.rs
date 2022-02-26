use std::sync::Arc;

use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::buffer::TypedBufferAccess;
use vulkano::command_buffer::{
  AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
};
use vulkano::device::{Device, Queue};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Framebuffer;

use crate::Vertex2d;

pub fn create_only_vertex_command_buffers(
  device: Arc<Device>,
  queue: Arc<Queue>,
  pipeline: Arc<GraphicsPipeline>,
  framebuffers: &Vec<Arc<Framebuffer>>,
  vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex2d]>>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
  framebuffers
    .iter()
    .map(|framebuffer| {
      let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::MultipleSubmit,
      )
      .unwrap();

      builder
        .begin_render_pass(
          framebuffer.clone(),
          SubpassContents::Inline,
          vec![[0.1, 0.1, 0.1, 1.0].into()],
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
