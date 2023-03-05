use std::cell::RefCell;

use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Application, CompositeTemplate, ColumnView, ListView, NoSelection, gio};
use glib::Object;
//use crate::templates::filebrowse::FileBrowseWidget;
use super::contentobj::ContentObject;

mod imp {
	use super::*;

	#[derive(Default, CompositeTemplate)]
	#[template(resource="/templates/windows/mainmenu.ui")]
	pub struct MainMenuWindow {
		// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
		#[template_child(id="content_columns")]
		pub content_columns: TemplateChild<ColumnView>,
		#[template_child(id="content_list")]
		pub content_list_ui: TemplateChild<ListView>,
		pub content_list: RefCell<Option<gio::ListStore>>, 
	}

	#[glib::object_subclass]
	impl ObjectSubclass for MainMenuWindow {
		const NAME: &'static str = "JCCMainMenuWindow";
		type Type = super::MainMenuWindow;
		type ParentType = gtk::ApplicationWindow;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
	}

	impl ObjectImpl for MainMenuWindow {
		fn constructed(&self) {
			self.parent_constructed();

			let obj = self.obj();
			obj.setup_content_list();
		}
	}
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

	fn content_list(&self) -> gio::ListStore {
		self.imp()
			.content_list
			.borrow()
			.clone()
			.expect("Could not get content_list")
	}

	fn setup_content_list(&self) {
		let model = gio::ListStore::new(ContentObject::static_type());

		self.imp().content_list.replace(Some(model));

		let content_list = NoSelection::new(Some(&self.content_list()));
		self.imp().content_list_ui.set_model(Some(&content_list));
		self.imp().content_columns.set_model(Some(&content_list));
	}
}