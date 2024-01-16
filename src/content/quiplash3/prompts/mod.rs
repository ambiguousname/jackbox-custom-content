use crate::{quick_template, content::{ContentWindow, Content, ContentWindowImpl}};

mod prompt_util;
use prompt_util::QuiplashGenericRoundPrompt;

// TODO: Transfer prompt data across notebooks?
quick_template!(QuiplashRoundPrompt, "/content/quiplash3/prompts/round_prompt.ui", ContentWindow, (gtk::Window, gtk::Widget, Content), (gtk::Native, gtk::Root, gtk::ShortcutManager),
    #[derive(Default, CompositeTemplate)]
    handlers struct {
        
    }
);

impl ObjectImpl for imp::QuiplashRoundPrompt {}
impl WidgetImpl for imp::QuiplashRoundPrompt {}
impl WindowImpl for imp::QuiplashRoundPrompt {}
impl ContentWindowImpl for imp::QuiplashRoundPrompt {}

#[gtk::template_callbacks]
impl QuiplashRoundPrompt {
    #[template_callback]
    pub fn handle_create_clicked(&self) {
        self.emit_by_name::<()>("content-created", &[&"Test Value"]);
    }
    
    pub fn ensure_all_types() {
        QuiplashGenericRoundPrompt::ensure_all_types();
        QuiplashRoundPrompt::ensure_type();
    }
}