pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/movable_square/vertex.glsl",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/movable_square/fragment.glsl",
    }
}
