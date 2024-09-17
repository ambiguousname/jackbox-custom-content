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

        let mut array_nonzero = false;

        let mut written_values = false;

        // Now update our manifest value:
        while let (buf, Some(array_value)) = manifest.read_array_item() {
            array_nonzero = true;

            let val = array_value.map_err(|e| {
                if let ManifestError::StdErr(err) = e {
                    return err;
                }
                std::io::Error::new(ErrorKind::Other, e.to_string())
            })?;

            let test_id = val.as_object().and_then(|o| o.get("id"));

            if test_id.is_some() && test_id.unwrap().to_string() == format!(r#""{id}""#) {
                // manifest.active_writer = WriteTo::OutFile;
                manifest.write_to_outfile(&serde_out)?;
                written_values = true;
            } else {
                manifest.write_to_outfile(&buf)?;
            }
        }
        
        // If we've reached the end of the array with no out values, then we need to go back right before the array ends and write our value.
        if !written_values {
            // If there are items before this, we need to add a comma.
            if array_nonzero {
                manifest.write(b",")?;
            }

            manifest.write(&serde_out)?;
        }
        
        manifest.write(b"]")?;
        manifest.flush()?;

        Ok(())
    }

    fn load_from_dir(&self) {
        todo!()
    }
}

mod tests {
    use std::{fs::{self, File}, io::Read, path::Path};

    use serde_json::json;

    use crate::content::subcontent::Subcontent;

    use super::ManifestItem;

    struct TestManifest<'a> {
        path : &'a Path
    }

    impl Drop for TestManifest<'_> {
        fn drop(&mut self) {
            std::fs::remove_file(self.path).expect(format!("Could not remove {}", self.path.display()).as_str());
        }
    }

    fn assert_file_matches(path: &Path, buf: String) {
        assert!(path.exists());
        let read = File::open(path);
        assert!(read.is_ok(), "{}", read.err().unwrap());

        let mut reader = read.unwrap();
        let mut out_str = String::new();
        let write = reader.read_to_string(&mut out_str);
        assert!(write.is_ok(), "{}", write.err().unwrap());
        assert_eq!(out_str, buf);
    }

    #[test]
    fn write_single_manifest_item() {
        let p = Path::new("manifest-write-test.json");

        let _test = TestManifest {
            path: p
        };

        let v = ManifestItem::new(
            json!({
                "value": "test"
            })
        );

        v.write_to_mod("0".into(), Path::new("./"), vec![p.to_str().unwrap()]).unwrap();

        assert_file_matches(p, String::from(r#"[{"id":"0","value":"test"}]"#));
    }

    #[test]
    fn write_multiple_manifest_items() {
        let p = Path::new("multi-manifest-write.json");

        let _test = TestManifest {
            path: p
        };

        let mut values : Vec<String> = Vec::new();
        for i in 0..5 {
            let v = ManifestItem::new(
                json!({
                    "value": "testing"
                })
            );

            v.write_to_mod(i.to_string(), Path::new("./"), vec![p.to_str().unwrap()]).unwrap();

            values.push(format!(r#"{{"id":"{i}","value":"testing"}}"#).into());

            assert_file_matches(p, format!("[{}]", values.join(",")));
        }
    }

    #[test]
    fn edit_manifest_item() {
        let p = Path::new("manifest-edit.json");

        let _test = TestManifest {
            path: p
        };
        
        let mut values : Vec<String> = Vec::new();
        for i in 0..5 {
            let v = ManifestItem::new(
                json!({
                    "value": "testing"
                })
            );

            v.write_to_mod(i.to_string(), Path::new("./"), vec![p.to_str().unwrap()]).unwrap();

            values.push(format!(r#"{{"id":"{i}","value":"testing"}}"#).into());

            assert_file_matches(p, format!("[{}]", values.join(",")));
        }

        let edit = ManifestItem::new(json!({"otherValue": "test"}));
        
        edit.write_to_mod("3".into(), Path::new("./"), vec![p.to_str().unwrap()]).unwrap();

        values[3] = format!(r#"{{"id":"3","otherValue":"test"}}"#);

        assert_file_matches(p, format!("[{}]", values.join(",")))
    }

    #[test]
    fn edit_existing_items() {
        let p = Path::new("manifest-existing-edit.json");

        let _test = TestManifest {
            path: p
        };

        fs::write(p, format!(
r#"[{{ "id": "0", "value": "test"
}}, {{
"id": "1", "value": "item"
}},
{{
"id": "2", "value": "item"
}},
{{
"id": "1", "value": "a"
}}]"#)).unwrap();

        let edit = ManifestItem::new(json!({"newValue": "testing"}));

        edit.write_to_mod("1".into(), Path::new("./"), vec![p.to_str().unwrap()]).unwrap();

        assert_file_matches(p, format!(
r#"[{{ "id": "0", "value": "test"
}}, {{"id":"1","newValue":"testing"}},
{{
"id": "2", "value": "item"
}},
{{"id":"1","newValue":"testing"}}]"#));
    }
}