use gtk::gio;

pub mod filebrowse;
pub mod mainmenu;
pub mod content;

pub fn load_resources() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");
}