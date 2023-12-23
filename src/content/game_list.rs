use crate::quick_template;
use glib::derived_properties;
use std::cell::OnceCell;
// Way to specify no extension? For subclass GObject directly.

mod game_imp {
	use std::cell::RefCell;

use super::*;

	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::Game)]
	pub struct Game {
		#[property(get, set)]
		pub title : RefCell<String>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for Game {
		const NAME: &'static str = "JCCGame";
		type Type = super::Game;
		type ParentType = Object;
	}

	#[derived_properties]
	impl ObjectImpl for Game {
		
	}
}

glib::wrapper! {
	pub struct Game(ObjectSubclass<game_imp::Game>);
}

impl Game {
	pub fn ensure_all_types() {
		Game::ensure_type();
	}
}

quick_template!(GameList, "/content/game_list.ui", gtk::Box, (gtk::Widget), (), struct {});

impl ObjectImpl for imp::GameList {}
impl WidgetImpl for imp::GameList {}
impl BoxImpl for imp::GameList {}

impl GameList {
	pub fn ensure_all_types() {
		GameList::ensure_type();
		Game::ensure_type();
	}
}