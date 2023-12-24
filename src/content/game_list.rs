use crate::quick_template;
use glib::derived_properties;
use gtk::gio::{ListModel, ListStore};
use std::cell::RefCell;

use super::Game;
// Way to specify no extension? For subclass GObject directly.

mod party_imp {

	use super::*;

	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::PartyPack)]
	pub struct PartyPack {
		// The game's display name.
		#[property(get, set)]
		pub title : RefCell<String>,


		#[property(get, set)]
		pub children : RefCell<Option<ListModel>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for PartyPack {
		const NAME: &'static str = "JCCPartyPack";
		type Type = super::PartyPack;
		type ParentType = Object;
	}

	#[derived_properties]
	impl ObjectImpl for PartyPack {
		
	}
}

glib::wrapper! {
	pub struct PartyPack(ObjectSubclass<party_imp::PartyPack>);
}

impl PartyPack {
	pub fn ensure_all_types() {
		PartyPack::ensure_type();
	}
}

quick_template!(GameList, "/content/game_list.ui", gtk::Box, (gtk::Widget), (), struct {
	#[template_child(id="game_select_model")]
	pub model: TemplateChild<gtk::SingleSelection>,
});

impl ObjectImpl for imp::GameList {
	fn constructed(&self) {
		self.parent_constructed();
		
		let obj = self.obj();
		obj.setup_model();
	}
}
impl WidgetImpl for imp::GameList {}
impl BoxImpl for imp::GameList {}

impl GameList {
	pub fn ensure_all_types() {
		GameList::ensure_type();
		PartyPack::ensure_all_types();
		Game::ensure_type();
	}

	fn setup_model(&self) {
		let data : ListStore = gtk::Builder::from_resource("/content/content_list.ui").object("content_list").expect("Could not get store.");
		let tree = gtk::TreeListModel::new(data, false, true, |item| {
			let party_pack : PartyPack = item.clone().downcast().expect("Could not get party pack item.");

			if party_pack.children().is_some() {
				party_pack.children()
			} else {
				None
			}
		});
		self.imp().model.set_model(Some(&tree));
	}
}