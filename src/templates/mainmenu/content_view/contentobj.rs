use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::{Object, Value, ParamSpec, ParamSpecBoolean, once_cell};
use once_cell::sync::Lazy;

#[derive(Default)]
pub struct ContentData {
	pub enabled: bool,
	pub game_name: String,
	pub content_type: String,
}

mod imp {
	use super::*;
	#[derive(Default)]
	pub struct ContentObject {
		pub data: Rc<RefCell<ContentData>>,
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
					self.data.borrow_mut().enabled = input_value;
				},
				_ => unimplemented!(),
			}
		}

		fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
			match _pspec.name() {
				"enabled" => self.data.borrow().enabled.to_value(),
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