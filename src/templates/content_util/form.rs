// Form object for utility functions like not allowing submission if this form is not completed.
use std::{cell::RefCell, borrow::Borrow};

use gtk::glib::{ObjectExt, derived_properties, Properties};

use crate::quick_object;

use super::form_manager::FormManager;

quick_object!(FormObject, gtk::Widget, (), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget), 
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::FormObject)]
	struct {
		#[property(get, set)]
		pub required : RefCell<bool>,
	}
);

impl FormObject {
	pub fn ensure_all_types() {
		FormObject::ensure_type();
	}
}

#[derived_properties]
impl ObjectImpl for imp::FormObject {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();
		let manager : FormManager = obj.ancestor(FormManager::static_type()).and_downcast().expect("Could not find parent FormManager");

	}
}
impl WidgetImpl for imp::FormObject {}

pub trait FormObjectExt : IsA<FormObject> + 'static {
	fn is_required(&self) -> bool {
		self.property("required")
	}

	fn set_required(&self, required : bool) {
		self.set_property("required", required);
	}
}

impl<O: IsA<FormObject>> FormObjectExt for O {}

pub trait FormObjectImpl : WidgetImpl {}

unsafe impl<T: FormObjectImpl> IsSubclassable<T> for FormObject {}