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
        pub window : OnceCell<ContentWindow>,
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
        ContentWindowImpl::create_content(&window.imp());
        window.present();
    }
}

mod window_imp {
    pub struct ContentWindow {}
}

// Easier than doing the subclassing route for now:
// pub trait ContentWindow {
//     fn ensure_all_types();
//     // Would be nice to figure this out somehow:
//     fn create_content(&self);
// }

mod content_window_imp {
    use super::*;

    #[derive(Default)]
    pub struct ContentWindow {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentWindow {
        const NAME: &'static str = "JCCContentWindow";
        type Type = super::ContentWindow;
        type ParentType = gtk::Window;
    }

    impl ObjectImpl for ContentWindow {}
    impl WidgetImpl for ContentWindow {}
    impl WindowImpl for ContentWindow {}
}

glib::wrapper! {
    pub struct ContentWindow(ObjectSubclass<content_window_imp::ContentWindow>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// impl ContentWindowExt for ContentWindow {
//     fn ensure_all_types() {
        
//     }
// }

pub trait ContentWindowImpl : WindowImpl {
    fn ensure_all_types();
    fn create_content(&self);
}

unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {}