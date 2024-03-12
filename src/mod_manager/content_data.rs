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
		pub full_id : RefCell<String>,

		/// The number for this piece of content.
		#[property(get, set)]
		pub id : RefCell<i32>,

		/// The particular type of this content, set in the xml definition.
		#[property(get, set)]
		pub content_type : RefCell<String>,
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
	pub fn new(id : i32, full_id : String) -> Self{
		Object::builder()
		.property("enabled", true)
		.property("id", id)
		.build()
	}
}