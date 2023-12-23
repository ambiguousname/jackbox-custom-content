use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::{Object, Value, ParamSpec, ParamSpecBoolean, once_cell};
use once_cell::sync::Lazy;

// TODO: Use properties.
mod imp {
	use super::*;
	#[derive(Default)]
	pub struct ContentObject {
		// Allow this to be written to JSON?
		pub enabled : RefCell<bool>,
		// pub data: RefCell<Option<ContentData>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ContentObject {
		const NAME: &'static str = "JCCContentObject";
		type Type = super::ContentObject;
	}

	// region: Property definitions
	impl ObjectImpl for ContentObject {
		fn properties() -> &'static [ParamSpec] {
			static PROPERTIES : Lazy<Vec<ParamSpec>> = Lazy::new(|| {
				vec![
					ParamSpecBoolean::builder("enabled").build(),
				]
			});
			PROPERTIES.as_ref()
		}

		fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
			match _pspec.name() {
				"enabled" => {
					let input_value = _value.get().expect("Value should be of type `bool`.");
					self.enabled.replace(input_value);
				},
				_ => unimplemented!(),
			}
		}

		fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
			match _pspec.name() {
				"enabled" => self.enabled.borrow().to_value(),
				_ => unimplemented!(),
			}
		}
	}
	// endregion
}

glib::wrapper!{
	pub struct ContentObject(ObjectSubclass<imp::ContentObject>);
}

impl ContentObject {
	pub fn new(enabled: bool) -> Self{
		Object::builder()
		.property("enabled", enabled)
		.build()
	}

	pub fn enabled(&self) -> bool {
		self.property("enabled")
	}
}