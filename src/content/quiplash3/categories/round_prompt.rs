use crate::content::ContentCategory;
use gtk::{Window, Dialog};

pub trait QuiplashRoundPrompt {

}

pub struct QuiplashRound1Prompt {}

impl QuiplashRoundPrompt for QuiplashRound1Prompt {}

pub struct QuiplashRound2Prompt {}

impl QuiplashRoundPrompt for QuiplashRound2Prompt {}

pub struct QuiplashFinalRoundPrompt {}

impl QuiplashRoundPrompt for QuiplashFinalRoundPrompt {}

// TODO: Modify so this is static?
fn prompt_window() -> Window {
    let window = Dialog::builder().title("Quiplash Round Prompt").build();
    
    window.into()
}

pub const QUIPLASH_PROMPT : ContentCategory = ContentCategory {
    name: "Quiplash Round Prompt",
    open_window : prompt_window,
};