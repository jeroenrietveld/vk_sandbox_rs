use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

use std::sync::Arc;

use core::engine_settings::EngineSettings;


pub struct Renderer<'a> {
	settings: &'a EngineSettings,	
	instance: Arc<Instance>,
}

impl<'a> Renderer<'a> {
	pub fn new(settings:  &EngineSettings) -> Renderer {
		let instance = create_instance();
		Renderer{
			settings,
			instance
		}

	}

}


fn create_instance() -> Arc<Instance> {
	let extensions = InstanceExtensions{
			khr_surface: true,
			khr_xlib_surface: true,
			..InstanceExtensions::none()
		};

		let instance_try = Instance::new(None, &extensions, None);

		let instance = match instance_try {
			Ok(i) => i,
			Err(err) => panic!("Couldn't build Instance {:?}", err),
		};
		instance
}
