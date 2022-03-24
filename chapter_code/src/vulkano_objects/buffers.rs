use bytemuck::Pod;
use std::sync::Arc;
use vulkano::buffer::{
  BufferContents, BufferUsage, CpuAccessibleBuffer, ImmutableBuffer, TypedBufferAccess,
};
use vulkano::command_buffer::{CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{
  DescriptorSetsCollection, PersistentDescriptorSet, WriteDescriptorSet,
};
use vulkano::device::{Device, Queue};
use vulkano::pipeline::graphics::vertex_input::VertexBuffersCollection;
use vulkano::sync::{GpuFuture, NowFuture};

use crate::models::Model;

pub type Uniform<U> = (Arc<CpuAccessibleBuffer<U>>, Arc<PersistentDescriptorSet>);

// This trait will apply to all structs that contain vertex, index and uniform buffers
pub trait Buffers<Vb, Ib, D>
where
  Vb: VertexBuffersCollection,                      // Vertex buffer
  Ib: TypedBufferAccess<Content = [u16]> + 'static, // Index buffer
  D: DescriptorSetsCollection,
{
  fn get_vertex(&self) -> Vb;

  // Vb and D have their own collection, so they are implicitly wrapped in an Arc, but Ib should be wrapped explicitly
  fn get_index(&self) -> Arc<Ib>;
  fn get_uniform_descriptor_set(&self, i: usize) -> D;
}

// Struct with a cpu accessible vertex, index and uniform buffer, with generic (V)ertices and (U)niforms
pub struct SimpleBuffers<V: BufferContents + Pod, U: BufferContents> {
  pub vertex: Arc<CpuAccessibleBuffer<[V]>>,
  pub index: Arc<CpuAccessibleBuffer<[u16]>>,
  pub uniforms: Vec<Uniform<U>>,
}

impl<V: BufferContents + Pod, U: BufferContents + Copy> SimpleBuffers<V, U> {
  pub fn initialize<M: Model<V, U>>(
    device: Arc<Device>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    uniform_buffer_count: usize,
  ) -> Self {
    Self {
      vertex: create_cpu_accessible_vertex::<V, U, M>(device.clone()),
      index: create_cpu_accessible_index::<V, U, M>(device.clone()),
      uniforms: create_cpu_accessible_uniforms::<V, U, M>(
        device,
        descriptor_set_layout,
        uniform_buffer_count,
      ),
    }
  }
}

impl<'a, V, U>
  Buffers<Arc<CpuAccessibleBuffer<[V]>>, CpuAccessibleBuffer<[u16]>, Arc<PersistentDescriptorSet>>
  for SimpleBuffers<V, U>
where
  V: BufferContents + Pod,
  U: BufferContents,
{
  fn get_vertex(&self) -> Arc<CpuAccessibleBuffer<[V]>> {
    self.vertex.clone()
  }

  fn get_index(&self) -> Arc<CpuAccessibleBuffer<[u16]>> {
    self.index.clone()
  }

  fn get_uniform_descriptor_set(&self, i: usize) -> Arc<PersistentDescriptorSet> {
    self.uniforms[i].1.clone()
  }
}

// Struct with immutable vertex and index buffer and a cpu accessible uniform buffer, with generic (V)ertices and (U)niforms
pub struct ImmutableBuffers<V: BufferContents + Pod, U: BufferContents> {
  pub vertex: Arc<ImmutableBuffer<[V]>>,
  pub index: Arc<ImmutableBuffer<[u16]>>,
  pub uniforms: Vec<Uniform<U>>,
}

impl<V: BufferContents + Pod, U: BufferContents + Copy> ImmutableBuffers<V, U> {
  pub fn initialize<M: Model<V, U>>(
    device: Arc<Device>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    uniform_buffer_count: usize,
    transfer_queue: Arc<Queue>,
  ) -> Self {
    let (vertex, vertex_future) = create_immutable_vertex::<V, U, M>(transfer_queue.clone());
    let (index, index_future) = create_immutable_index::<V, U, M>(transfer_queue);

    let fence = vertex_future
      .join(index_future)
      .then_signal_fence_and_flush()
      .unwrap();

    fence.wait(None).unwrap();

    Self {
      vertex,
      index,
      uniforms: create_cpu_accessible_uniforms::<V, U, M>(
        device,
        descriptor_set_layout,
        uniform_buffer_count,
      ),
    }
  }
}

impl<'a, V, U>
  Buffers<Arc<ImmutableBuffer<[V]>>, ImmutableBuffer<[u16]>, Arc<PersistentDescriptorSet>>
  for ImmutableBuffers<V, U>
where
  V: BufferContents + Pod,
  U: BufferContents,
{
  fn get_vertex(&self) -> Arc<ImmutableBuffer<[V]>> {
    self.vertex.clone()
  }

  fn get_index(&self) -> Arc<ImmutableBuffer<[u16]>> {
    self.index.clone()
  }

  fn get_uniform_descriptor_set(&self, i: usize) -> Arc<PersistentDescriptorSet> {
    self.uniforms[i].1.clone()
  }
}

fn create_cpu_accessible_vertex<V, U, M>(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[V]>>
where
  V: BufferContents + Pod,
  U: BufferContents,
  M: Model<V, U>,
{
  CpuAccessibleBuffer::from_iter(
    device,
    BufferUsage::vertex_buffer(),
    false,
    M::get_vertices().into_iter(),
  )
  .unwrap()
}

fn create_immutable_vertex<V, U, M>(
  queue: Arc<Queue>,
) -> (
  Arc<ImmutableBuffer<[V]>>,
  CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>,
)
where
  V: BufferContents + Pod,
  U: BufferContents,
  M: Model<V, U>,
{
  ImmutableBuffer::from_iter(
    M::get_vertices().into_iter(),
    BufferUsage::vertex_buffer(),
    queue,
  )
  .unwrap()
}

fn create_cpu_accessible_index<V, U, M>(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[u16]>>
where
  V: BufferContents,
  U: BufferContents,
  M: Model<V, U>,
{
  CpuAccessibleBuffer::from_iter(
    device,
    BufferUsage::index_buffer(),
    false,
    M::get_indices().into_iter(),
  )
  .unwrap()
}

fn create_immutable_index<V, U, M>(
  queue: Arc<Queue>,
) -> (
  Arc<ImmutableBuffer<[u16]>>,
  CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>,
)
where
  V: BufferContents,
  U: BufferContents,
  M: Model<V, U>,
{
  ImmutableBuffer::from_iter(
    M::get_indices().into_iter(),
    BufferUsage::index_buffer(),
    queue,
  )
  .unwrap()
}

fn create_cpu_accessible_uniforms<V, U, M>(
  device: Arc<Device>,
  descriptor_set_layout: Arc<DescriptorSetLayout>,
  buffer_count: usize,
) -> Vec<Uniform<U>>
where
  V: BufferContents,
  U: BufferContents + Copy,
  M: Model<V, U>,
{
  (0..buffer_count)
    .map(|_| {
      let buffer = CpuAccessibleBuffer::from_data(
        device.clone(),
        BufferUsage::uniform_buffer(),
        false,
        M::get_initial_uniform_data(),
      )
      .unwrap();

      let descriptor_set = PersistentDescriptorSet::new(
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, buffer.clone())],
      )
      .unwrap();

      (buffer, descriptor_set)
    })
    .collect()
}
