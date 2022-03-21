use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct Vertex2d {
  pub position: [f32; 2],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct Vertex3d {
  pub position: [f32; 3],
}

vulkano::impl_vertex!(Vertex2d, position);
vulkano::impl_vertex!(Vertex3d, position);
