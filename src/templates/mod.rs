use gtk::gio;

pub mod filebrowse;

pub fn load_resources() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");
}