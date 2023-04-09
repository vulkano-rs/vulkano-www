use std::sync::Arc;

use vulkano::instance::{Instance, InstanceCreateInfo, LayerProperties};

const LIST_AVAILABLE_LAYERS: bool = false;
const ENABLE_VALIDATION_LAYERS: bool = false;
const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_api_dump"];

pub fn get_instance() -> Arc<Instance> {
    let library = vulkano::VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let required_extensions = vulkano_win::required_extensions(&library);

    if LIST_AVAILABLE_LAYERS {
        let layers: Vec<_> = library.layer_properties().unwrap().collect();
        let layer_names = layers.iter().map(LayerProperties::name);
        println!(
            "Available layers:\n {:?}",
            layer_names.clone().collect::<Vec<&str>>()
        );
    }

    let mut create_info = InstanceCreateInfo {
        enabled_extensions: required_extensions,
        ..Default::default()
    };

    if ENABLE_VALIDATION_LAYERS {
        create_info.enabled_layers = VALIDATION_LAYERS.iter().map(|s| s.to_string()).collect();
    }

    Instance::new(library, create_info).unwrap()
}
