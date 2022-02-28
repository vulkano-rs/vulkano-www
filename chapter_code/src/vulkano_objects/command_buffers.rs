use std::sync::Arc;

use vulkano::buffer::{CpuAccessibleBuffer, TypedBufferAccess};
use vulkano::command_buffer::{
  AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
};
use vulkano::descriptor_set::DescriptorSetsCollection;
use vulkano::device::{Device, Queue};
use vulkano::pipeline::graphics::vertex_input::VertexBuffersCollection;
use vulkano::pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint};
use vulkano::render_pass::Framebuffer;

use crate::vulkano_objects::buffers::Buffers;
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

pub fn create_simple_command_buffers<
  Vb: VertexBuffersCollection,
  Ib: TypedBufferAccess<Content = [u16]> + 'static,
  D: DescriptorSetsCollection,
>(
  device: Arc<Device>,
  queue: Arc<Queue>,
  pipeline: Arc<GraphicsPipeline>,
  framebuffers: &Vec<Arc<Framebuffer>>,
  buffers: &dyn Buffers<Vb, Ib, D>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
  framebuffers
    .iter()
    .enumerate()
    .map(|(i, framebuffer)| {
      let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue.family(),
        CommandBufferUsage::MultipleSubmit,
      )
      .unwrap();

      let index_buffer = buffers.get_index();
      let index_buffer_length = index_buffer.len();

      builder
        .begin_render_pass(
          framebuffer.clone(),
          SubpassContents::Inline,
          vec![[0.1, 0.1, 0.1, 1.0].into()],
        )
        .unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .bind_descriptor_sets(
          PipelineBindPoint::Graphics,
          pipeline.layout().clone(),
          0,
          buffers.get_uniform_descriptor_set(i),
        )
        .bind_vertex_buffers(0, buffers.get_vertex())
        .bind_index_buffer(index_buffer)
        .draw_indexed(index_buffer_length as u32, 1, 0, 0, 0)
        .unwrap()
        .end_render_pass()
        .unwrap();

      Arc::new(builder.build().unwrap())
    })
    .collect()
}
