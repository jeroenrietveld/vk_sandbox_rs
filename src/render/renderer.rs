use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::PhysicalDeviceType;

use std::sync::Arc;

use core::engine_settings::EngineSettings;

pub struct Renderer<'a> {
    settings: &'a EngineSettings,
    instance: Arc<Instance>,
    physical_device: PhysicalDevice<'a>
}

impl<'a> Renderer<'a> {
    pub fn new(settings: &EngineSettings) -> Renderer {
        let instance = create_instance();
        let _debug_callback = enable_debug(instance.clone());
        let local_devices = PhysicalDevice::enumerate(&instance);

    	println!("Available Devices: ");
    	let mut device_index = 0;

    	for (i, device) in local_devices.enumerate() {
        	println!(
            	"\tIndex {}: Name: {}, Type: {:?}",
            	i,
            	device.name(),
            	device.ty()
        );

        match device.ty() {
            PhysicalDeviceType::DiscreteGpu => {
                device_index = i;                
            	}
            _ => (),
        	}
    	}

    // let physical_device = local_devices.next().expect("No Device Available");
    // println!("Device Chosen: {:?}", physical_device.name());
    let physical_device = PhysicalDevice::from_index(&instance, device_index).expect("Unable to create phsyical device");

        println!("Using device: {:?}", physical_device.name());
        

        Renderer { 
        	settings,
        	physical_device,
        	instance,
        	 }
    }
}

fn create_instance() -> Arc<Instance> {
    let extensions = InstanceExtensions {
        khr_surface: true,
        khr_xlib_surface: true,
        ext_debug_report: true,
        ..InstanceExtensions::none()
    };

    let layer = "VK_LAYER_LUNARG_standard_validation";
    let layers = vec![&layer];

    let instance_try = Instance::new(None, &extensions, layers);

    let instance = match instance_try {
        Ok(i) => i,
        Err(err) => panic!("Couldn't build Instance {:?}", err),
    };

    instance
}

fn create_device<'a>(instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    let local_devices = PhysicalDevice::enumerate(&instance);

    println!("Available Devices: ");
    let mut device_index = 0;

    for (i, device) in local_devices.enumerate() {
        println!(
            "\tIndex {}: Name: {}, Type: {:?}",
            i,
            device.name(),
            device.ty()
        );

        match device.ty() {
            PhysicalDeviceType::DiscreteGpu => {
                device_index = i;
                
            }
            _ => (),
        }
    }

    // let physical_device = local_devices.next().expect("No Device Available");
    // println!("Device Chosen: {:?}", physical_device.name());
    PhysicalDevice::from_index(&instance, device_index)
}

fn enable_debug(instance: Arc<Instance>) -> DebugCallback {
    let all = MessageTypes {
        error: true,
        warning: true,
        performance_warning: true,
        information: true,
        debug: true,
    };

    let debug_callback = DebugCallback::new(&instance, all, |msg| {
        let ty = if msg.ty.error {
            "error"
        } else if msg.ty.warning {
            "warning"
        } else if msg.ty.performance_warning {
            "performance_warning"
        } else if msg.ty.information {
            "information"
        } else if msg.ty.debug {
            "debug"
        } else {
            panic!("no-impl");
        };
        println!("{} {}: {}", msg.layer_prefix, ty, msg.description);
    }).ok();
    debug_callback.unwrap()
}
