// Form object for utility functions like not allowing submission if this form is not completed.
use gtk::{subclass::prelude::*, glib::{self, Value}, prelude::*};

// Huge thanks to the code from https://github.com/sdroege/gst-plugin-rs/blob/95c007953c0874bc46152078775d673cf44cc255/net/webrtc/src/signaller/iface.rs, otherwise I never would have figured this out myself. There are just way too many steps and there's not enough documentation.

use super::form_manager::FormManager;

pub enum FormError {
	/// Generic invalid statement. Just highlight the form object without showing anything special.
	INVALID,
	// Might add more error types later, but this is fine for now.
}

mod imp {
	use gtk::glib::{Properties, ParamSpec, ParamSpecBoolean, subclass::{prelude::*, Signal}, ParamSpecString};
	use std::sync::OnceLock;

	use super::*;

	#[derive(Clone, Copy)]
	#[repr(C)]
	pub struct FormObject {
		parent : glib::gobject_ffi::GTypeInterface,
		// Create the list of functions to be stored by our Interface definition in GTK GObject stuff:
		pub is_valid : fn(&super::FormObject) -> bool,
		pub value : fn(&super::FormObject) -> Value,
		pub set_value : fn(&super::FormObject, Value),
		pub display_error : fn(&super::FormObject, Option<FormError>),
	}
	
	// Default functions:
	impl FormObject {
		fn is_valid(_this : &super::FormObject) -> bool {
			true
		}
		fn value(_this : &super::FormObject) -> Value {
			None::<String>.to_value()
		}
		fn set_value(_this : &super::FormObject, value: Value) {
			
		}
		fn display_error(_this : &super::FormObject, _error : Option<FormError>) {

		} 
	}

	#[glib::object_interface]
	unsafe impl ObjectInterface for FormObject {
		const NAME : &'static str = "JCCFormObject";

		fn properties() -> &'static [ParamSpec] {
			static PROPERTIES : OnceLock<Vec<ParamSpec>> = OnceLock::new(); 
			
			PROPERTIES.get_or_init(|| {
				vec![ParamSpecBoolean::builder("required").readwrite().build(), ParamSpecString::builder("label").readwrite().build()]
			})
		}

		fn interface_init(&mut self) {
			self.is_valid = FormObject::is_valid;
			self.value = FormObject::value;
			self.set_value = FormObject::set_value;
			self.display_error = FormObject::display_error;
		}
	}
}

glib::wrapper!{
	pub struct FormObject(ObjectInterface<imp::FormObject>) @requires gtk::Widget;
}

// Impl definition for people to override.
pub trait FormObjectImpl: ObjectImpl + ObjectSubclass {
	/// Only called if the property required is true.
	fn is_valid(&self) -> bool;
	fn value(&self) -> Value;
	fn set_value(&self, value: Value);
	fn display_error(&self, error : Option<FormError>);
}

unsafe impl<T: FormObjectImpl> IsImplementable<T> for FormObject {
	// Assign our struct functions to their actual values (i.e., the Impl definitions from implementors)
	fn interface_init(iface: &mut glib::Interface<Self>) {
		let iface = iface.as_mut();

		fn is_valid_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject) -> bool {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			FormObjectImpl::is_valid(this)
		}
		iface.is_valid = is_valid_trampoline::<T>;

		fn value_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject) -> Value {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			FormObjectImpl::value(this)
		}
		iface.value = value_trampoline::<T>;

		fn set_value_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject, value: Value) {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			T::set_value(this, value);
		}
		iface.set_value = set_value_trampoline::<T>;

		fn display_error_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject, error : Option<FormError>) {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			FormObjectImpl::display_error(this, error);
		}
		iface.display_error = display_error_trampoline::<T>;
	}
}

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
		let vtable = self.interface::<FormObject>().unwrap();
		let form = vtable.as_ref();
		(form.is_valid)(self.upcast_ref::<FormObject>())
	}

	fn label(&self) -> String {
		self.property("label")
	}

	fn value(&self) -> Value {
		let vtable = self.interface::<FormObject>().unwrap();
		let form = vtable.as_ref();
		(form.value)(self.upcast_ref::<FormObject>())
	}

	fn set_value(&self, value : Value) {
		let vtable = self.interface::<FormObject>().unwrap();
		let form = vtable.as_ref();
		(form.set_value)(self.upcast_ref::<FormObject>(), value)
	}

	fn display_error(&self, error: Option<FormError>) {
		let vtable = self.interface::<FormObject>().unwrap();
		let form = vtable.as_ref();
		(form.display_error)(self.upcast_ref::<FormObject>(), error);
	}
}

impl <T: IsA<FormObject> + IsA<gtk::Widget>> FormObjectExt for T {}