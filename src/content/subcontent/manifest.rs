use std::{fs::{self, File}, path::Path};

use super::Subcontent;

/// A manifest.jet file that lists our content.
#[derive(Debug)]
pub struct ManifestItem {
	/// A serde_json value of content to write.
	item_content: serde_json::Value,
}

// TODO: Write loading.

impl ManifestItem {
	pub fn new(item_content : serde_json::Value) -> Self {
		ManifestItem {item_content: item_content}
	}

	pub fn content(&self) -> serde_json::Value {
		self.item_content.clone()
	}
}

impl Subcontent for ManifestItem {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn write_to_game(&self) {
		todo!()
	}

	fn write_to_mod(&self, id: String, args : Vec<&'static str>) -> std::io::Result<()> {
		let file_to_write = args[0];
		let file_path = Path::new(file_to_write);

		let manifest_read = File::open(file_path)?;
		// Merge the manifest with what we have.
		

		let manifest_write = File::create(file_path)?;
		Ok(())
	}
	
	fn load_from_dir(&self) {
			todo!()
	}
}