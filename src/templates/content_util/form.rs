// Form object for utility functions like not allowing submission if this form is not completed.
use gtk::{subclass::prelude::*, glib, prelude::*};

use super::form_manager::FormManager;

mod imp {
	use gtk::glib::{Properties, ParamSpec, once_cell::sync::Lazy, ParamSpecBoolean};

	use super::*;

	#[derive(Clone, Copy)]
	#[repr(C)]
	pub struct FormObject {
		parent : glib::gobject_ffi::GTypeInterface,
	}

	#[glib::object_interface]
	unsafe impl ObjectInterface for FormObject {
		const NAME : &'static str = "JCCFormObject";
		

		fn properties() -> &'static [ParamSpec] {
			static PROPERTIES : Lazy<Vec<ParamSpec>> = Lazy::new(|| {
				vec![ParamSpecBoolean::builder("required").readwrite().build()]
			});
			PROPERTIES.as_ref()
		}
	}
}

glib::wrapper!{
	pub struct FormObject(ObjectInterface<imp::FormObject>);
}

unsafe impl<T: ObjectSubclass> IsImplementable<T> for FormObject {}

impl FormObject {
	pub fn ensure_all_types() {
		FormObject::ensure_type();
	}
}

pub trait FormObjectExt : IsA<FormObject> + 'static {
	fn required(&self) -> bool {
		self.property("required")
	}

	fn set_required(&self, required : bool) {
		self.set_property("required", required);
	}
}

impl <T: IsA<FormObject>> FormObjectExt for T {}