use crate::{content::ContentCategory, quick_template};
use gtk::{Window, prelude::StaticTypeExt};

use super::prompt_util::QuiplashGenericRoundPrompt;


quick_template!(QuiplashRoundPrompt, "/content/quiplash3/categories/round_prompt.ui", gtk::Window, (), (gtk::Native, gtk::Root, gtk::ShortcutManager), {
    impl ObjectImpl for QuiplashRoundPrompt {}
    impl WidgetImpl for QuiplashRoundPrompt {}
    impl WindowImpl for QuiplashRoundPrompt {}
});

impl QuiplashRoundPrompt {
    fn new() -> Self {
        glib::Object::new()
    }
}

// TODO: Modify so this is static (using round_prompt.ui)
fn prompt_window() -> Window {
    // For unknown templates we have to ensure a type:
    QuiplashGenericRoundPrompt::ensure_type();
    QuiplashRoundPrompt::new().into()
}

pub const QUIPLASH_PROMPT : ContentCategory = ContentCategory {
    name: "Quiplash Round Prompt",
    open_window : prompt_window,
};