

pub struct EngineSettings {
	pub window_title: String,
	pub window_dimensions: [u32; 2],
}

impl EngineSettings {
	pub fn default() -> EngineSettings {
		EngineSettings {
			window_title: String::from("Default Window"),
			window_dimensions: [800, 600],
		}
	}

	pub fn with_title(mut self, title: &str) -> EngineSettings {
		self.window_title = String::from(title);
		self
	}

	pub fn with_dimensions(mut self, width: u32, height: u32) -> EngineSettings {
		self.window_dimensions = [width, height];
		self
	}

	pub fn get_dimensions(&self) -> [u32; 2] {
		self.window_dimensions.clone()
	}

}