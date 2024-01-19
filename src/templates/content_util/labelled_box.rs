use std::{cell::{Cell, RefCell}, sync::atomic::AtomicBool};

use gtk::glib::{Properties, derived_properties, value::ToValueOptional, Value};
use crate::quick_template;
use crate::templates::content_util::form::FormObject;

use super::form::{FormObjectExt, FormObjectImpl};

// use super::form::FormObjectImpl;

quick_template!(LabelledBox, "/templates/content_util/labelled_box.ui", gtk::Box, (gtk::Widget), (;FormObject), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::LabelledBox)]
	struct {
		#[property(get, set)]
		pub label : RefCell<String>,

		// region: Bindings for setting is_valid:
		#[property(get, set)]
		pub bool_bind_valid : RefCell<Option<String>>,

		#[property(get, set)]
		pub string_bind_valid : RefCell<Option<String>>,
		// endregion

		// FormObject requirements:
		#[property(get, set)]
		pub required : RefCell<bool>,	
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
		println!("Valid check");
		let bool_bind_valid = self.bool_bind_valid.borrow().clone();
		let string_bind_valid = self.string_bind_valid.borrow().clone();

		if bool_bind_valid.is_some() {
			return bool_bind_valid.unwrap() == "true";
		} else if string_bind_valid.is_some() {
			return !string_bind_valid.unwrap().is_empty();
		}

		false
	}

	fn value(&self) -> Value {
		if !self.is_valid() {
			return None::<String>.to_value();
		}
		
		let bool_bind_valid = self.bool_bind_valid.borrow().clone();
		let string_bind_valid = self.string_bind_valid.borrow().clone();

		if bool_bind_valid.is_some() {
			return (bool_bind_valid.unwrap() == "true").to_value();
		} else if string_bind_valid.is_some() {
			return string_bind_valid.unwrap().to_value();
		}

		None::<String>.to_value()
	}
}

impl LabelledBox {
	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		LabelledBox::ensure_type();
	}
}