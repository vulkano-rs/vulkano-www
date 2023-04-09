use std::sync::Arc;

use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, CopyBufferInfo,
    PrimaryCommandBufferAbstract,
};
use vulkano::descriptor_set::layout::DescriptorSetLayout;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::device::Queue;
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryUsage};
use vulkano::sync::future::NowFuture;
use vulkano::sync::GpuFuture;
use vulkano::DeviceSize;

use super::allocators::Allocators;
use crate::models::Model;

pub type Uniform<U> = (Subbuffer<U>, Arc<PersistentDescriptorSet>);

/// Struct with a vertex, index and uniform buffer, with generic (V)ertices and (U)niforms.
pub struct Buffers<V: BufferContents, U: BufferContents> {
    pub vertex: Subbuffer<[V]>,
    pub index: Subbuffer<[u16]>,
    pub uniforms: Vec<Uniform<U>>,
}

impl<V: BufferContents, U: BufferContents> Buffers<V, U> {
    pub fn initialize_host_accessible<M: Model<V, U>>(
        allocators: &Allocators,
        descriptor_set_layout: Arc<DescriptorSetLayout>,
        uniform_buffer_count: usize,
    ) -> Self {
        Self {
            vertex: create_cpu_accessible_vertex::<V, U, M>(allocators),
            index: create_cpu_accessible_index::<V, U, M>(allocators),
            uniforms: create_cpu_accessible_uniforms::<V, U, M>(
                allocators,
                descriptor_set_layout,
                uniform_buffer_count,
            ),
        }
    }

    pub fn initialize_device_local<M: Model<V, U>>(
        allocators: &Allocators,
        descriptor_set_layout: Arc<DescriptorSetLayout>,
        uniform_buffer_count: usize,
        transfer_queue: Arc<Queue>,
    ) -> Self {
        let (vertex, vertex_future) =
            create_device_local_vertex::<V, U, M>(allocators, transfer_queue.clone());
        let (index, index_future) =
            create_device_local_index::<V, U, M>(allocators, transfer_queue);

        let fence = vertex_future
            .join(index_future)
            .then_signal_fence_and_flush()
            .unwrap();

        fence.wait(None).unwrap();

        Self {
            vertex,
            index,
            uniforms: create_cpu_accessible_uniforms::<V, U, M>(
                allocators,
                descriptor_set_layout,
                uniform_buffer_count,
            ),
        }
    }

    pub fn get_vertex(&self) -> Subbuffer<[V]> {
        self.vertex.clone()
    }

    pub fn get_index(&self) -> Subbuffer<[u16]> {
        self.index.clone()
    }

    pub fn get_uniform_descriptor_set(&self, i: usize) -> Arc<PersistentDescriptorSet> {
        self.uniforms[i].1.clone()
    }
}

fn create_cpu_accessible_vertex<V, U, M>(allocators: &Allocators) -> Subbuffer<[V]>
where
    V: BufferContents,
    U: BufferContents,
    M: Model<V, U>,
{
    Buffer::from_iter(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        M::get_vertices(),
    )
    .unwrap()
}

fn create_device_local_vertex<V, U, M>(
    allocators: &Allocators,
    queue: Arc<Queue>,
) -> (Subbuffer<[V]>, CommandBufferExecFuture<NowFuture>)
where
    V: BufferContents,
    U: BufferContents,
    M: Model<V, U>,
{
    let vertices = M::get_vertices();

    let buffer = Buffer::new_slice(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER | BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::DeviceOnly,
            ..Default::default()
        },
        vertices.len() as DeviceSize,
    )
    .unwrap();

    let staging_buffer = Buffer::from_iter(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        vertices,
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        &allocators.command_buffer,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .copy_buffer(CopyBufferInfo::buffers(staging_buffer, buffer.clone()))
        .unwrap();

    let future = builder.build().unwrap().execute(queue).unwrap();

    (buffer, future)
}

fn create_cpu_accessible_index<V, U, M>(allocators: &Allocators) -> Subbuffer<[u16]>
where
    V: BufferContents,
    U: BufferContents,
    M: Model<V, U>,
{
    Buffer::from_iter(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        M::get_indices(),
    )
    .unwrap()
}

fn create_device_local_index<V, U, M>(
    allocators: &Allocators,
    queue: Arc<Queue>,
) -> (Subbuffer<[u16]>, CommandBufferExecFuture<NowFuture>)
where
    V: BufferContents,
    U: BufferContents,
    M: Model<V, U>,
{
    let indices = M::get_indices();

    let buffer = Buffer::new_slice(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::INDEX_BUFFER | BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::DeviceOnly,
            ..Default::default()
        },
        indices.len() as DeviceSize,
    )
    .unwrap();

    let staging_buffer = Buffer::from_iter(
        &allocators.memory,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        indices,
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        &allocators.command_buffer,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    builder
        .copy_buffer(CopyBufferInfo::buffers(staging_buffer, buffer.clone()))
        .unwrap();

    let future = builder.build().unwrap().execute(queue).unwrap();

    (buffer, future)
}

fn create_cpu_accessible_uniforms<V, U, M>(
    allocators: &Allocators,
    descriptor_set_layout: Arc<DescriptorSetLayout>,
    buffer_count: usize,
) -> Vec<Uniform<U>>
where
    V: BufferContents,
    U: BufferContents,
    M: Model<V, U>,
{
    (0..buffer_count)
        .map(|_| {
            let buffer = Buffer::from_data(
                &allocators.memory,
                BufferCreateInfo {
                    usage: BufferUsage::INDEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                M::get_initial_uniform_data(),
            )
            .unwrap();

            let descriptor_set = PersistentDescriptorSet::new(
                &allocators.descriptor_set,
                descriptor_set_layout.clone(),
                [WriteDescriptorSet::buffer(0, buffer.clone())],
            )
            .unwrap();

            (buffer, descriptor_set)
        })
        .collect()
}
