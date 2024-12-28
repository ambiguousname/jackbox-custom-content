use gtk::{
    gio::ListStore,
    glib::{derived_properties, Object, Properties},
    AlertDialog, ColumnView,
};
use serde_json::Value;

use std::{
    cell::{OnceCell, RefCell},
    fs::{self, File},
    io::{Error, ErrorKind},
    path::{Path, PathBuf},
};

use super::ContentData;
use crate::{content::{subcontent::manifest::ManifestItem, SubcontentBox}, quick_template};

quick_template!(ModStore, "/mod_manager/mod_store.ui", gtk::Box, (gtk::Widget), (gtk::Orientable),
    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type=super::ModStore)]
    struct {
        #[template_child(id="column_view")]
        pub column_view : TemplateChild<ColumnView>,

        /// Store of [`ContentData`]
        #[template_child(id="store")]
        pub store : TemplateChild<ListStore>,

        #[property(get)]
        pub name : OnceCell<String>,
        #[property(get)]
        pub id: OnceCell<String>,

        #[property(get)]
        /// The folder where this specific mod store is located.
        pub mod_folder : RefCell<PathBuf>,
    }
);

#[derived_properties]
impl ObjectImpl for imp::ModStore {}
impl WidgetImpl for imp::ModStore {}
impl BoxImpl for imp::ModStore {}

impl ModStore {
    /// * `name` - Name of this mod.
    /// * `mod_dir` - Folder where this particular mod is stored. Should be relative to the location of the executable.
    fn new(name: String, mod_dir: PathBuf) -> Result<Self, Error> {
        let id = ModStore::string_to_id(name.clone());
        let this = Object::new::<Self>();

        this.imp().mod_folder.replace(mod_dir);

        this.imp()
            .name
            .set(name)
            .or_else(|err| Err(Error::new(std::io::ErrorKind::Other, err)))?;
        this.imp()
            .id
            .set(id)
            .or_else(|err| Err(Error::new(std::io::ErrorKind::Other, err)))?;

        // Create the folder if it does not exist:
        if !this.mod_folder().exists() {
            fs::create_dir_all(this.mod_folder())?;
        } else {
            // Otherwise, load the manifest:
            this.read_manifest()?;
        }

        Ok(this)
    }

    /// If successful, add [`crate::content::Content`] to the ModStore. This will allow for things like merging the Content to the game folder.
    /// This should be called from [`super::ModManager::add_content_to_mod`] (which is in turn, called from the main menu.)
    pub fn add_content(
        &self,
        xml_def_path: String,
        content_type: String,
        subcontent: Vec<SubcontentBox>,
    ) {
        // Get arguments ready for Content construction.
        let subcontent_args: Vec<Vec<&'static str>> =
            crate::content::get_subcontent_args(&xml_def_path, &content_type);
        let content_data = &self.imp().store;

        // region: ID construction
        let mod_id = self.id();
        let id = content_data.n_items();
        let content_id = format!("{}_{}", mod_id.to_string(), id);
        // endregion

        // region: Folder creation
        let game_folder = crate::content::get_relative_folder(&xml_def_path);
        let mod_folder = self.mod_folder();

        // endregion

        // region: Make [`ContentData`]
        let new_content_data = ContentData::new(id, content_id.clone(), game_folder.into());

        new_content_data.set_subcontent(subcontent, subcontent_args);
        let res = new_content_data.write_to_mod(&mod_folder);

        if res.is_err() {
            let dlg = AlertDialog::builder()
                .message("Could not create content.")
                .detail(format!("Write operations failed: {}", res.unwrap_err()))
                .build();
            dlg.show(None::<&gtk::Window>);
            return;
        }
        // endregion

        // region: Add relevant information to the mod manifest
        let res = self.write_to_manifest(&new_content_data);
        if res.is_err() {
            let dlg = AlertDialog::builder()
                .message("Could not write content to manifest.json")
                .detail(format!("Write operations failed: {}", res.unwrap_err()))
                .build();
            dlg.show(None::<&gtk::Window>);
            return;
        }
        // endregion

        // Finally, push it to the ModStore:
        content_data.append(&new_content_data);
    }

    pub fn new_folder(base_mods_folder: &Path, name: String) -> Result<Self, Error> {
        // Create mod folder:
        let mod_dir = base_mods_folder.join(&name);
        ModStore::new(name, mod_dir)
    }

    pub fn from_folder(store_name: String, mod_dir: PathBuf) -> Result<Self, Error> {
        ModStore::new(store_name, mod_dir)
    }

    fn string_to_id(string: String) -> String {
        string.to_ascii_lowercase().replace(" ", "_")
    }

    pub fn read_manifest_content(&self, dat : &ContentData) -> std::io::Result<()> {
        self.imp().store.append(dat);
        Ok(())
    }

    pub fn read_manifest(&self) -> std::io::Result<()> {
        let mod_folder = self.mod_folder();
        let manifest_path = mod_folder.join("manifest.json");

        // Mods are allowed to exist without a manifest, we can create one from scratch.
        if !manifest_path.exists() {
            return Ok(());
        }

        let file = File::open(manifest_path)?;

        // We need to read the whole manifest, so we use Serde:
        let val : Value = serde_json::from_reader(file)?;

        let arr = val.as_array();

        if let None = arr {
            return Err(Error::new(ErrorKind::Other, format!("Could not read {val} as an array.")))
        }

        let arr = arr.unwrap();
        for val in arr {
            let dat = ContentData::deserialize(val.clone())?;
            self.read_manifest_content(&dat)?;
        }

        Ok(())
    }

    pub fn write_to_manifest(&self, data : &ContentData) -> Result<(), Error> {
        let mod_folder = self.imp().mod_folder.borrow();

        let item = ManifestItem::new(data.json_value()?);

        item.write_item(data.full_id(), &mod_folder, "manifest.json")
    }
}
