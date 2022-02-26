use std::sync::Arc;
use vulkano::instance::Instance;
use vulkano::Version;

const LIST_AVAILABLE_LAYERS: bool = false;
const ENABLE_VALIDATION_LAYERS: bool = false;
const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_api_dump"];

pub fn get_instance() -> Arc<Instance> {
  let required_extensions = vulkano_win::required_extensions();

  if LIST_AVAILABLE_LAYERS {
    let layers: Vec<_> = vulkano::instance::layers_list().unwrap().collect();
    let layer_names = layers.iter().map(|l| l.name());
    println!(
      "Using layers {:?}",
      layer_names.clone().collect::<Vec<&str>>()
    );
  }

  if ENABLE_VALIDATION_LAYERS {
    Instance::new(
      None,
      Version::V1_1,
      &required_extensions,
      VALIDATION_LAYERS.iter().cloned(),
    )
    .unwrap()
  } else {
    Instance::new(None, Version::V1_1, &required_extensions, None).unwrap()
  }
}