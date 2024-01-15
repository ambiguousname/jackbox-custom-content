use gtk::{subclass::prelude::*, glib::{self, Value}, prelude::*};
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
	impl ObjectImpl for Content {
    }
}

glib::wrapper! {
    pub struct Content(ObjectSubclass<imp::Content>);
}


// TODO: Connet a create button with this and an imp trait:
type ContentCallback = fn();

impl Content {
    pub fn ensure_all_types() {
        Content::ensure_type();
        ensure_types();
    }

    pub fn create_content(&self, parent : Option<&impl IsA<gtk::Window>>, callback : Option<ContentCallback>) {
        let window = self.window();
        window.set_hide_on_close(true);
        window.set_transient_for(parent);
        if callback.is_some() {
            window.connect("content-created", false, move |values| {
                callback.unwrap()();
                None
            });
        }
        // ContentWindowImpl::create_content(&window.imp());
        // window.create_content(callback);
        window.present();
    }
}

// Because GTK's implementation in Rust is a nightmare to read (I don't know why you would migrate an object oriented framework to something like Rust), this is the solution I've come up with to try and ensure some kind of consistency across different windows:
mod content_window_imp {
    use gtk::{glib::{subclass::Signal, once_cell::sync::Lazy}, Button, HeaderBar};

    use super::*;

    
    // TODO: add a create button to this subclass so new windows don't have to call the signal themselves every time.
    // And make it a property.
    #[derive(Default)] //, Properties
    // #[properties(wrapper_type=super::ContentWindow)]
    pub struct ContentWindow {
    }

    // #[repr(C)]
    // pub struct ContentWindowClass<T: ObjectSubclass> {
    //     parent_class: <T::ParentType as ObjectType>::GlibClassType,
    //     pub content_created: Option<unsafe extern "C" fn(*mut ContentWindow)>,
    // }

    // unsafe impl<T : ObjectSubclass> ClassStruct for ContentWindowClass<T> {
    //     type Type = T;
    // }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentWindow {
        const NAME: &'static str = "JCCContentWindow";
        type Type = super::ContentWindow;
        type ParentType = gtk::Window;
        // type Class = ContentWindowClass<Self>;

        // fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            
        // }
    }

    // #[derived_properties]
    impl ObjectImpl for ContentWindow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("content-created").build()]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            self.parent_constructed();
            
            let create = Button::builder()
            .label("Create")
            .build();

            let header = HeaderBar::new();
            header.pack_end(&create);

            self.obj().set_property("child", &header);
        }
    }
    impl WidgetImpl for ContentWindow {}
    impl WindowImpl for ContentWindow {}
}

glib::wrapper! {
    pub struct ContentWindow(ObjectSubclass<content_window_imp::ContentWindow>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

// What would really be ideal is a way to set up some sort of trait that I can just implement or test for and be able to call like that.
// The issue there is that the signal system (or even setting up a callback) requires knowing types ahead of time for proper GObject storage.
// So this is what I'm stuck with for now.
// Experiments commented below in case anyone wants to try them.
pub trait ContentWindowImpl : WindowImpl {
    // fn content_created(&self) -> &[&dyn ToValue] {
    //     self.parent_content_created()
    // }
}

// pub trait ContentWindowImplExt : ObjectSubclass {
//     fn parent_content_created(&self) -> &[&dyn ToValue] {
//         &[]
//     }
// }

// impl<T: WindowImpl> ContentWindowImplExt for T {}

unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {
    // fn class_init(class: &mut glib::Class<Self>) {
    //     Self::parent_class_init::<T>(class);
        
        
    //     let klass = class.as_mut();
    //     klass.content_created = Some(content_created::<T>);
    // }
}

// unsafe extern "C" fn content_created<T: ContentWindowImpl>(ptr: *mut content_window_imp::ContentWindow) {
//     let instance = &*(ptr as *mut T::Instance);
    

//     let imp = instance.imp();
//     // imp.obj().emit_by_name::<()>("content-created", imp.content_created());
// }