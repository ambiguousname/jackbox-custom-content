use std::cell::RefCell;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;

// TODO: Use properties.
mod imp {
	use super::*;
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::ContentData)]
	pub struct ContentData {
		// Allow this to be written to JSON?
		#[property(get, set)]
		pub enabled : RefCell<bool>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ContentData {
		const NAME: &'static str = "JCCContentData";
		type Type = super::ContentData;
	}

	impl ObjectImpl for ContentData {}
}

glib::wrapper!{
	pub struct ContentData(ObjectSubclass<imp::ContentData>);
}

impl ContentData {
	pub fn new(enabled: bool) -> Self{
		Object::builder()
		.property("enabled", enabled)
		.build()
	}
}