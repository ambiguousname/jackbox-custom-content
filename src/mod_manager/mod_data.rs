use std::cell::RefCell;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;

// TODO: Use properties.
mod imp {
	use super::*;
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::ModData)]
	pub struct ModData {
		// Allow this to be written to JSON?
		#[property(get, set)]
		pub enabled : RefCell<bool>,
		// pub data: RefCell<Option<ContentData>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ModData {
		const NAME: &'static str = "JCCModData";
		type Type = super::ModData;
	}

	impl ObjectImpl for ModData {}
}

glib::wrapper!{
	pub struct ModData(ObjectSubclass<imp::ModData>);
}

impl ModData {
	pub fn new(enabled: bool) -> Self{
		Object::builder()
		.property("enabled", enabled)
		.build()
	}
}