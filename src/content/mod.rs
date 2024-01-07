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

    pub fn create_content(&self, parent : Option<&impl IsA<gtk::Window>>, callback : Option<fn()>) {
        let window = self.window();
        window.set_hide_on_close(true);
        window.set_transient_for(parent);
        // ContentWindowImpl::create_content(&window.imp());
        // window.create_content(callback);
        window.present();
    }
}

// Because GTK's implementation in Rust is a nightmare to read (I don't know why you would migrate an object oriented framework to something like Rust), this is the solution I've come up with to try and ensure some kind of consistency across different windows:
mod content_window_imp {
    use std::cell::RefCell;
    use super::*;

    #[derive(Default, Properties)]
    #[properties(wrapper_type=super::ContentWindow)]
    pub struct ContentWindow {
        #[property(get, set)]
        pub content : RefCell<String>,

        // FIXME: Hacky workaround for me not being able to figure out subclassing. Or properties.
        // All I can really get working properly is the Ext trait, so we're using what works.
        pub create_content_callback : RefCell<Option<fn()>>,
    }

    // #[repr(C)]
    // pub struct ContentWindowClass<T: ObjectSubclass> {
    //     parent_class: <T::ParentType as ObjectType>::GlibClassType,
    //     pub on_content_create: Option<unsafe extern "C" fn(*mut ContentWindow)>,
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
    }

    #[derived_properties]
    impl ObjectImpl for ContentWindow {
        // fn signals() -> &'static [Signal] {
        //     static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
        //         vec![Signal::builder("create-finished").build()]
        //     });
        //     SIGNALS.as_ref()
        // }
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
pub trait ContentWindowImpl : WindowImpl {
    // fn on_content_create(&self) {
    //     self.parent_on_content_create();
    // }
}

// pub trait ContentWindowImplExt : ObjectSubclass {
//     fn parent_on_content_create(&self) {
        
//     }
// }

// impl<T: WindowImpl> ContentWindowImplExt for T {}

unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {
    // fn class_init(class: &mut glib::Class<Self>) {
    //     Self::parent_class_init::<T>(class);
        
        
    //     let klass = class.as_mut();
    //     klass.on_content_create = Some(on_content_create::<T>);
    // }
}

// FIXME: I hate this. So much.
// More hacky workarounds.
// I really wish I could figure out subclassing for Glib.
pub trait ContentWindowExt : IsA<ContentWindow> + 'static {
    fn create_content(&self, callback : Option<fn()>) {
        let window = self.as_ref();

        window.imp().create_content_callback.replace(callback);
        window.present();
    }

    fn finish_create_content(&self) {
        let func = self.as_ref().imp().create_content_callback.borrow();
        if func.is_some() {
            func.unwrap()();
        }
    }
}

// unsafe extern "C" fn on_content_create<T: ContentWindowImpl>(ptr: *mut content_window_imp::ContentWindow) {
//     let instance = &*(ptr as *mut T::Instance);
//     let imp = instance.imp();

//     imp.on_content_create();
// }

impl <O: IsA<ContentWindow>> ContentWindowExt for O {}