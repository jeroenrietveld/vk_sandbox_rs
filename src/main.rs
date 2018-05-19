extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

pub mod core;
pub mod render;

// use render::renderer;
use core::engine_settings;

use vulkano::instance;
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::instance::{PhysicalDevice, PhysicalDeviceType};
use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;

fn main() {
    render::print_layer_list();

    let settings = engine_settings::EngineSettings::default()
        .with_title("Vulkan Window")
        .with_dimensions(1600, 900);

    let instance = create_instance();

    let _debug_callback = enable_debug(instance.clone());

    let physical_device =
        get_physical_device(&instance).expect("Unable to get Physical Device");
    println!("Device Chosen: {:?}", physical_device.name());

    let queue_family = get_queue_family(&physical_device).expect("Unable to get Graphics Queue Family");

    // let renderer = renderer::Renderer::new(&settings);

    let mut events_loop = winit::EventsLoop::new();
    let surface = winit::WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .expect("Unable to create surface");
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

fn get_physical_device<'a>(instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
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


fn get_queue_family<'a>(physical_device: &'a PhysicalDevice) -> Option<instance::QueueFamily<'a>> {
	let queue_families = physical_device.queue_families();

	let mut found_graphics_family = false;
	let mut queue_index = 0;
	println!("Queue Families available: ");

	for (i, queue_family) in queue_families.enumerate() {
		println!("\tQueue Family Index: {:?}", i);
		println!("\t\tID{:?}", queue_family.id());
		println!("\t\tQueue Count: {:?}", queue_family.queues_count());
		println!("\t\tSupports Graphics: {:?}", queue_family.supports_graphics());
		println!("\t\tSupports Compute: {:?}", queue_family.supports_compute());
		println!("\t\tSupports Transfers: {:?}", queue_family.supports_transfers());
		println!("\t\tSupports Sparse Biding: {:?}", queue_family.supports_sparse_binding());
		if queue_family.supports_graphics() {
			found_graphics_family = true;
			queue_index = queue_family.id();
		}
	}

	if found_graphics_family {		
		return physical_device.queue_family_by_id(queue_index as u32)
	} else {
		return None
	}

}