use std::{
    fs::File,
    io::{Cursor, ErrorKind, Read, Write},
    path::Path,
};

use crate::util::manifest_writer::{ManifestError, ManifestWriter, WriteTo};

use super::Subcontent;

/// A manifest.jet file that lists our content.
#[derive(Debug)]
pub struct ManifestItem {
    /// A serde_json value of content to write.
    item_content: serde_json::Value,
}

// TODO: Write loading.

impl ManifestItem {
    pub fn new(item_content: serde_json::Value) -> Self {
        ManifestItem {
            item_content: item_content,
        }
    }

    pub fn content(&self) -> serde_json::Value {
        self.item_content.clone()
    }

    fn create_manifest(&self, path: &Path) -> std::io::Result<()> {
        let mut manifest = File::create(path)?;

        // Compatibility with anything else that might want to read our manifest file.
        // Really we just want a comma separated list of values to read, so we use an array.
        // Then we can easily merge with other manifest.json files.
        manifest.write(b"[\n]")?;
        Ok(())
    }
}

impl Subcontent for ManifestItem {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn write_to_game(&self) {
        todo!()
    }

    fn write_to_mod(
        &self,
        id: String,
        relative_path: &Path,
        args: Vec<&'static str>,
    ) -> std::io::Result<()> {
        let file_to_write = args[0];
        let file_path_buf = relative_path.join(file_to_write);
        let file_path = file_path_buf.as_path();

        if !file_path.exists() {
            self.create_manifest(file_path)?;
        }

        let mut manifest = ManifestWriter::<std::io::Empty>::open(file_path)?;
        // Clone our manifest to insert new values:
        let mut to_insert = self.item_content.as_object().unwrap().clone();
        // Add our given ID to the manifest:
        to_insert.insert(String::from("id"), serde_json::Value::String(id.clone()));

        // Parse the first node to enter our manifest object:
        let _ = manifest.initialize().map_err(|e| {
            if let ManifestError::StdErr(err) = e {
                return err;
            }
            std::io::Error::new(ErrorKind::Other, e.to_string())
        })?;

        let serde_out = serde_json::to_vec(&to_insert)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e.to_string()))?;

        manifest.active_writer = WriteTo::Buffer(Cursor::new(Vec::new()));

        let mut written_values = false;
        // Now update our manifest value:
        while let Some(array_value) = manifest.read_array_item() {
            let val = array_value.map_err(|e| {
                if let ManifestError::StdErr(err) = e {
                    return err;
                }
                std::io::Error::new(ErrorKind::Other, e.to_string())
            })?;

            let test_id = val.as_object().and_then(|o| o.get("id"));

            if !written_values && test_id.is_some() {
                if test_id.unwrap().to_string() == id {
                    manifest.active_writer = WriteTo::OutFile;

                    manifest.write(&serde_out)?;
                    manifest.write(b",")?;
                    written_values = true;
                } else {
                    manifest.write(val.to_string().as_bytes())?;
                }
            }
        }
        // If we've reached the end of the array with no out values, then we need to go back right before the array ends and write our value.
        if !written_values {
            // Flush our buffer first:
            let mut str = String::new();
            match &mut manifest.active_writer {
                WriteTo::Buffer(w) => {
                    w.set_position(0);
                    w.read_to_string(&mut str)?;
                    w.get_mut().clear();
                }
                _ => unreachable!("Unrecognized writer."),
            };
            manifest.write_to_outfile(str.as_bytes())?;

            // Then write our values:
            manifest.active_writer = WriteTo::OutFile;
            manifest.write(&serde_out)?;
            manifest.write(b"]")?;
        }
        manifest.flush()?;

        Ok(())
    }

    fn load_from_dir(&self) {
        todo!()
    }
}
