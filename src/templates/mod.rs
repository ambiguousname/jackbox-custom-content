use gtk::gio;

pub mod filebrowse;
pub mod selector;
pub mod mainmenu;


pub fn load_resources() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");
}