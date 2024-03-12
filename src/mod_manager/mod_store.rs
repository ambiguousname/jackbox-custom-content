use gtk::{ColumnView, glib::{Properties, derived_properties, Object}};

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
		let content_data = self.imp().content_data.borrow();
		content.create_content(|subcontent_type, subcontent| {
			// TODO: Write this.
			/*let id : String = self.id();
			let mod_id : String = content_data.len();
			let new_content_data = ContentData::new(format!("{}_{}", id, mod_id));
			for s in subcontent {
				s.write_to_mod(content_id.clone());
			}*/
		});
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