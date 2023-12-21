use crate::quick_template;

use gtk::{Window, Switch};

quick_template!(PreferencesWindow, "/templates/preferences/preferences.ui", Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager), struct {
	#[template_child(id="dark_mode")]
	pub dark_mode_switch : TemplateChild<Switch>,
});

impl ObjectImpl for imp::PreferencesWindow {
	fn constructed(&self) {
        self.parent_constructed();
    }
}
impl WidgetImpl for imp::PreferencesWindow {}
impl WindowImpl for imp::PreferencesWindow {}

impl PreferencesWindow {
	pub fn new(parent: &impl IsA<Window>) -> Self {
		Object::builder()
		.property("transient-for", parent)
		.property("hide-on-close", true)
		.build()
	}
}

impl Default for PreferencesWindow {
    fn default() -> Self {
        let this : Self = Object::builder().build();
        this
    }
}