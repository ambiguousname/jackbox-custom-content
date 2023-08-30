use crate::content::ContentCategory;
use gtk::{Window, Dialog};

fn safety_window() -> Window {
    let window = Dialog::builder().title("TEST").build();
    window.into()
}

pub const QUIPLASH_SAFETY : ContentCategory = ContentCategory {
    name: "Safety Quip",
    open_window: safety_window,
};