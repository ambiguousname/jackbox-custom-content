use std::cell::RefCell;

// Template construction:
use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Application, CompositeTemplate, gio};
use glib::Object;

// Lists:
use gtk::{ColumnView, ColumnViewColumn, SingleSelection, SignalListItemFactory, ListItem};
use super::content::{contentobj::ContentObject, contentcol::ContentCol};
//use crate::templates::filebrowse::FileBrowseWidget;

// region: Boilerplate definitions
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

	// region: Boring Subclass Defs
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
	// endregion
}

glib::wrapper! {
	pub struct MainMenuWindow(ObjectSubclass<imp::MainMenuWindow>) @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
	@implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
// endregion

impl MainMenuWindow {
	pub fn new(app: &Application) -> Self {
		Object::builder().property("application", app).build()
	}
	
	pub fn add_content(&self){
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

	// region: Setup code (create list store and set up factories)
	fn setup_content_list(&self) {
		let model = gio::ListStore::new(ContentObject::static_type());

		self.imp().content_list.replace(Some(model));

		let content_list = SingleSelection::new(Some(&self.content_list()));
		self.imp().content_columns.set_model(Some(&content_list));
	}

	fn setup_factory(&self) {
		let columns = self.imp().content_columns.columns();
		let len = columns.n_items();
		for i in 0..len {
			let column = columns.item(i).and_downcast::<ColumnViewColumn>().expect("Column should be `ColumnViewColumn`.");
			
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
			
			column.set_factory(Some(&factory));
		}
	}
	// endregion
}