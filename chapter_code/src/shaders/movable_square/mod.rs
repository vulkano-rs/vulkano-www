pub mod vs {
  vulkano_shaders::shader! {
      ty: "vertex",
      path: "src/shaders/movable_square/vertex.glsl",
      types_meta: {
        use bytemuck::{Pod, Zeroable};
        #[derive(Clone, Copy, Zeroable, Pod)]
    },
  }
}

pub mod fs {
  vulkano_shaders::shader! {
      ty: "fragment",
      path: "src/shaders/movable_square/fragment.glsl"
  }
}
