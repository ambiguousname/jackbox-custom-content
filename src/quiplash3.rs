const GAME_NAME : &str = "Quiplash3";

use crate::util::{file_to_json};
use crate::content::{Content, ContentCategory, GAME_DIR};

pub struct Round1Question;

fn load_round_question(content_name : &str) -> Vec<Content> {
    let content_list = file_to_json(format!("{}{}/{}{}.jet", GAME_DIR, GAME_NAME, GAME_NAME, content_name));
    let vector: Vec<Content> = Vec::new();
    for item in content_list["content"].as_array().iter() {
        
    }
    
    vector
}

impl<'a> ContentCategory<'a> for Round1Question {
    const CONTENT_NAME : &'a str = "Round1Question";
    fn load_content() -> Vec<Content> {
        load_round_question(Round1Question::CONTENT_NAME)
    }

    fn save_as_json(content: &Content) -> String {
        String::from("")
    }
}