use super::Subcontent;

/// A manifest.jet file that lists our content.
pub struct Manifest {
	name : String,
}

impl Manifest {
	pub fn new(name : Option<String>) -> Self {
		Manifest {name: name.unwrap_or("manifest.jet".to_string())}
	}
}

impl Subcontent for Manifest {
	fn write_to_game(&self) {
		todo!()
	}

	fn write_to_mod(&self) {
		todo!()
	}
	
	fn load_from_dir(&self) {
			todo!()
		}
}