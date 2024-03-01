use std::path::PathBuf;

use super::Subcontent;

/// Represents a sub-folder 
pub struct Subfolder {
	/// The location that this subfolder should go into.
	sublocation : PathBuf,
}

impl Subfolder {
	fn new() -> Self {
		Subfolder {sublocation: PathBuf::new()}
	}
}

impl Subcontent for Subfolder {
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