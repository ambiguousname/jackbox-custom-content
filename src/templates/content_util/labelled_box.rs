use std::cell::{Cell, RefCell};

use gtk::glib::{Properties, derived_properties};
use crate::quick_template;
use crate::templates::content_util::form::FormObject;

quick_template!(LabelledBox, "/templates/content_util/labelled_box.ui", gtk::Box, (gtk::Widget), (;FormObject), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::LabelledBox)]
	struct {
		#[property(get, set)]
		pub label : RefCell<String>,

		// FormObject requirements:
		#[property(get, set)]
		pub required : RefCell<bool>,	
	}
);

#[derived_properties]
impl ObjectImpl for imp::LabelledBox {}
impl WidgetImpl for imp::LabelledBox {}
impl BoxImpl for imp::LabelledBox {}

impl LabelledBox {
	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		LabelledBox::ensure_type();
	}
}