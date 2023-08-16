use gtk::subclass::prelude::*;
use gtk::prelude::{GtkWindowExt, WidgetExt};
use gtk::traits::{ButtonExt, FileChooserExt, DialogExt};
use gtk::{ResponseType, FileChooserDialog, FileChooserAction};

use gtk::glib;
use glib::clone;

use crate::MainMenuWindow;

impl MainMenuWindow {
    pub(super) fn jackbox_folder(&self) -> Option<gtk::gio::File> {
        self.imp().jackbox_folder.borrow().clone()
    }

    fn set_folder(&self, file_chooser : &FileChooserDialog, response_type : ResponseType) {
        if response_type == ResponseType::Ok {
            // TODO: Folder validation.
            if (file_chooser.file().is_some()) {
                let folder = file_chooser.file();
                self.imp().jackbox_folder.replace(folder.clone());
                //println!("{}", self.jackbox_folder().path().expect("Could not get path name.").display());
                if (!self.imp().content_columns.is_visible()) {
                    self.toggle_content_columns_visibility(true);
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
            self.toggle_content_columns_visibility(false);
            self.toggle_folder_visibility(true);
        }
    }

    pub fn toggle_folder_visibility(&self, visible: bool) {
        self.imp().folder_box.set_visible(visible);
    }
}