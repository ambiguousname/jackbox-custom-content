use crate::content::ContentCategory;

pub const QUIPLASH_PROMPT : ContentCategory = ContentCategory {
    name: "Quiplash Round Prompt",
    open_window : || {
        println!("TEST");
    },
};

pub const QUIPLASH_SAFETY : ContentCategory = ContentCategory {
    name: "Safety Quip",
    open_window: || {
        println!("SAFE");
    },
};