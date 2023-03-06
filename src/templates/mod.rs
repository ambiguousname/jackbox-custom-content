use gtk::gio;

pub mod filebrowse;
pub mod mainmenu;
pub mod contentobj;
pub mod contentcol;

pub fn load_resources() {
    gio::resources_register_include!("resources.gresource").expect("Failed to register resources.");
}