use crate::quick_template;
use glib::derived_properties;
use gtk::gio::ListModel;
use std::cell::OnceCell;

use super::Game;
// Way to specify no extension? For subclass GObject directly.

mod party_imp {

	use super::*;

	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::PartyPack)]
	pub struct PartyPack {
		// The game's display name.
		#[property(get, set)]
		pub title : OnceCell<String>,


		#[property(get, set)]
		pub children : OnceCell<Option<ListModel>>,
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

quick_template!(GameList, "/content/game_list.ui", gtk::Box, (gtk::Widget), (), struct {});

impl ObjectImpl for imp::GameList {}
impl WidgetImpl for imp::GameList {}
impl BoxImpl for imp::GameList {}

impl GameList {
	pub fn ensure_all_types() {
		GameList::ensure_type();
		PartyPack::ensure_all_types();
		Game::ensure_type();
	}
}