use gtk::{subclass::prelude::*, glib, prelude::*};
use glib::{Object, Properties, derived_properties};

use std::cell::OnceCell;

pub mod quiplash3;
pub mod game_list;

mod imp {
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::Game)]
    pub struct Game {
        #[property(get, set)]
        pub title : OnceCell<String>,
		// Internal ID (the relative folder from the /games/ directory)
		#[property(get, set)]
		pub id : OnceCell<String>,
    }

    #[glib::object_subclass]
	impl ObjectSubclass for Game {
		const NAME: &'static str = "JCCGame";
		type Type = super::Game;
		type ParentType = Object;
	}

    #[derived_properties]
	impl ObjectImpl for Game {}
}

glib::wrapper! {
    pub struct Game(ObjectSubclass<imp::Game>);
}

impl Game {
    pub fn ensure_all_types() {
        Game::ensure_type();
    }
}