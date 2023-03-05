use std::cell::RefCell;
use std::rc::Rc;

use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

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

	impl ObjectImpl for ContentObject {}
}

glib::wrapper!{
	pub struct ContentObject(ObjectSubclass<imp::ContentObject>);
}