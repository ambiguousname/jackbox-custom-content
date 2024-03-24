use gtk::{glib::{clone, derived_properties, Object, Properties}, AlertDialog, ColumnView};

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

	pub fn add_content(&self, content : crate::content::Content) {
		let opt = content.xml_definition();
		let xml_def = std::rc::Rc::new(opt);
		content.create_content(clone!(@weak self as m => move |content_type, subcontent| {
			let subcontent_args : Vec<Vec<&'static str>> = crate::content::get_subcontent_args(xml_def.to_string(), content_type.clone());

			let mut content_data = m.imp().content_data.borrow_mut();

			let mod_id = m.id();
			let id_try = content_data.len().try_into();

			if id_try.is_err() {
				let dlg = AlertDialog::builder().message("Could not create content.").detail(format!("ID of {}_{} could not be created.", mod_id, content_data.len())).build();
				dlg.show(None::<&gtk::Window>);
				return;
			}
			let id = id_try.unwrap();

			let content_id = format!("{}_{}", id, mod_id.to_string());
			let new_content_data = ContentData::new(id, content_id.clone());

			new_content_data.set_subcontent(subcontent, subcontent_args);
			let res = new_content_data.write_to_mod();

			if res.is_err() {
				let dlg = AlertDialog::builder().message("Could not create content.").detail(format!("Write operations failed: {}", res.unwrap_err())).build();
				dlg.show(None::<&gtk::Window>);
				return;
			}
			
			content_data.push(new_content_data);
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