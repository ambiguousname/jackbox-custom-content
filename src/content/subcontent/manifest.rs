use super::Subcontent;

/// A manifest.jet file that lists our content.
/// TODO: Requires id for each item.
#[derive(Debug)]
pub struct ManifestItem {
	/// A serde_json value of content to write.
	item_content: serde_json::Value,
}

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

	fn write_to_mod(&self, id: String, args : Vec<&'static str>) {
		todo!()
	}
	
	fn load_from_dir(&self) {
			todo!()
		}
}