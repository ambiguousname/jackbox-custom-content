use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;
use serde::{Deserialize, Serialize};

use crate::content::SubcontentBox;

mod imp {
    use std::sync::OnceLock;

    use glib::{property::PropertySet, ParamSpec, ParamSpecBoolean, ParamSpecString};

    use super::*;

    #[derive(Default, Serialize, Deserialize, Clone)]
    pub(super) struct ContentDataInner {
        pub enabled : bool,
        /// The ID for this particular piece of content.
        pub full_id : String,

        /// The number for this piece of content.
        #[serde(rename="num_id")]
        pub id : u32,

        /// The particular type of this content, set in the xml definition for a ContentWindow.
        pub content_type: String,

        /// The relative path where this content is stored. This is relative from the mod folder, specifically.
        pub relative_path: PathBuf,
    }

    /// Data for how to write a given [`crate::content::Content`] type to disk.
    /// Serialized mostly for `manifest.json` that [`crate::mod_manager::mod_store::ModStore`] writes to.
    #[derive(Default, Serialize, Deserialize)]
    pub struct ContentData {
        pub(super) data_inner : RefCell<ContentDataInner>,

        /// Store for [`crate::content::Subcontent`], used to invoke various Subcontent functions for writing to and loading from disk.
        #[serde(skip)]
        pub subcontent: RefCell<Vec<SubcontentBox>>,

        /// The arguments used when calling `subcontent` functions.
        #[serde(skip)]
        pub subcontent_args: RefCell<Vec<Vec<&'static str>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentData {
        const NAME: &'static str = "CustomBoxContentData";
        type Type = super::ContentData;
    }

    impl ObjectImpl for ContentData {
        fn properties() -> &'static [ParamSpec] {
           static PROPERTIES : OnceLock<Vec<ParamSpec>> = OnceLock::new();

           PROPERTIES.get_or_init(|| {
                vec![
                    ParamSpecBoolean::builder("enabled").readwrite().build(),
                    ParamSpecString::builder("id").readwrite().build()
                ]
           })
        }

        fn set_property(&self, _: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            let name = pspec.name();
            match name {
                "enabled" => self.data_inner.borrow_mut().enabled = value.get().unwrap(),
                "id" => self.data_inner.borrow_mut().id = value.get().unwrap(),
                _ => panic!("Property {name} setter not defined.")
            }
        }

        fn property(&self, _: usize, pspec: &glib::ParamSpec) -> glib::Value {
            let name = pspec.name();
            match name {
                "enabled" => self.data_inner.borrow().enabled.into(),
                "id" => self.data_inner.borrow().id.into(),
                _ => panic!("Property {name} getter not defined.")
            }
        }
    }
}

glib::wrapper! {
    pub struct ContentData(ObjectSubclass<imp::ContentData>);
}

impl ContentData {
    // TODO: Write ContentData on creation or modification to a manifest.
    pub fn new(id: u32, full_id: String, relative_path: PathBuf) -> Self {
        let this : Self = Object::new();

        let obj_ref = this.clone();
        let mut data_inner = obj_ref.imp().data_inner.borrow_mut();
        data_inner.id = id;
        data_inner.full_id = full_id;
        data_inner.relative_path = relative_path;
        
        this
    }

    fn data_inner(&self) -> imp::ContentDataInner {
        self.imp().data_inner.borrow().clone()
    }

    pub fn deserialize(val : serde_json::Value) -> Result<Self, serde_json::Error> {
        let inner : imp::ContentDataInner = serde_json::from_value(val)?;

        let new : Self = Object::new();
        
        new.imp().data_inner.replace(inner);

        Ok(new)
    }

    pub fn full_id(&self) -> String {
        self.data_inner().full_id
    }

    pub fn relative_path(&self) -> PathBuf {
        self.data_inner().relative_path
    }

    pub fn set_subcontent(&self, subcontent: Vec<SubcontentBox>, args: Vec<Vec<&'static str>>) {
        if args.len() != subcontent.len().clone() {
            panic!(
                "XML configuration args: {:?} and actual boxed subcontent {:?} do not match.",
                args, subcontent
            );
        }
        self.imp().subcontent.replace(subcontent);
        self.imp().subcontent_args.replace(args);
    }

    pub fn write_to_mod(&self, mod_dir_full_pth : &Path) -> std::io::Result<()> {
        // TODO: Undo previous write operations if there was an error with the current one?
        let subcontent = self.imp().subcontent.borrow();
        let args = self.imp().subcontent_args.borrow();
        let relative_pth = self.relative_path();
        
        let full_path = mod_dir_full_pth.join(relative_pth);

        
        if !full_path.exists() {
            std::fs::create_dir_all(&full_path)?;
        }
        
        for i in 0..subcontent.len() {
            subcontent[i].write_to_mod(self.full_id(), &full_path, args[i].clone())?;
        }

        Ok(())
    }

    pub fn json_value(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self.imp())
    }

    pub fn write_to_game(&self) {
        let subcontent = self.imp().subcontent.borrow();
        for d in subcontent.iter() {
            d.write_to_game();
        }
    }
}