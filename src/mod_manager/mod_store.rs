use gtk::{ColumnView, glib::{Properties, derived_properties, Object}, ColumnViewColumn, gio, SignalListItemFactory, ListItem, SingleSelection, BuilderListItemFactory};

use std::{cell::{RefCell, OnceCell}, fs::{self, DirEntry}, path::PathBuf, io::Error};

use crate::quick_template;

quick_template!(ModStore, "/mod_manager/mod_store.ui", gtk::Box, (gtk::Widget), (gtk::Orientable),
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::ModStore)]
	struct {
		#[template_child(id="column_view")]
		pub column_view : TemplateChild<ColumnView>,

		// TODO: Need some way to write the list store to JSON.

		#[property(get)]
		pub name : OnceCell<String>,
		#[property(get)]
		pub id: OnceCell<String>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::ModStore {}
impl WidgetImpl for imp::ModStore {}
impl BoxImpl for imp::ModStore {}

impl ModStore {
    pub fn new(name : String) -> Result<Self, Error> {
		// Create mod folder:
		let mod_dir = PathBuf::from("./mods/").join(name.clone());
		if mod_dir.exists() {
			let msg = format!("Folder {name} already exists.");
			return Err(Error::new(std::io::ErrorKind::Other, msg));
		}
		fs::create_dir(mod_dir.clone())?;
		fs::create_dir(mod_dir.join("The Jackbox Party Pack 7"))?;
		
		let id = ModStore::string_to_id(name.clone());

		let this = Object::new::<Self>();
		this.imp().name.set(name);
		this.imp().id.set(id);

		Ok(this)
    }

	pub fn from_folder(dir : DirEntry) -> Result<Self, Error> {
		ModStore::new(dir.file_name().into_string().expect("Could not get directory string."))
	}

	fn string_to_id(string : String) -> String {
        string.to_ascii_lowercase().replace(" ", "_")
    }
}