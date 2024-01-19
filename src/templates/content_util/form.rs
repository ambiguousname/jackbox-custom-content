// Form object for utility functions like not allowing submission if this form is not completed.
use gtk::{subclass::prelude::*, glib::{self, Value}, prelude::*};

// Huge thanks to the code from https://github.com/sdroege/gst-plugin-rs/blob/95c007953c0874bc46152078775d673cf44cc255/net/webrtc/src/signaller/iface.rs, otherwise I never would have figured this out myself. There are just way too many steps and there's not enough documentation.

use super::form_manager::FormManager;

mod imp {
	use gtk::glib::{Properties, ParamSpec, once_cell::sync::Lazy, ParamSpecBoolean, subclass::{prelude::*, Signal}};

	use super::*;

	#[derive(Clone, Copy)]
	#[repr(C)]
	pub struct FormObject {
		parent : glib::gobject_ffi::GTypeInterface,
		// Create the list of functions to be stored by our Interface definition in GTK GObject stuff:
		pub is_valid : fn(&super::FormObject) -> bool,
		pub value : fn(&super::FormObject) -> String,
	}
	
	// Default functions:
	impl FormObject {
		fn is_valid(_this : &super::FormObject) -> bool {
			true
		}
		fn value(_this : &super::FormObject) -> String {
			"".to_string()
		}
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

		// Signals so we can call our Impl functions from FormObjectExt.
		fn signals() -> &'static [Signal] {
			static SIGNALS : Lazy<Vec<Signal>> = Lazy::new(|| {
				vec![Signal::builder("is-valid").return_type::<bool>()
				// Class handlers connect the signals we get to the actual Impl functions we spent all this time assigning.
				.class_handler(|_token, args| {
					let this = args[0usize].get::<&super::FormObject>().unwrap_or_else(|e| {
						panic!("Wrong type for argument {}: {:?}", 0usize, e)
					});

					let vtable = this.interface::<super::FormObject>().unwrap();
					let vtable = vtable.as_ref();

					Some((vtable.is_valid)(this).to_value())
				}).build(), Signal::builder("value").return_type::<String>().build()]
			});
			SIGNALS.as_ref()
		}

		fn interface_init(&mut self) {
			self.is_valid = FormObject::is_valid;
			self.value = FormObject::value;
		}
	}
}

glib::wrapper!{
	pub struct FormObject(ObjectInterface<imp::FormObject>) @requires gtk::Widget;
}

// Impl definition for people to override.
pub trait FormObjectImpl: ObjectImpl + ObjectSubclass {
	fn is_valid(&self) -> bool;
	fn value(&self) -> String;
}

unsafe impl<T: FormObjectImpl> IsImplementable<T> for FormObject {
	// Assign our struct functions to their actual values (i.e., the Impl definitions from implementors)
	fn interface_init(iface: &mut glib::Interface<Self>) {
		let iface = ::std::convert::AsMut::as_mut(iface);

		fn is_valid_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject) -> bool {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			FormObjectImpl::is_valid(this)
		}
		iface.is_valid = is_valid_trampoline::<T>;

		fn value_trampoline<T: ObjectSubclass + FormObjectImpl>(obj : &FormObject) -> String {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			FormObjectImpl::value(this)
		}
		iface.value = value_trampoline::<T>;
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
		// let vtable = self.interface::<FormObject>().unwrap();
		// let vtable = vtable.as_ref();
		// (vtable.is_valid)(this)

		let valid : bool = self.emit_by_name("is-valid", &[]);
		valid
	}

	fn value(&self) -> String {
		let value : String = self.emit_by_name("value", &[]);
		value
	}
}

impl <T: IsA<FormObject> + IsA<gtk::Widget>> FormObjectExt for T {}