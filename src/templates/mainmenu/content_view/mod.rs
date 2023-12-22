pub mod contentcol;
pub mod contentobj;

use contentobj::ContentObject;
use contentcol::ContentCol;

use gtk::{ColumnView, glib::derived_properties, ColumnViewColumn, gio, SignalListItemFactory, ListItem, SingleSelection, BuilderListItemFactory};

use std::{cell::{RefCell, OnceCell}, fs::{self, DirEntry}, path::PathBuf, io::Error};

use crate::quick_template;

quick_template!(ContentList, "/templates/mainmenu/content_view/contentlist.ui", gtk::Box, (gtk::Widget), (gtk::Orientable), props struct {
    #[template_child(id="column_view")]
    pub column_view : TemplateChild<ColumnView>,

	// TODO: Need some way to write the list store to JSON.
    #[property(get, set)]
    pub model : RefCell<Option<gio::ListStore>>,

	#[property(get, set)]
	pub name : OnceCell<String>,
	#[property(get, set)]
	pub id: OnceCell<String>,
});

#[derived_properties]
impl ObjectImpl for imp::ContentList {}
impl WidgetImpl for imp::ContentList {}
impl BoxImpl for imp::ContentList {}

impl ContentList {
    pub fn new(name : String) -> Result<Self, Error> {
		// Create mod folder:
		let mod_dir = PathBuf::from("./mods/").join(name.clone());
		if mod_dir.exists() {
			let msg = format!("Folder {name} already exists.");
			return Err(Error::new(std::io::ErrorKind::Other, msg));
		}
		fs::create_dir(mod_dir.clone())?;
		fs::create_dir(mod_dir.join("The Jackbox Party Pack 7"))?;

        ContentList::create(name)
    }

	pub fn from_folder(dir : DirEntry) -> Result<Self, Error> {
		ContentList::create(dir.file_name().into_string().expect("Could not get directory string."))
	}

	fn create(name : String) -> Result<Self, Error> {
		// We can clone the model however we want, the data stays the same.
		let model = gio::ListStore::new::<ContentObject>(); 
		let list : SingleSelection = SingleSelection::new(Some(model.clone()));

		// model.append(&ContentObject::new(false));
		/*// Uncomment to show:
		let view_clone = view.clone();

		model.append(&ContentObject::new(true));

		println!("{} {}", view_clone.model().unwrap().item(0).and_downcast::<ContentObject>().unwrap().enabled(), model.item(0).and_downcast::<ContentObject>().unwrap().enabled());
		*/

		let id = ContentList::string_to_id(name.clone());

		let this : ContentList = Object::builder()
		.property("model", model)
		.property("id", id)
		.property("name", name)
		.build();


		this.imp().column_view.set_model(Some(&list));
		this.setup_factory();

		Ok(this)
	}

	fn string_to_id(string : String) -> String {
        string.to_ascii_lowercase().replace(" ", "_")
    }

    fn setup_factory(&self) {
		let factory = BuilderListItemFactory::from_resource(None::<&gtk::BuilderScope>, "/templates/mainmenu/content_view/contentlistitem.ui");
		
        self.imp().column_view.set_row_factory(Some(&factory));
    }
}