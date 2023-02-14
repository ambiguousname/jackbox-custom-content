use serde_json::Value;

pub mod quiplash3;

// TODO: Change
pub const GAME_DIR: &str = "../games/";

struct ContentCategoryDat {
    name: String,
    content_name: String,
    master_jet: Vec<Content>,
    category_type: dyn ContentCategory,
}

pub struct Content<'a>
{
    id: u32,
    // This is to easily convert to and from JSON/JET format.
    values: Value,
    category: &'a ContentCategoryDat,
}

pub trait ContentLoader {
    fn save_as_json(&self) {
        format!("{}{}/content/{}{}.jet", GAME_DIR, self.category.game_name, self.category.game_name, self.category.name);
    }
}

pub trait ContentCategory {
    fn load_content(&mut self) {
        // Load the .JET master file to list ALL content of its type.
        let content_list = file_to_vaues(format!("{}{}/content/{}{}.jet", GAME_DIR, self.game_name, self.game_name, self.name));
        self.master_jet: Vec<Content> = Vec::new();
        for item in content_list["content"].as_array().iter() {
            self.master_jet.push(item);
        }
    }
    fn render_window(&self);
}