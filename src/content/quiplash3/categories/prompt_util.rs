use crate::quick_template;

quick_template!(QuiplashGenericRoundPrompt, "/content/quiplash3/categories/generic_round_prompt.ui", gtk::Box, (), (gtk::Orientable), {
    impl ObjectImpl for QuiplashGenericRoundPrompt {}
    impl WidgetImpl for QuiplashGenericRoundPrompt {}
    impl BoxImpl for QuiplashGenericRoundPrompt {}
});

impl QuiplashGenericRoundPrompt {
    pub fn new() -> Self {
        glib::Object::new()
    }
}