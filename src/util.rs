use std::{fs::read_to_string, path::Path, collections::HashMap};
use serde_json::Value;

pub fn file_to_values(filename: String) -> HashMap<String, Value> {
    let path = Path::new(filename.as_str());
    let result = read_to_string(path);
    let full_string = result.unwrap();

    let values = serde_json::from_str(full_string.as_str()).unwrap();
    values
}

macro_rules! full_custom_dat {
    // Should be of the format full_custom_dat!(["t", "v", "n"], ["t", "v", "n"], ["t", "v", "n"], ...)
    ($(dat:tt),*) => {
        json!({
            "fields": [
                $(custom_dat! $dat)*
            ]
        });
    }
}

macro_rules! custom_dat {
    ($t:expr, $v:expr, $n: expr) => {
        "t": $t,
        "v": $v,
        "n": $n,
    }
}