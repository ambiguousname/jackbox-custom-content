use std::cell::RefCell;

use gtk::glib::{Properties, derived_properties, Value};
use crate::quick_template;
use crate::templates::content_util::form::FormObject;

use super::form::{FormObjectExt, FormObjectImpl};

// use super::form::FormObjectImpl;

quick_template!(LabelledBox, "/templates/content_util/labelled_box.ui", gtk::Box, (gtk::Widget), (;FormObject), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::LabelledBox)]
	struct {

		#[property(get, set)]
		pub value_property : RefCell<String>,

		// FormObject requirements:
		#[property(get, set)]
		pub required : RefCell<bool>,
		
		#[property(get, set)]
		pub label : RefCell<String>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::LabelledBox {}
impl WidgetImpl for imp::LabelledBox {
	fn realize(&self) {
		self.parent_realize();
		self.obj().construct_form_obj();
	}
}
impl BoxImpl for imp::LabelledBox {}
impl FormObjectImpl for imp::LabelledBox {
	fn is_valid(&self) -> bool {
		self.obj().is_valid()
	}

	fn value(&self) -> Value {
		self.obj().value()
	}
}

impl LabelledBox {
	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		LabelledBox::ensure_type();
	}

	pub fn is_valid(&self) -> bool {
		let property = self.value();

		let prop_type = property.value_type();

		// Not easily created as a constant:
		let static_str_vec = Vec::<String>::static_type();
		if prop_type.is_a(static_str_vec) {
			return property.get::<Vec::<String>>().unwrap().is_empty();
		}

		return match prop_type {
			// For checkboxes requiring an acknowledgement or something.
			// Will probably never happen 
			glib::Type::BOOL => property.get::<bool>().unwrap() == true,
			// For things like Entries:
			glib::Type::STRING => !property.get::<String>().unwrap().is_empty(),
			_ => false,
		}
	}

	pub fn value(&self) -> Value {
		self.last_child().unwrap().property(&self.imp().value_property.borrow().clone())
	}
}

pub trait LabelledBoxImpl : BoxImpl {}

unsafe impl<T: LabelledBoxImpl> IsSubclassable<T> for LabelledBox {}