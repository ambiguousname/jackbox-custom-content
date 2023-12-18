// TODO:
// 1. Store mod folder locations in settings. 
// 2. Load mod data from settings.
// 3. Save lists of mod data.

use crate::content::ContentData;
use std::{fs::{self, File, DirEntry}, path::Path, collections::HashMap};
use serde_json;
use serde::{Serialize, Deserialize};
use gtk::gio::prelude::FileExt;

#[derive(Default)]
pub struct JackboxMod {
    name : String,
    id : String,
    // Store content on the heap, so the vast amount of content that there may be isn't stored on the stack.
    // I'm curious if this will ever be a problem for lots of content with lots of different values. Dangerous to store all the mods you have one editor all at once?
    // Maybe to mitigate at some point it'd be good to just select one mod at a time?
    // Or dynamically load/unload as you select mods.
    content_list : Box<Vec<ContentData>>,
}

impl JackboxMod {
    fn new(name: String) -> Self {
        let id = JackboxMod::string_to_id(name.clone());

        JackboxMod {
            name,
            id,
            ..Default::default()
        }
    }

    fn string_to_id(string : String) -> String {
        string.to_ascii_lowercase().replace(" ", "_")
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    folder_path : Option<String>,
}

impl Settings {
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

#[derive(Default)]
pub struct ModsConfig {
    pub settings : Settings,
    pub mods : HashMap<String, JackboxMod>,
}

impl ModsConfig {
    // Load mods and settings from local .json file and their folders.
    pub fn initialize(&mut self) {
        self.settings.load_from_json();

        // Does the mods folder exist?
        let mods_folder = Path::new("./mods");

        if !mods_folder.exists() {
            let result = fs::create_dir(mods_folder.clone());
            if result.is_err() {
                eprintln!("Could not create ./mods directory.");
            }
        }

        for directory in fs::read_dir(mods_folder).unwrap() {
            let dir = directory.expect("Could not get child directory.");
            self.load_mod_from_dir(dir);
        }
    }

    pub fn mod_exists(&self, mod_name : String) -> bool {
        self.mods.contains_key(&JackboxMod::string_to_id(mod_name))
    }

    pub fn new_mod(&mut self, mod_name : String) -> Result<(), &'static str> {
        if self.mod_exists(mod_name.clone()) {
            return Err("Mod already exists.");
        }

        let jackbox_mod = JackboxMod::new(mod_name);

        self.mods.insert(jackbox_mod.get_id(), jackbox_mod);

        Ok(())
    }

    fn load_mod_from_dir(&mut self, dir : DirEntry) {
        
    }
}