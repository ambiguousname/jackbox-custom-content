use crate::{quick_template, content::{Content, ContentWindow}};

mod prompt_util;
use prompt_util::QuiplashGenericRoundPrompt;

// TODO: Transfer prompt data across notebooks?
quick_template!(QuiplashRoundPrompt, "/content/quiplash3/prompts/round_prompt.ui", gtk::Window, (gtk::Widget, Content), (gtk::Native, gtk::Root, gtk::ShortcutManager));
impl ObjectImpl for imp::QuiplashRoundPrompt {}
impl WidgetImpl for imp::QuiplashRoundPrompt {}
impl WindowImpl for imp::QuiplashRoundPrompt {}

impl ContentWindow for QuiplashRoundPrompt {
    fn ensure_all_types() {
        QuiplashGenericRoundPrompt::ensure_all_types();
        QuiplashRoundPrompt::ensure_type();
    }
}