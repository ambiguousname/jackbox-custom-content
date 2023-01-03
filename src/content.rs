use serde_json::Value;

// TODO: Change
pub const GAME_DIR: &str = "../games/";

pub struct Content
{
    id: u32,
    // This is to easily convert to and from JSON/JET format.
    values: Value,
};

pub trait ContentCategory {
    fn load_content(&mut self) {
        // Load the .JET master file to list ALL content of type content_name.
        let content_list = file_to_json(format!("{}{}/content/{}{}.jet", GAME_DIR, GAME_NAME, GAME_NAME, content_name));
        self.master_jet: Vec<Content> = Vec::new();
        for item in content_list["content"].as_array().iter() {
            self.master_jet.push(item);
        }
    };
    fn save_as_json(&self) {
        
    };
    fn render_window(&self);
};

struct ContentCategoryDat {
    name: String,
    master_jet: Vec<Content>,
    category_type: dyn ContentCategory,
};