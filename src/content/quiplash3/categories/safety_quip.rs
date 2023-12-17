use crate::content::ContentCategory;
use gtk::Window;

fn safety_window() -> Window {
    let window = Window::builder().title("TEST").build();
    window.into()
}

pub const QUIPLASH_SAFETY : ContentCategory = ContentCategory {
    name: "Safety Quip",
    open_window: safety_window,
};