use gtk::gio;

pub mod filebrowse;
pub mod mainmenu;
pub mod content;
pub mod content_creation;
pub mod selector;

pub fn load_resources() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");
}