mod categories;

use super::GameContent;

pub const GAME_INFO: GameContent = GameContent {
    game_id: "Quiplash3",
    name: "Quiplash 3",
    content_categories: vec![],
};

// Let's break down what needs to happen to create or modify a question, in reverse:
// 4. Create/modify the .JET file containing the specific question information (can just copy/modify create_quiplash_data_jet behavior from jppc.py)
// 3. Show a window for the user to create/modify a question, with a list of all current questions. (Starts getting specific to the category)
// 2. Load the master .JET file for the question type to get ALL questions of that type.
// 1. Have the player select a content type.


// pub trait Quiplash3Round1Question : Quiplash3RoundQuestion;
/*fn save_as_json(&self) {
    json!({
        full_custom_dat!([
            ["B", "", "HasJokeAudio"]
        ]);
    });
    super::ContentCategory::save_as_json(&self);
}*/