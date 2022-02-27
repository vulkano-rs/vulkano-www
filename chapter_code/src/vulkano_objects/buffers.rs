use std::sync::Arc;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::Device;
use vulkano::memory::Content;

use crate::models::Model;
use crate::Vertex2d;

pub type Uniform<U> = (Arc<CpuAccessibleBuffer<U>>, Arc<PersistentDescriptorSet>);

pub struct SimpleBuffers<U: Content + Copy + Send + Sync + 'static> {
  pub vertex: Arc<CpuAccessibleBuffer<[Vertex2d]>>,
  pub index: Arc<CpuAccessibleBuffer<[u16]>>,
  pub uniforms: Vec<Uniform<U>>,
}

impl<U: Content + Copy + Send + Sync + 'static> SimpleBuffers<U> {
  pub fn initialize<M: Model<Vertex2d, U>>(
    device: Arc<Device>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    uniform_buffer_count: usize,
  ) -> Self {
    Self {
      vertex: Self::create_vertex::<M>(device.clone()),
      index: Self::create_index::<M>(device.clone()),
      uniforms: Self::create_uniforms::<M>(device, descriptor_set_layout, uniform_buffer_count),
    }
  }

  fn create_vertex<M: Model<Vertex2d, U>>(
    device: Arc<Device>,
  ) -> Arc<CpuAccessibleBuffer<[Vertex2d]>> {
    CpuAccessibleBuffer::from_iter(
      device,
      BufferUsage::vertex_buffer(),
      false,
      M::get_vertices().into_iter(),
    )
    .unwrap()
  }

  fn create_index<M: Model<Vertex2d, U>>(device: Arc<Device>) -> Arc<CpuAccessibleBuffer<[u16]>> {
    CpuAccessibleBuffer::from_iter(
      device,
      BufferUsage::index_buffer(),
      false,
      M::get_indices().into_iter(),
    )
    .unwrap()
  }

  fn create_uniforms<M: Model<Vertex2d, U>>(
    device: Arc<Device>,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    buffer_count: usize,
  ) -> Vec<Uniform<U>> {
    let mut uniforms = Vec::with_capacity(buffer_count);
    for _ in 0..buffer_count {
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

      uniforms.push((buffer, descriptor_set));
    }
    uniforms
  }
}
