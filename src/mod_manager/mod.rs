pub mod mod_store;
mod mod_data;

use std::{collections::HashMap, fs::{self, DirEntry}, path::Path};

use gtk::{gio::Cancellable, glib::{self, clone}, prelude::*, AlertDialog, Window};

use crate::templates::mainmenu::MainMenuWindow;

use self::mod_store::ModStore;

pub struct ModManager {
	mod_creation : Option<Window>,
	main_menu : MainMenuWindow,
	mods : HashMap<String, ModStore>,
}

impl ModManager {
	pub fn new(main_menu : MainMenuWindow) -> Self{

		let manager = ModManager {
			// Need to set up a callback before adding the window:
			mod_creation: None,
			main_menu: main_menu,
			mods: HashMap::new(),
		};
		manager.setup_mod_creation_dialog();
		manager
	}

	fn setup_mod_creation_dialog(&self) {
		let grid = gtk::Grid::builder()
		.build();

		let entry = gtk::Entry::builder()
		.hexpand(true)
		.vexpand(true)
		.placeholder_text("Mod Name")
		.build();
		grid.attach(&entry, 0, 0, 2, 1);

		let submit = gtk::Button::builder()
		.hexpand(true)
		.vexpand(true)
		.label("Ok")
		.build();
		grid.attach(&submit, 0, 1, 1, 1);

		submit.connect_clicked(move |_| {
			self.mod_creation_finish(entry.text().to_string());
			entry.set_text("");
		});

		let cancel = gtk::Button::builder()
		.label("Cancel")
		.build();
		grid.attach(&cancel, 1, 1, 1, 1);
		
		cancel.connect_clicked(|this| {
			this.ancestor(Window::static_type()).and_downcast::<Window>().expect("Could not get window.").close();
		});

		let dlg = Window::builder()
		.name("Mod Creation Dialog")
		.child(&grid)
		.hide_on_close(true)
		.build();

		self.mod_creation = Some(dlg);
	}

	fn load_mods(&self) {
		// TODO: Make an "All" content list that doesn't use traditional mod loading.
		// self.add_mod("All".to_string());
		let mods_folder = Path::new("./mods");

        if !mods_folder.exists() {
            let result = fs::create_dir(mods_folder);
            if result.is_err() {
                eprintln!("Could not create ./mods directory.");
            }
        }

        for directory in fs::read_dir(mods_folder).unwrap() {
            let dir = directory.expect("Could not get child directory.");
            self.load_mod_from_dir(dir);
        }
		// let gesture = &self.imp().sidebar_gesture;
		// gesture.set_property("widget", self.imp().mod_stack_sidebar.to_value());
	}

	// region: Add Mods
	fn add_mod(&self, mod_store : ModStore) {
		self.mods.insert(mod_store.name(), mod_store.clone());
		self.main_menu.add_mod_to_stack(mod_store.name(), &mod_store);
	}

	pub(super) fn new_mod(&self) {
		self.mod_creation.unwrap().present();
	}

	fn mod_creation_finish(&self, name : String) {
		// Create new ModStore:
		let result = ModStore::new(name.clone());
		if result.is_err() {
			let error = result.err().unwrap();
			AlertDialog::builder()
			.message("Could not create mod folder.")
			.detail(error.to_string()).build().show(Some(&self.main_menu));
			return;
		}

		let mod_store = result.unwrap();
		self.add_mod(mod_store);
	}

	fn load_mod_from_dir(&self, dir : DirEntry) {
		let result = ModStore::from_folder(dir);
		let mod_store = result.unwrap();
		self.add_mod(mod_store);
	}

	// endregion

	// region: Mod Deletion

	pub(super) fn start_mod_deletion(&self) {
		let visible_child = self.main_menu.visible_mod_stack_name();
		if visible_child.is_none() {
			return;
		}
		let mod_name : String = visible_child.unwrap().to_string();
		let msg = format!("Are you sure you want to delete {mod_name}?");

		let warn = AlertDialog::builder()
		.buttons(["Yes", "No"])
		.message(msg)
		.detail("This action cannot be undone.")
		.build();

		warn.choose(Some(&self.main_menu), Some(&Cancellable::new()), move |result| {
			let option = result.expect("Could not get warn option.");
			if option == 0 {
				self.delete_mod(mod_name);
			}
		});
	}

	fn delete_mod(&self, mod_name : String) {
		let mods_folder = Path::new("./mods");
		let mod_folder = mods_folder.join(mod_name.clone());
		
		let result = fs::remove_dir_all(mod_folder);

		if result.is_err() {
			let msg = format!("Could not delete mod {mod_name}");
			let err = AlertDialog::builder()
			.message(msg)
			.detail(result.err().unwrap().to_string())
			.build();

			err.show(Some(&self.main_menu));
			return;
		}

		self.mods.remove(&mod_name.clone());
		self.main_menu.remove_mod_from_stack(mod_name);
	}

	// endregion
}