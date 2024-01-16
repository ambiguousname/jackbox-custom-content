use std::cell::{Cell, RefCell};

use gtk::glib::{Properties, derived_properties};

use crate::quick_template;

quick_template!(LabelledEntry, "/templates/content_util/labelled_entry.ui", gtk::Box, (gtk::Widget), (), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::LabelledEntry)]
	struct {
		// #[template_child]
		// pub label_child : TemplateChild<gtk::Label>,
		// #[template_child]
		// pub entry_child : TemplateChild<gtk::Entry>,

		#[property(get, set)]
		pub label : RefCell<String>,

		#[property(get, set)]
		pub placeholder_text : RefCell<String>,

		#[property(get, set)]
		pub entry_hexpand : RefCell<bool>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::LabelledEntry {}
impl WidgetImpl for imp::LabelledEntry {}
impl BoxImpl for imp::LabelledEntry {}

impl LabelledEntry {
	pub fn ensure_all_types() {
		LabelledEntry::ensure_type();
	}
}