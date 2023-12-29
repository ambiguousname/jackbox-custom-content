use gtk::{subclass::prelude::*, glib, prelude::*};
use glib::Object;
use glib::{Properties, derived_properties};
use gtk::gio::ListModel;
use std::cell::{RefCell, OnceCell};

// Way to specify no extension? For subclass GObject directly.

mod imp {

	use super::*;

	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::GameListItem)]
	pub struct GameListItem {
		// The display name of the item.
		#[property(get, set)]
		pub title : OnceCell<String>,

		#[property(get, set)]
		pub children : RefCell<Option<ListModel>>,

		#[property(get, set)]
		pub content : RefCell<Option<ListModel>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for GameListItem {
		const NAME: &'static str = "JCCGameListItem";
		type Type = super::GameListItem;
		type ParentType = Object;
	}

	#[derived_properties]
	impl ObjectImpl for GameListItem {
		
	}
}

glib::wrapper! {
	pub struct GameListItem(ObjectSubclass<imp::GameListItem>);
}

impl GameListItem {
	pub fn ensure_all_types() {
		GameListItem::ensure_type();
	}

	pub fn get_title(&self) -> String {
		self.title()
	}
}