extern crate vulkano;

pub mod render;
pub mod core;

use render::renderer::Renderer;

use core::engine_settings::EngineSettings;


fn main() {
    let settings = EngineSettings::default()
    .with_title("Vulkan Window")
    .with_dimensions(1600, 900);

    let renderer = Renderer::new(&settings);

   

}