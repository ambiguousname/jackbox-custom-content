use std::cell::{Cell, RefCell};

use gtk::glib::{Properties, derived_properties};

use crate::quick_template;

quick_template!(LabelledBox, "/templates/content_util/labelled_box.ui", gtk::Box, (gtk::Widget), (), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::LabelledBox)]
	struct {
		// #[template_child]
		// pub label_child : TemplateChild<gtk::Label>,
		// #[template_child]
		// pub entry_child : TemplateChild<gtk::Entry>,

		#[property(get, set)]
		pub label : RefCell<String>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::LabelledBox {}
impl WidgetImpl for imp::LabelledBox {}
impl BoxImpl for imp::LabelledBox {}

impl LabelledBox {
	pub fn ensure_all_types() {
		LabelledBox::ensure_type();
	}
}