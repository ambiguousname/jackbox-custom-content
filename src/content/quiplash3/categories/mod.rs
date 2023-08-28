use crate::content::ContentCategory;
use gtk::{Window, Dialog};

// TODO: Modify so this is static?
fn prompt_window() -> Window {
    let window = Dialog::builder().title("Text").build();
    window.into()
}

pub const QUIPLASH_PROMPT : ContentCategory = ContentCategory {
    name: "Quiplash Round Prompt",
    open_window : prompt_window,
};

fn safety_window() -> Window {
    let window = Dialog::builder().title("TEST").build();
    window.into()
}

pub const QUIPLASH_SAFETY : ContentCategory = ContentCategory {
    name: "Safety Quip",
    open_window: safety_window,
};