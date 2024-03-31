use super::content_data::ContentData;

impl ContentData {
	pub fn serialize(&self) -> serde_json::Value {
		let mut map = serde_json::Map::new();


		serde_json::Value::Object(map)
	}
}