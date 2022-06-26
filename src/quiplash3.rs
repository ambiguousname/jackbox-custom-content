const GAME_NAME : String = String::from("Quiplash3");
// TODO: Change
const PATH : String = format!("../games/{}/content/", GAME_NAME);

use crate::util::{file_to_json};
use crate::content::{Content, ContentCategory};

pub struct Round1Question;

fn load_round_question(path : String, content_name : String) -> Vec<Content> {
    let content_list = file_to_json(format!("{}{}{}.jet", PATH, GAME_NAME, content_name));
}

impl ContentCategory for Round1Question {
    const CONTENT_NAME : String = String::from("Round1Question");
    fn load_content() -> Vec<Content> {
        load_round_question(PATH, Round1Question::CONTENT_NAME)
    }

    fn save_as_json(content: &Content) -> String {
        String::from("")
    }
}