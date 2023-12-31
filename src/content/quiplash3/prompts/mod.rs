use crate::{quick_template, content::{ContentWindow, Content, ContentWindowImpl}};

mod prompt_util;
use prompt_util::QuiplashGenericRoundPrompt;

// TODO: Transfer prompt data across notebooks?
quick_template!(QuiplashRoundPrompt, "/content/quiplash3/prompts/round_prompt.ui", ContentWindow, (gtk::Window, gtk::Widget, Content), (gtk::Native, gtk::Root, gtk::ShortcutManager));
impl ObjectImpl for imp::QuiplashRoundPrompt {}
impl WidgetImpl for imp::QuiplashRoundPrompt {}
impl WindowImpl for imp::QuiplashRoundPrompt {}

impl ContentWindowImpl for imp::QuiplashRoundPrompt {
    // fn ensure_all_types() {
    //     QuiplashGenericRoundPrompt::ensure_all_types();
    //     QuiplashRoundPrompt::ensure_type();
    // }

    // fn create_content(&self) {
        
    // }
}

impl QuiplashRoundPrompt {
    // fn test(&self) {
    //     self.test()
    // }
    pub fn ensure_all_types() {
        QuiplashGenericRoundPrompt::ensure_all_types();
        QuiplashRoundPrompt::ensure_type();
    }
}