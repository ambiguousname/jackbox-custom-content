use gtk::{glib::{clone, derived_properties, Object, Properties}, ColumnView};

use std::{cell::{OnceCell, RefCell}, collections::HashMap, fs::{self, DirEntry}, path::PathBuf, io::Error};

use crate::{content::Content, quick_template};
use super::ContentData;

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

		pub content_data : RefCell<Vec<ContentData>>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::ModStore {}
impl WidgetImpl for imp::ModStore {}
impl BoxImpl for imp::ModStore {}

impl ModStore {
    fn new(name : String) -> Result<Self, Error> {
		let id = ModStore::string_to_id(name.clone());
		let this = Object::new::<Self>();
		this.imp().name.set(name).or_else(|err| {
			Err(Error::new(std::io::ErrorKind::Other, err))
		})?;
		this.imp().id.set(id).or_else(|err| {
			Err(Error::new(std::io::ErrorKind::Other, err))
		})?;

		Ok(this)
    }

	pub fn add_content(&self, content : Content) {
		content.create_content(clone!(@weak self as m => move |subcontent_type, subcontent| {
			// TODO: Write this.
			let content_data = m.imp().content_data.borrow();

			let id : String = m.id();
			let mod_id : String = content_data.len().to_string();

			let content_id = format!("{}_{}", id, mod_id);
			let new_content_data = ContentData::new(content_id.clone());
			for s in subcontent {
				s.write_to_mod(content_id.clone());
			}
		}));
	}

	pub fn create_content() {
		
	}

	pub fn new_folder(name : String) -> Result<Self, Error> {
		// Create mod folder:
		let mod_dir = PathBuf::from("./mods/").join(name.clone());
		if mod_dir.exists() {
			let msg = format!("Folder {name} already exists.");
			return Err(Error::new(std::io::ErrorKind::Other, msg));
		}
		fs::create_dir(mod_dir.clone())?;
		fs::create_dir(mod_dir.join("The Jackbox Party Pack 7"))?;

		ModStore::new(name)
	}

	pub fn from_folder(dir : DirEntry) -> Result<Self, Error> {
		ModStore::new(dir.file_name().into_string().expect("Could not get directory string."))
	}

	fn string_to_id(string : String) -> String {
        string.to_ascii_lowercase().replace(" ", "_")
    }
}