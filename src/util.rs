use std::{fs::read_to_string, path::Path, collections::HashMap};
use serde_json::Value;

pub fn file_to_json(filename: String) -> HashMap<String, Value> {
    let path = Path::new(filename.as_str());
    let result = read_to_string(path);
    let full_string = result.unwrap();

    let values = serde_json::from_str(full_string.as_str()).unwrap();
    values
}