use vulkano::buffer::BufferContents;
use vulkano::pipeline::graphics::vertex_input::Vertex;

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex2d {
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct Vertex3d {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}
