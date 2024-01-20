use gtk::glib::{Properties, derived_properties};

use crate::quick_object;
use super::form::{FormObject, FormObjectImpl};

use std::cell::RefCell;

quick_object!(FormCheckbox, gtk::CheckButton, (gtk::Widget), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget; FormObject),
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::FormCheckbox)]
	struct {
		#[property(get, set)]
		pub required : RefCell<bool>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::FormCheckbox {}
impl WidgetImpl for imp::FormCheckbox {}
impl CheckButtonImpl for imp::FormCheckbox {}
impl FormObjectImpl for imp::FormCheckbox {
	fn display_error(&self, error : Option<super::form::FormError>) {
		match error {
			Some(super::form::FormError::INVALID) => self.obj().add_css_class("error"),
			_ => self.obj().remove_css_class("error"),
		};
	}

	/// is_valid should only be called if required is set to true.
	/// If required is true, then we make the assumption that the checkbox value must be set to true (like an "I acknowledge" checkbox).
	fn is_valid(&self) -> bool {
		self.value().get().unwrap()
	}

	fn value(&self) -> glib::Value {
		self.obj().is_active().to_value()
	}
}

impl FormCheckbox {
	pub fn ensure_all_types() {
		FormCheckbox::ensure_type();
	}
}