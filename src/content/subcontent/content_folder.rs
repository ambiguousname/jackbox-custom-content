use super::Subcontent;

/// A folder in the game directory that contains many [`super::subfolder::Subfolder`]s.
pub struct ContentFolder {

}

impl Subcontent for ContentFolder {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn write_to_mod(&self, id: String) {
		todo!()
	}

	fn write_to_game(&self, id: String) {
		todo!()
	}

	fn load_from_dir(&self) {
		todo!()
	}
}