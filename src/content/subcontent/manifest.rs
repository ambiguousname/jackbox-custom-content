use std::{fs::{self, File, OpenOptions}, io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Lines, Write}, path::Path};

use regex::Regex;

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

	fn create_manifest(&self, path : &Path) -> std::io::Result<()> {
		let mut manifest = File::create(path)?;

		// Compatibility with anything else that might want to read our manifest file.
		// Really we just want a comma separated list of values to read, so we use an array.
		// Then we can easily merge with other manifest.json files.
		manifest.write(b"[\n]")?;
		Ok(())
	}

	// region: Manifest Modifying

	fn write_values(&self, id : &str, writer : &mut BufWriter<File>) -> std::io::Result<()> {
		let mut base_value = serde_json::to_string(&self.item_content)?;
		// Get rid of the opening {
		base_value.remove(0);
		// Insert our ID:
		let out = format!("{{\"id\": \"{}\", {}", id, base_value);
		writer.write(out.as_bytes())?;
		writer.write(b",\n")?;
		Ok(())
	}

	/// Given a file ID, modify an existing manifest to include our ID.
	/// This does NOT create a new manifest. That should be done if the manifest does not exist.
	/// This should NOT be called for Jackbox .jet files, since this only supports modifying one item of a manifest,
	/// and assumes that each item has its own line.
	/// 
	/// AGAIN: This function assumes each item is on its own line.
	fn modify_manifest(&self, id : String, reader : BufReader<File>, writer : &mut BufWriter<File>) -> std::io::Result<()> {
		let id_regex = Regex::new(format!(r#""{}"\s*:"#, id).as_str()).unwrap();
		
		let mut line_iter = reader.lines();

		let mut written_new : bool = false;
		while let Some(l) = line_iter.next() {
			let line = l?;

			if line.ends_with("]") {
				if !written_new {
					self.write_values(&id, writer)?;
					written_new = true;
				}
			}

			// Assumes that per `modify_manifest`, there is one and only one item per one line.
			if id_regex.is_match(&line) {
				// Overwrite multiple IDs.
				// Don't expect this to happen, but you never know.
				if !written_new {
					self.write_values(&id, writer)?;
					written_new = true;
				}
			} else {
				writer.write(line.as_bytes())?;
				writer.write(b"\n")?;
			}
		}
		
		Ok(())
	}

	// endregion

}

impl Subcontent for ManifestItem {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn write_to_game(&self) {
		todo!()
	}

	fn write_to_mod(&self, id: String, relative_path : &Path, args : Vec<&'static str>) -> std::io::Result<()> {
		let file_to_write = args[0];
		let file_path_buf = relative_path.join(file_to_write);
		let file_path = file_path_buf.as_path();
		
		let buf = file_path.with_extension(".tmp");
  		let tmp_path = buf.as_path();

		if !file_path.exists() {
			self.create_manifest(file_path)?;
		}

		{
			// Read the manifest to modify.
			let manifest_read = File::open(file_path)?;
			let reader = BufReader::new(manifest_read);

			if tmp_path.exists() {
				std::fs::remove_file(tmp_path)?;
			}
			// To avoid having to store extensive manifests to program memory, we open a temporary file to write to:
			let manifest_tmp = File::create(tmp_path)?;
			let mut writer = BufWriter::new(manifest_tmp);

			self.modify_manifest(id, reader, &mut writer)?;
		}

		// Remove the old file:
		std::fs::remove_file(file_path)?;
		// Replace it with our temp file:
		std::fs::rename(tmp_path, file_path)?;

		Ok(())
	}
	
	fn load_from_dir(&self) {
			todo!()
	}
}