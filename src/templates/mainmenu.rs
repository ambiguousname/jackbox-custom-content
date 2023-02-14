use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, Application, CompositeTemplate, gio};
use glib::Object;
use crate::templates::filebrowse::FileBrowseWidget;

mod imp {
	use super::*;

	#[derive(Default, CompositeTemplate)]
	#[template(resource="/templates/windows/mainmenu.ui")]
	pub struct MainMenuWindow {
		#[template_child(id="mainfilebrowse")]
		// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
		pub file_browse: TemplateChild<FileBrowseWidget>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for MainMenuWindow {
		const NAME: &'static str = "MainMenuWindow";
		type Type = super::MainMenuWindow;
		type ParentType = gtk::ApplicationWindow;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
	}

	impl ObjectImpl for MainMenuWindow {}
    impl WidgetImpl for MainMenuWindow {}
	impl WindowImpl for MainMenuWindow {}
	impl ApplicationWindowImpl for MainMenuWindow {}
}

glib::wrapper! {
	pub struct MainMenuWindow(ObjectSubclass<imp::MainMenuWindow>) @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
	@implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MainMenuWindow {
	pub fn new(app: &Application) -> Self {
		Object::builder().property("application", app).build()
	}
}