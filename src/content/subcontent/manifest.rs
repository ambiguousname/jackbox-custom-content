use super::Subcontent;

/// A manifest.jet file that lists our content.
pub struct Manifest {
	name : String,
}

impl Manifest {
	pub fn new(name : Option<String>) -> Self {
		Manifest {name: name.or(Some("manifest.jet".to_string())).unwrap()}
	}
}

impl Subcontent for Manifest {
	fn write_to_game(&self) {
		
	}

	fn write_to_mod(&self) {
		
	}
}