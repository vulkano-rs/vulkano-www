use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::Device;

use crate::Vertex2d;

pub type Vertex2dBuffer = CpuAccessibleBuffer<[Vertex2d]>;

pub fn create_static_triangle_vertex_buffer(device: Arc<Device>) -> Arc<Vertex2dBuffer> {
  let vertex1 = Vertex2d {
    position: [-0.5, -0.5],
  };
  let vertex2 = Vertex2d {
    position: [0.0, 0.5],
  };
  let vertex3 = Vertex2d {
    position: [0.5, -0.25],
  };
  CpuAccessibleBuffer::from_iter(
    device,
    BufferUsage::all(),
    false,
    vec![vertex1, vertex2, vertex3].into_iter(),
  )
  .unwrap()
}
