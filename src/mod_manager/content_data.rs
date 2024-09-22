use std::cell::RefCell;
use std::io::BufWriter;
use std::path::PathBuf;

use gtk::glib::{self, Properties};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use glib::Object;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use crate::content::subcontent::manifest::ManifestItem;
use crate::content::subcontent::Subcontent;
use crate::content::SubcontentBox;

mod imp {
    use super::*;

    /// Data for how to write a given [`crate::content::Content`] type to disk.
    /// Serialized mostly for `manifest.json` that [`crate::mod_manager::mod_store::ModStore`] writes to.
    #[derive(Default, Serialize, Properties)]
    #[properties(wrapper_type=super::ContentData)]
    pub struct ContentData {
        #[property(get, set)]
        #[serde(serialize_with = "get_ref")]
        pub enabled: RefCell<bool>,

        /// The ID for this particular piece of content.
        #[property(get, set)]
        #[serde(serialize_with = "get_ref")]
        pub full_id: RefCell<String>,

        /// The number for this piece of content.
        #[property(get, set)]
        #[serde(serialize_with = "get_ref", rename="num_id")]
        pub id: RefCell<u32>,

        #[property(get, set)]
        #[serde(skip_serializing)]
        /// The relative path where this content is stored.
        pub relative_path: RefCell<PathBuf>,

        /// The particular type of this content, set in the xml definition for a ContentWindow.
        #[property(get, set)]
        #[serde(serialize_with = "get_ref")]
        pub content_type: RefCell<String>,

        /// Store for [`crate::content::Subcontent`], used to invoke various Subcontent functions for writing to and loading from disk.
        #[serde(skip_serializing)]
        pub subcontent: RefCell<Vec<SubcontentBox>>,

        /// The arguments used when calling `subcontent` functions.
        #[serde(skip_serializing)]
        pub subcontent_args: RefCell<Vec<Vec<&'static str>>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentData {
        const NAME: &'static str = "CustomBoxContentData";
        type Type = super::ContentData;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ContentData {}
}

glib::wrapper! {
    pub struct ContentData(ObjectSubclass<imp::ContentData>);
}

impl ContentData {
    // TODO: Write ContentData on creation or modification to a manifest.
    pub fn new(id: u32, full_id: String, relative_path: PathBuf) -> Self {
        Object::builder()
            .property("enabled", true)
            .property("id", id)
            .property("full-id", full_id)
            .property("relative-path", relative_path)
            .build()
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

    pub fn write_to_mod(&self) -> std::io::Result<()> {
        // TODO: Undo previous write operations if there was an error with the current one?
        let subcontent = self.imp().subcontent.borrow();
        let args = self.imp().subcontent_args.borrow();
        let pth = self.relative_path();
        for i in 0..subcontent.len() {
            subcontent[i].write_to_mod(self.full_id(), pth.as_path(), args[i].clone())?;
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

fn get_ref<T : Clone + RefSerialization<S>, S : Serializer>(t : &RefCell<T>, serializer : S) -> Result<S::Ok, S::Error> {
    T::serialize(&t.borrow().clone(), serializer)
}

pub trait RefSerialization<S: Serializer> {
    fn serialize(&self, serializer : S) -> Result<S::Ok, S::Error>;
}

impl<S : Serializer> RefSerialization<S> for String {
    fn serialize(&self, serializer : S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self)
    }
}

impl <S: Serializer> RefSerialization<S> for bool {
    fn serialize(&self, serializer : S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
        serializer.serialize_bool(*self)
    }
}

impl <S: Serializer> RefSerialization<S> for u32 {
    fn serialize(&self, serializer : S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
        serializer.serialize_u32(*self)
    }
}