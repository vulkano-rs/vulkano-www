use crate::models::Model;
use crate::shaders::movable_square;
use crate::Vertex2d;

pub struct SquareModel;

type UniformData = movable_square::vs::Data;

impl Model<Vertex2d, UniformData> for SquareModel {
    fn get_vertices() -> Vec<Vertex2d> {
        vec![
            Vertex2d {
                position: [-0.25, -0.25],
            },
            Vertex2d {
                position: [0.25, -0.25],
            },
            Vertex2d {
                position: [-0.25, 0.25],
            },
            Vertex2d {
                position: [0.25, 0.25],
            },
        ]
    }

    fn get_indices() -> Vec<u16> {
        vec![0, 1, 2, 1, 2, 3]
    }

    fn get_initial_uniform_data() -> UniformData {
        UniformData {
            color: [0.0, 0.0, 0.0].into(),
            position: [0.0, 0.0],
        }
    }
}
