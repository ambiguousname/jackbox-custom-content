use std::cell::RefCell;

// Template construction:
use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Application, CompositeTemplate, gio};
use glib::Object;

// Lists:
use gtk::{ColumnView, ColumnViewColumn, SingleSelection, SignalListItemFactory, ListItem};
use super::{contentobj::ContentObject, contentcol::ContentCol};
//use crate::templates::filebrowse::FileBrowseWidget;

mod imp {
	use super::*;

	#[derive(Default, CompositeTemplate)]
	#[template(resource="/templates/windows/mainmenu.ui")]
	pub struct MainMenuWindow {
		// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
		#[template_child(id="content_columns")]
		pub content_columns: TemplateChild<ColumnView>,
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
			obj.setup_factory();
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
	
	pub fn new_content(&self){
		let test_content = ContentObject::new(false);
		self.content_list().append(&test_content);
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

		let content_list = SingleSelection::new(Some(&self.content_list()));
		self.imp().content_columns.set_model(Some(&content_list));
	}

	fn setup_factory(&self) {
		let factory = SignalListItemFactory::new();

		factory.connect_setup(move |_, list_item| {

			let widget = gtk::Label::new(Some("Test"));
			let content_row = ContentCol::new(gtk::Widget::from(widget));
			list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
			.set_child(Some(&content_row));
		});

		factory.connect_bind(move |_, list_item| {
			let content_object = list_item.downcast_ref::<ListItem>()
				.expect("Should be ListItem")
				.item()
				.and_downcast::<ContentObject>()
				.expect("Item should be `ContentObject`.");

			let content_row = list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
			.child()
			.and_downcast::<ContentCol>().expect("Child should be `ContentCol`.");

			content_row.bind(&content_object);
		});

		factory.connect_unbind(move |_, list_item| {
			let content_row = list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
			.child()
			.and_downcast::<ContentCol>().expect("Child should be `ContentCol`.");

			content_row.unbind();
		});
		let enabled_col = ColumnViewColumn::new(Some("enabled"), Some(&factory));
		let game_col = ColumnViewColumn::new(Some("Game"), Some(&factory));

		self.imp().content_columns.insert_column(0, &enabled_col);
		self.imp().content_columns.insert_column(1, &game_col);
	}
}