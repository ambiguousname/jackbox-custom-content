use gtk::{subclass::prelude::*, glib, prelude::*};
use glib::{Object, Properties, derived_properties};

use std::cell::OnceCell;

pub mod quiplash3;

// TODO: Can re-do this if there's a ensure type problem.
include!(concat!(env!("OUT_DIR"), "/content_list.rs"));

mod imp {
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::Content)]
    pub struct Content {
        #[property(get, set)]
        pub title : OnceCell<String>,

        #[property(get, set)]
        pub window_path : OnceCell<String>,

        #[property(get, set)]
        pub window : OnceCell<gtk::Window>,
    }

    #[glib::object_subclass]
	impl ObjectSubclass for Content {
		const NAME: &'static str = "JCCContent";
		type Type = super::Content;
		type ParentType = Object;
	}

    #[derived_properties]
	impl ObjectImpl for Content {}
}

glib::wrapper! {
    pub struct Content(ObjectSubclass<imp::Content>);
}

impl Content {
    pub fn ensure_all_types() {
        Content::ensure_type();
        ensure_types();
    }

    pub fn create_content(&self, parent : Option<&impl IsA<gtk::Window>>) {
        let window = self.window();
        window.set_transient_for(parent);
        window.present();
    }
}