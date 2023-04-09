pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/static_triangle/vertex.glsl",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/static_triangle/fragment.glsl",
    }
}
