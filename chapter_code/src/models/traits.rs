use vulkano::memory::Content;

pub trait Model<V, U: Content + Copy + 'static> {
  fn get_indices() -> Vec<u16>;
  fn get_vertices() -> Vec<V>;
  fn get_initial_uniform_data() -> U;
}