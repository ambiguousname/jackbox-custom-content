//! A combination of two "classes" (GObject classes, a whole lot of Rust structs and interfaces):
//! Content - ListItem to be displayed in templates/mainmenu/content_creation (i.e., when you click to create a new piece of content). Lots of properties for relevant display information.
//! ContentWindow - A subclassable window that implements methods for creating content according to the Jackbox system.

use gtk::{subclass::prelude::*, glib, prelude::*};
use glib::{Object, Properties, derived_properties};

use std::cell::OnceCell;

use self::subcontent::Subcontent;

pub mod quiplash3;
pub mod subcontent;

// There's an automation in /build/content_list.rs. It auto generates this file based on the content/content_list.ui XML file (instructions on how to integrate it with that automation are in there) to avoid errors when including any of these windows in that very XML definition (mostly by calling ensure_all_types, as defined in the ContentWindowImpl trait.)
// Can re-do this if there's a ensure type problem.
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

        /// In the XML definition (content_list.ui), every piece of Content has an associated window property. This makes it much easier to quickly set up a definition for a window, then add it to the list of existing content. Less work for devs, but more work in the background.
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

/// Direct function that the mod manager uses to organize files after creation.
/// TODO: Actually get this working as intended.
type ContentCallback = fn(String);

impl Content {
    pub fn ensure_all_types() {
        Content::ensure_type();
        // Calls the function created in the include! call above.
        ensure_types();
    }

    /// Creates the content window from the properties and sets up the appropriate [`ContentCallback`] to the window. 
    pub fn create_content(&self, callback : Option<ContentCallback>) {
        let window = self.window();
        window.set_hide_on_close(true);
        window.create_content_window(callback);
        window.present();
    }
}

mod content_window_imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Default)]
    pub struct ContentWindow {
        /// The callback set by [`Content::create_content`]. 
        pub content_callback : RefCell<Option<ContentCallback>>,
    }

    /// The struct used for virtual functions. You should override this in your custom ContentWindow extension (see [`quiplash3::prompts::QuiplashRoundPrompt`] for an example of this.)
    #[repr(C)]
    pub struct ContentWindowClass<T: ObjectSubclass> {
        parent_class: <T::ParentType as ObjectType>::GlibClassType,
        /// Called by the ContentWindow itself (although indirectly), for when content creation is done and it's ready to pass along info to the callback.
        /// This is sort of an intermediary between [`ContentWindowImpl::finalize_content`] and [`ContentWindow`]'s call of it. This will pass along the callback to [`ContentWindowImpl`] and call it.
        /// Set in [`IsSubclassable<T: ContentWindowImpl>::class_init`]
        pub finalize_content : fn(&super::ContentWindow),

        /// List of the [`Subcontent`] being implemented.
        pub subcontent : fn() -> &'static [SubcontentBox],
    }

    /// Custom class structure to be able to use [`ContentWindowClass`]
    /// Sets the default callback for [`ContentWindowClass<T>::finalize_content`]
    unsafe impl<T : ObjectSubclass> ClassStruct for ContentWindowClass<T> {
        type Type = T;
    }

    /// Default implementations. Nothing special here.
    impl ContentWindow {
        fn finalize_content(_this : &super::ContentWindow) {}
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentWindow {
        const NAME: &'static str = "JCCContentWindow";
        type Type = super::ContentWindow;
        type ParentType = gtk::Window;
        type Class = ContentWindowClass<Self>;
        fn class_init(klass: &mut Self::Class) {
            klass.finalize_content = ContentWindow::finalize_content;
        }
    }

    // #[derived_properties]
    impl ObjectImpl for ContentWindow {}
    impl WidgetImpl for ContentWindow {}
    impl WindowImpl for ContentWindow {}
}

glib::wrapper! {
    pub struct ContentWindow(ObjectSubclass<content_window_imp::ContentWindow>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ContentWindow {}

type SubcontentBox = Box<dyn Subcontent + Send + Sync>;

/// The actual impl definition for any [`ContentWindow`] subclasser to override.
pub trait ContentWindowImpl : WindowImpl {
    /// Whenever [`ContentWindow`] has finished creating content and is ready to pass along the relevant data for the mod manager, call [`ContentWindowExt::finalize_content`] and this will be called with the appropriate callback.
    /// Automatically closes the window.
    fn finalize_content(&self, callback : Option<ContentCallback>);

    fn subcontent() -> &'static [SubcontentBox];
}

/// Assigns the actual functions to be called (this is mostly based on templates/content_util/form.rs, as well as https://github.com/sdroege/gst-plugin-rs/blob/95c007953c0874bc46152078775d673cf44cc255/net/webrtc/src/signaller/iface.rs).
unsafe impl<T: ContentWindowImpl> IsSubclassable<T> for ContentWindow {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        
        let klass = class.as_mut();

        /// Grab the callback from [`content_window_imp::ContentWindow::content_callback`] and then call [`ContentWindowImpl::finalize_content`] with that callback.
        /// Will also automatically close the window for you.
        fn finalize_content_trampoline<T: ObjectSubclass + ContentWindowImpl>(obj : &ContentWindow) {
            let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();

            let imp = obj.imp();
            let content_callback = imp.content_callback.borrow().clone();
            T::finalize_content(this, content_callback);
            obj.close();
        }
        klass.finalize_content = finalize_content_trampoline::<T>;

        fn subcontent_trampoline<T: ObjectSubclass + ContentWindowImpl>() -> &'static [SubcontentBox] {
            T::subcontent()
        }
        klass.subcontent = subcontent_trampoline::<T>;
    }
}

/// The outward facing functions.
pub trait ContentWindowExt : IsA<ContentWindow> + 'static {
    /// Called by [`Content::create_content`], sets up the callback.
    fn create_content_window(&self, callback : Option<ContentCallback>) {
        let window = self.upcast_ref::<ContentWindow>();

        window.imp().content_callback.replace(callback);
    }

    /// Should be called by [`ContentWindow`] when the window is done, will call [`content_window_imp::ContentWindowClass<T>::finalize_content`] as an intermediary step.
    /// This will automatically close the window.
    fn finalize_content(&self) {
        let window = self.upcast_ref::<ContentWindow>();
        let klass = window.class().as_ref();

        (klass.finalize_content)(window);
    }

    fn subcontent(&self) -> &'static [SubcontentBox] {
        let window = self.upcast_ref::<ContentWindow>();
        let klass = window.class().as_ref();

        (klass.subcontent)()
    }
}

/// Just exposes ContentWindowExt for everybody.
impl <T: IsA<ContentWindow>> ContentWindowExt for T {}