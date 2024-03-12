use std::path::PathBuf;

use super::Subcontent;

/// Represents a sub-folder of a game's content folder for content to be in.
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
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn write_to_game(&self) {
		todo!()
	}

	fn write_to_mod(&self, id: String) {
		todo!()
	}

	fn load_from_dir(&self) {
		todo!()
	}
}