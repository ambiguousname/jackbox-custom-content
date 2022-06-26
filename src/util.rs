use std::{fs::read_to_string, path::Path};
use serde_json::Value;

pub fn file_to_json(filename: &str) -> Value {
    let path = Path::new(filename);
    let full_string = read_to_string(path).unwrap().as_str();

    let values : Value = serde_json::from_str(full_string).unwrap();
    values
}