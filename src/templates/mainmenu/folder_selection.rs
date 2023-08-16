use gtk::subclass::prelude::*;
use gtk::prelude::{ObjectExt, GtkWindowExt, WidgetExt};
use gtk::traits::{ButtonExt, FileChooserExt, DialogExt};
use gtk::{ApplicationWindow, ResponseType, FileChooserDialog, FileChooserAction};

use gtk::glib;
use glib::{clone, Value};

use crate::MainMenuWindow;

impl MainMenuWindow {

    fn jackbox_folder(&self) -> gtk::gio::File {
        self.imp().jackbox_folder.borrow().clone().expect("Could not get jackbox folder.")
    }

    fn set_folder_name(&self, file_chooser : &FileChooserDialog, response_type : ResponseType) {

        if response_type == ResponseType::Ok {
            if (file_chooser.file().is_some()) {
                let folder = file_chooser.file().clone().expect("Could not find jackbox folder.");
                self.jackbox_folder().clone_from(&folder);
            }
        }
        if (response_type == ResponseType::Ok || response_type == ResponseType::Cancel) {
            file_chooser.close();
        }
    }

    pub(super) fn setup_folder_selection(&self) {
        let file_chooser = FileChooserDialog::new(Some("Select the folder for the Jackbox Party Pack 7"), Some(self), FileChooserAction::SelectFolder, &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);

        file_chooser.connect_response(clone!(@weak self as window => move |file_chooser,response_type| {
            window.set_folder_name(file_chooser, response_type);
        }));

        self.imp().folder_choose.connect_clicked(move |_| { file_chooser.present(); });
    }

    pub fn toggle_folder_visibility(&self, visible: bool) {
        self.imp().folder_box.set_visible(visible);
    }
}