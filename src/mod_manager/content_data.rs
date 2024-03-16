use std::cell::RefCell;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;

use crate::content::SubcontentBox;

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
		pub id : RefCell<u32>,

		/// The particular type of this content, set in the xml definition for a ContentWindow.
		#[property(get, set)]
		pub content_type : RefCell<String>,

		pub subcontent : RefCell<Vec<SubcontentBox>>,

		pub subcontent_args : RefCell<Vec<Vec<&'static str>>>,
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
	pub fn new(id : u32, full_id : String) -> Self{
		Object::builder()
		.property("enabled", true)
		.property("id", id)
		.property("full-id", full_id)
		.build()
	}

	pub fn set_subcontent(&self, subcontent: Vec<SubcontentBox>, args : Vec<Vec<&'static str>>) {
		if args.len() != subcontent.len().clone() {
			panic!("XML configuration args: {:?} and actual boxed subcontent {:?} do not match.", args, subcontent);
		}
		self.imp().subcontent.replace(subcontent);
		self.imp().subcontent_args.replace(args);
	}

	pub fn write_to_mod(&self) {
		let subcontent = self.imp().subcontent.borrow();
		let args = self.imp().subcontent_args.borrow();
		for i in 0..subcontent.len() {
			subcontent[i].write_to_mod(self.full_id(), args[i].clone());
		}
	}

	pub fn write_to_game(&self) {
		let subcontent = self.imp().subcontent.borrow();
		for d in subcontent.iter() {
			d.write_to_game();
		}
	}
}