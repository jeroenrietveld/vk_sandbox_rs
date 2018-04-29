pub mod renderer;

use vulkano::instance;

pub fn print_layer_list(){

	for layer in instance::layers_list().unwrap() {
		println!("Available Layer: {}", layer.name());
	}
}

