// Form object for utility functions like not allowing submission if this form is not completed.
use gtk::{subclass::prelude::*, glib, prelude::*};

use super::form_manager::FormManager;

mod imp {
	use gtk::glib::{Properties, ParamSpec, once_cell::sync::Lazy, ParamSpecBoolean, subclass::{prelude::*, Signal}};

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

		fn signals() -> &'static [Signal] {
			static SIGNALS : Lazy<Vec<Signal>> = Lazy::new(|| {
				vec![Signal::builder("is-valid").return_type::<bool>().build()]
			});
			SIGNALS.as_ref()
		}
	}
}

glib::wrapper!{
	pub struct FormObject(ObjectInterface<imp::FormObject>) @requires gtk::Widget;
}

pub trait FormObjectImpl: ObjectImpl + ObjectSubclass {
	fn is_valid(&self) -> bool;
}

unsafe impl<T: FormObjectImpl> IsImplementable<T> for FormObject {}

impl FormObject {
	pub fn ensure_all_types() {
		FormObject::ensure_type();
	}
}

pub trait FormObjectExt : IsA<FormObject> + IsA<gtk::Widget> + 'static {
	// FIXME: This would be nice to do automatically on constructed/realized/whatever.
	fn construct_form_obj(&self) {
		let ancestor : FormManager = self.ancestor(FormManager::static_type()).and_downcast().expect("Could not get FormManager.");
		ancestor.add_form_object(self.clone().into());
	}

	fn required(&self) -> bool {
		self.property("required")
	}

	fn set_required(&self, required : bool) {
		self.set_property("required", required);
	}

	fn is_valid(&self) -> bool {
		let valid : bool = self.emit_by_name("is-valid", &[]);
		valid
	}
}

impl <T: IsA<FormObject> + IsA<gtk::Widget>> FormObjectExt for T {}