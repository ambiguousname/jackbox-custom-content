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
        // ContentWindowImpl::create_content(&window.imp());
        window.present();
    }
}

// Because GTK's implementation in Rust is a nightmare to read (I don't know why you would migrate an object oriented framework to something like Rust), this is the solution I've come up with to try and ensure some kind of consistency across different windows:
mod content_window_imp {
    use std::cell::RefCell;

    use gtk::glib::{subclass::Signal, once_cell::sync::Lazy};

    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::ContentWindow)]
    pub struct ContentWindow {
        #[property(get, set)]
        pub content : RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentWindow {
        const NAME: &'static str = "JCCContentWindow";
        type Type = super::ContentWindow;
        type ParentType = gtk::Window;
    }

    #[derived_properties]
    impl ObjectImpl for ContentWindow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("create-finished").build()]
            });
            SIGNALS.as_ref()
        }
    }
    impl WidgetImpl for ContentWindow {}
    impl WindowImpl for ContentWindow {}
}

glib::wrapper! {
    pub struct ContentWindow(ObjectSubclass<content_window_imp::ContentWindow>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// What would really be ideal is a way to set up some sort of trait that I can just implement or test for and be able to call like that.
// Ideal might be to replace this with an interface (but that's even harder for me to figure out).
// So this is what I'm stuck with for now.
pub trait ContentWindowImpl : WindowImpl {}

unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {}