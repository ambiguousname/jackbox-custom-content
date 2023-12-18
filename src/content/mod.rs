use std::collections::HashMap;

use super::templates::mainmenu::MainMenuWindow;

pub mod quiplash3;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize)]
pub struct ContentData {
    // Unique identifier for the data:
    id : i32,
    // The relative path to the .jet file from the /games/ directory.
    master_jet : &'static str,
    properties : HashMap<&'static str, Value>,
}

impl ContentData {
    fn write_to_game(&self, games_path : &str) {
        
    }
}

pub struct ContentCategory
{
    pub name: &'static str,
    pub open_window : fn() -> gtk::Window,
}

pub struct GameContent {
    // Internal game ID (the relative folder from the /games/ directory)
    pub game_id: &'static str,
    // The game's display name.
    pub name: &'static str,
    pub content_categories: &'static [ContentCategory],
}


pub fn initialize_content(window : MainMenuWindow) {
    let categories = vec![quiplash3::GAME_INFO];

    window.add_game_info(categories);
}