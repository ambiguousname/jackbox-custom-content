use std::{fs::File, path::Path};
use serde_json;
use serde::{Serialize, Deserialize};
use gtk::gio::prelude::FileExt;

#[derive(Default, Serialize, Deserialize)]
pub struct ModsConfig {
    folder_path : Option<String>,
}

impl ModsConfig {
    pub fn initialize(&mut self) {
        self.load_from_json();
    }

    pub fn reset(&mut self) {
        // TODO: Check this works.
        *self = Default::default();
    }

    pub fn set_game_folder(&mut self, folder : gtk::gio::File) {
        self.folder_path = Some(folder.parse_name().to_string());
        self.write();
    }

    pub fn get_game_folder(&self) -> Option<String> {
        self.folder_path.clone()
    }

    pub fn write(&self) {
        let json = File::create("./settings.json").expect("Could not write to settings.json");

        let result = serde_json::to_writer(json, self);

        if result.is_err() {
            let msg = result.err().unwrap();
            eprintln!("Could not write to settings.json: {msg}");
        }
    }

    fn load_from_json(&mut self) {
        let json_file = Path::new("./settings.json");
        if !json_file.exists() {
            self.write();
        }

        let json = File::open(json_file).expect("Could not open settings.json");
        *self = serde_json::from_reader(json).expect("Could not read settings.json to Settings.");
    }
}