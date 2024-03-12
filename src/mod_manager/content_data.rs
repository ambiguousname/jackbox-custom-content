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
		#[property(get, set)]
		pub enabled : RefCell<bool>,

		/// The ID for this particular piece of content.
		#[property(get, set)]
		pub id : RefCell<String>,
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
	pub fn new(id : String) -> Self{
		Object::builder()
		.property("enabled", true)
		.property("id", id)
		.build()
	}
}