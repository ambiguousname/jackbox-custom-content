use super::Subcontent;

/// A folder in the game directory that contains many [`super::subfolder::Subfolder`]s.
pub struct ContentFolder {

}

impl Subcontent for ContentFolder {
	fn write_to_mod(&self) {
		todo!()
	}

	fn write_to_game(&self) {
		todo!()
	}

	fn load_from_dir(&self) {
		todo!()
	}
}