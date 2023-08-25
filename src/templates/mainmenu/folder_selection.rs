use std::ffi::OsStr;

use gtk::subclass::prelude::*;
use gtk::prelude::{GtkWindowExt, WidgetExt, FileExt};
use gtk::traits::{ButtonExt, FileChooserExt, DialogExt};
use gtk::{ResponseType, FileChooserDialog, FileChooserAction, MessageDialog};

use gtk::glib;
use glib::clone;

use crate::MainMenuWindow;

impl MainMenuWindow {
    pub(super) fn jackbox_folder(&self) -> Option<gtk::gio::File> {
        self.imp().jackbox_folder.borrow().clone()
    }

    pub fn verify_folder(&self, folder_opt : Option<gtk::gio::File>) -> Result<gtk::gio::File, &'static str> {
        let mut folder = folder_opt.expect("Could not get folder.");
        // First, verify base path.
        let mut path = folder.path().expect("Could not get folder pathname.");
        if (!path.has_root()) {
            return Err("Path does not contain root.");
        }

        if (!path.exists()) {
            return Err("Path does not exist.");
        }

        if (path.file_name() == Some(OsStr::new("The Jackbox Party Pack 7"))) {
            // TODO: Check if the games are in the subdirectory. And relevant files?
            path.push("games");
            if (!path.exists()) {
                return Err("Could not find games subdirectory.");
            }
            folder = folder.child("games");
        } else if (path.file_name() == Some(OsStr::new("games"))) {
            let parent = path.parent().expect("Could not get parent directory.");
            if parent.file_name() != Some(OsStr::new("The Jackbox Party Pack 7")) {
                return Err("games subdirectory not in Jackbox Party Pack 7 folder.");
            }
        } else {
            return Err("Could not find Jackbox Party Pack 7 directory.");
        }

        println!("Found folder at {}", folder.path().unwrap().display());

        Ok(folder)
    }

    fn set_folder(&self, file_chooser : &FileChooserDialog, response_type : ResponseType) {
        if response_type == ResponseType::Ok {
            // TODO: Folder validation.
            if (file_chooser.file().is_some()) {
                let folder = file_chooser.file();
                let verified_folder = self.verify_folder(folder);
                if (verified_folder.is_err()) {
                    // TODO: Print dialogue box.
                    let dialg = MessageDialog::new(Some(file_chooser), gtk::DialogFlags::MODAL | gtk::DialogFlags::DESTROY_WITH_PARENT, gtk::MessageType::Error, gtk::ButtonsType::Ok, verified_folder.expect_err("Could not get error."));
                    dialg.set_title(Some("Error"));
                    dialg.connect_response(move |this, _| {
                        this.close();
                    });
                    dialg.present();
                    return;
                }

                let folder_clone = verified_folder.expect("Could not get verified folder.");
                self.imp().jackbox_folder.replace(Some(folder_clone));
                //println!("{}", self.jackbox_folder().path().expect("Could not get path name.").display());
                if (!self.imp().content_columns.is_visible()) {
                    self.toggle_creation_visibility(true);
                    self.toggle_folder_visibility(false);
                }
            }
        }
        if (response_type == ResponseType::Ok || response_type == ResponseType::Cancel) {
            file_chooser.set_visible(false);
        }
    }

    pub(super) fn setup_folder_selection(&self) {
        let file_chooser = FileChooserDialog::new(Some("Select the folder for the Jackbox Party Pack 7"), Some(self), FileChooserAction::SelectFolder, &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
        file_chooser.set_hide_on_close(true);

        file_chooser.connect_response(clone!(@weak self as window => move |file_chooser,response_type| {
            window.set_folder(file_chooser, response_type);
        }));

        self.imp().folder_choose.connect_clicked(move |_| { file_chooser.present(); });

        // TODO: Add grabbing folder from config.
        // TODO: Hide other menu buttons.
        if (self.jackbox_folder().is_none()) {
            self.toggle_creation_visibility(false);
            self.toggle_folder_visibility(true);
        }
    }

    pub fn toggle_folder_visibility(&self, visible: bool) {
        self.imp().folder_box.set_visible(visible);
    }
}