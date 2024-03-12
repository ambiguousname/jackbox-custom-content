use super::Subcontent;

/// A manifest.jet file that lists our content.
/// TODO: Requires id for each item.
pub struct ManifestItem {
	/// A serde_json value of content to write.
	item_content: serde_json::Value,
}

impl ManifestItem {
	pub fn new(item_content : serde_json::Value) -> Self {
		ManifestItem {item_content: item_content}
	}
}

impl Subcontent for ManifestItem {
	fn write_to_game(&self, id: String) {
		todo!()
	}

	fn write_to_mod(&self, id: String) {
		todo!()
	}
	
	fn load_from_dir(&self) {
			todo!()
		}
}