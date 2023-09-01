use gtk::prelude::StaticTypeExt;

use crate::{quick_template, templates::editable_list::EditableList};

quick_template!(QuiplashGenericRoundPrompt, "/content/quiplash3/categories/generic_round_prompt.ui", gtk::Frame, (gtk::Widget), ());

impl ObjectImpl for imp::QuiplashGenericRoundPrompt {}
impl WidgetImpl for imp::QuiplashGenericRoundPrompt {}
impl FrameImpl for imp::QuiplashGenericRoundPrompt {}

impl QuiplashGenericRoundPrompt {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn ensure_all_types() {
        EditableList::ensure_type();
        QuiplashGenericRoundPrompt::ensure_type();
    }
}