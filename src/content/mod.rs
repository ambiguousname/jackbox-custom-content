use gtk::{subclass::prelude::*, glib::{self, Value, Type, clone}, prelude::*, gio::{SimpleActionGroup, ActionEntry}};
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

type ContentCallback = fn(String);

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
                let window : ContentWindow = values[0].get().expect("Could not get ContentWindow");
                window.close();

                callback.unwrap()(values[1].get().unwrap());
                None
            });
        }
        window.present();
    }
}

// Because GTK's implementation in Rust is a nightmare to read (I don't know why you would migrate an object oriented framework to something like Rust), this is the solution I've come up with to try and ensure some kind of consistency across different windows:
mod content_window_imp {
    use std::cell::RefCell;

    use gtk::glib::{subclass::Signal, once_cell::sync::Lazy};

    use crate::templates::content_util::form::FormObject;

    use super::*;

    
    // TODO: add a create button to this subclass so new windows don't have to call the signal themselves every time.
    // And make it a property.
    #[derive(Default)] //, Properties
    // #[properties(wrapper_type=super::ContentWindow)]
    pub struct ContentWindow {
        pub form_objects : RefCell<Vec<FormObject>>,
    }

    // #[repr(C)]
    // pub struct ContentWindowClass<T: ObjectSubclass> {
    //     parent_class: <T::ParentType as ObjectType>::GlibClassType,
    //     pub content_created: Option<unsafe extern "C" fn(*mut ContentWindow)-> &'static [&'static dyn ToValue]>,
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

    // #[derived_properties]
    impl ObjectImpl for ContentWindow {
        fn signals() -> &'static [Signal] {
            // Easiest way I can imagine handling things without knowing the gtk-rs library front and back.
            // FIXME: If there's any way someone can instead set this up with the signal experiments below, as well as minimizing the amount of boilerplate on a user's end (i.e., just a function you can override instead of making your own button and hooking that up), you would be my hero.
            // I dunno, maybe this works better if people can set up their own windows. But I'd sort of like some sort of template consistency, so the design can be somewhat uniform.
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("content-created").param_types([String::static_type()]).build()]
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

impl ContentWindow {}

// So, I learned that this trait system is actually for creating signals.
// I would really love for there to be some sort of designer friendly way to hook up signals, but this is what we've got.
// Experiments commented below in case anyone wants to try them.
pub trait ContentWindowImpl : WindowImpl {
    // fn content_created(&self) -> &[&dyn ToValue] {
    //     self.parent_content_created()
    // }
}

// pub trait ContentWindowImplExt : ObjectSubclass {
//     fn parent_content_created(&self) -> &[&dyn ToValue] {
//         unsafe {
//             let data = Self::type_data();
//             let parent_class = data.as_ref().parent_class() as *mut content_window_imp::ContentWindowClass<Self>;
//             // let f = (*parent_class)
//             //     .content_created
//             //     .expect("No parent class impl for \"content_created\"");
//             // f(self.obj().unsafe_cast_ref::<ContentWindow>().to_glib_none().0)
//         }
//     }
// }

// impl<T: ContentWindowImpl> ContentWindowImplExt for T {}

unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {
    // fn class_init(class: &mut glib::Class<Self>) {
    //     Self::parent_class_init::<T>(class);
        
        
    //     let klass = class.as_mut();
    //     klass.content_created = Some(content_created::<T>);
    // }
}

// unsafe extern "C" fn content_created<T: ContentWindowImpl>(ptr: *mut content_window_imp::ContentWindow) -> &'static[&'static dyn ToValue] {
//     let instance = &*(ptr as *mut T::Instance);
    

//     let imp = instance.imp();
//     imp.content_created()
// }