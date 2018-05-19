// pub mod renderer;

use vulkano::instance;

pub fn print_layer_list(){

	println!("List of Vulkan debugging layers available to use:");
    let mut layers = instance::layers_list().unwrap();
    while let Some(l) = layers.next() {
        println!("\t{}", l.name());
    }
}

