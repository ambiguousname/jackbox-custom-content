use std::ffi::OsStr;

use gtk::gio::Cancellable;
use gtk::subclass::prelude::*;
use gtk::prelude::{GtkWindowExt, WidgetExt, FileExt, SettingsExt};
use gtk::traits::{ButtonExt};
use gtk::{ResponseType, FileDialog, FileChooserAction, AlertDialog};

use gtk::{glib, gio};
use glib::clone;

use crate::MainMenuWindow;

impl MainMenuWindow {
    fn verify_folder(&self, folder_opt : gtk::gio::File) -> Result<gtk::gio::File, &'static str> {
        let mut folder = folder_opt;
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

    fn set_folder(&self, result : Result<gio::File, glib::Error>) {
        if result.is_ok() {
            let folder : gtk::gio::File = result.expect("Could not get file.");
            let verified_folder = self.verify_folder(folder);
            if (verified_folder.is_err()) {
                let dialg = AlertDialog::builder()
                // .buttons(gtk::ButtonsType::Ok)
                .message("Selection was not a valid folder.")
                .build();
                // Some(file_chooser), gtk::DialogFlags::MODAL | gtk::DialogFlags::DESTROY_WITH_PARENT, gtk::MessageType::Error, gtk::ButtonsType::Ok, verified_folder.expect_err("Could not get error.");
                // dialg.connect_response(move |this, _| {
                //     this.close();
                // });
                dialg.show(Some(self));
                return;
            }

            let folder_clone = verified_folder.expect("Could not get verified folder.");

            let path = folder_clone.path().expect("Could not get folder PathBuf.");
            let path_str = path.to_str().expect("Could not get PathBuf str.");

            self.config().set_string("game-folder", path_str).expect("Could not set folder setting.");
            //println!("{}", self.jackbox_folder().path().expect("Could not get path name.").display());
            if (!self.imp().mod_selection.is_visible()) {
                self.toggle_creation_visibility(true);
                self.toggle_folder_visibility(false);
            }
        }
    }

    pub(super) fn setup_folder_selection(&self) {
        
        let file_chooser = FileDialog::builder()
        .title("Select the folder for the Jackbox Party Pack 7")
        .build();
        // Some(), Some(self), FileChooserAction::SelectFolder, &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
        // file_chooser.set_hide_on_close(true);

        // file_chooser.connect_response(clone!(@weak self as window => move |file_chooser,response_type| {
        //     window.set_folder(file_chooser, response_type);
        // }));

        self.imp().folder_choose.connect_clicked(clone!(@weak self as window => move |_| {
            let cancel = Cancellable::new();
            file_chooser.select_folder(Some(&window), Some(&cancel), clone!(@weak window => move |result| {
                window.set_folder(result);
            }));
        }));

        // TODO: Hide other menu buttons.
        let folder = self.config().user_value("game-folder");
        if (folder.is_none()) {
            self.toggle_creation_visibility(false);
            self.toggle_folder_visibility(true);
        }
    }

    pub fn toggle_folder_visibility(&self, visible: bool) {
        self.imp().folder_box.set_visible(visible);
    }
}