pub mod mod_store;
mod content_data;

use std::{collections::HashMap, fs::{self, DirEntry}, cell::RefCell, sync::OnceLock, path::Path};

use gtk::{gio::Cancellable, glib::{self, clone, subclass::prelude::*, Object}, prelude::*, AlertDialog, Window};

use crate::templates::mainmenu::MainMenuWindow;

use self::{content_data::ContentData, mod_store::ModStore};

// This would be really nice as its own Rust structure, but Glib annoyances (like proper signal connectivity) means that this will have to do.

mod imp {
	use super::*;

	#[derive(Default)]
	pub struct ModManager {
		pub mod_creation : OnceLock<Window>,
		pub main_menu : OnceLock<MainMenuWindow>,
		pub mods : RefCell<HashMap<String, ModStore>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ModManager {
		const NAME: &'static str = "JCCModManager";
		type Type = super::ModManager;
	}

	impl ObjectImpl for ModManager {}
}

glib::wrapper!{
	pub struct ModManager(ObjectSubclass<imp::ModManager>);
}

impl ModManager {
	pub fn new(main_menu : MainMenuWindow) -> Self {
		ModStore::ensure_type();
		ContentData::ensure_type();
		let manager : Self = Object::new();
		manager.imp().main_menu.get_or_init(|| {
			main_menu
		});
		manager.load_mods();
		manager
		// let manager =  ModManager {
		// 	// Need to set up a callback before adding the window:
		// 	mod_creation: None,
		// 	main_menu: Some(main_menu),
		// 	mods: HashMap::new(),
		// };
		// manager.setup_mod_creation_dialog();
		// manager
	}

	// region: Getters
	fn main_menu(&self) -> &MainMenuWindow {
		self.imp().main_menu.get().unwrap()
	}

	fn mod_creation(&self) -> &Window {
		self.imp().mod_creation.get_or_init(|| {
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
	
			submit.connect_clicked(clone!(@weak self as m => move |_| {
				m.mod_creation_finish(entry.text().to_string());
				entry.set_text("");
			}));
	
			let cancel = gtk::Button::builder()
			.label("Cancel")
			.build();
			grid.attach(&cancel, 1, 1, 1, 1);
			
			cancel.connect_clicked(|this| {
				this.ancestor(Window::static_type()).and_downcast::<Window>().expect("Could not get window.").close();
			});
	
			Window::builder()
			.name("Mod Creation Dialog")
			.child(&grid)
			.hide_on_close(true)
			.build()
		})
	}

	pub fn get_mod(&self, mod_name : String) -> Option<ModStore> {
		let store = self.imp().mods.borrow();
		store.get(&mod_name).and_then(|val| {Some(val.clone())})
	}
	// endregion

	// region: Add Mods
	fn add_mod(&self, mod_store : ModStore) {
		self.imp().mods.borrow_mut().insert(mod_store.name(), mod_store.clone());
		self.main_menu().add_mod_to_stack(mod_store.name(), &mod_store);
	}

	pub(super) fn new_mod(&self) {
		self.mod_creation().present();
	}

	fn mod_creation_finish(&self, name : String) {
		// Create new ModStore:
		let result = ModStore::new_folder(name.clone());
		if result.is_err() {
			let error = result.err().unwrap();
			AlertDialog::builder()
			.message("Could not create mod folder.")
			.detail(error.to_string()).build().show(Some(self.main_menu()));
			return;
		}

		let mod_store = result.unwrap();
		self.add_mod(mod_store);
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

	fn load_mod_from_dir(&self, dir : DirEntry) {
		let result = ModStore::from_folder(dir);
		let mod_store = result.unwrap();
		self.add_mod(mod_store);
	}

	// endregion

	// region: Mod Deletion

	pub(super) fn start_mod_deletion(&self) {
		let main_menu = self.main_menu();
		let visible_child = main_menu.visible_mod_stack_name();
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

		warn.choose(Some(main_menu), Some(&Cancellable::new()), clone!(@weak self as w => move |result| {
			let option = result.expect("Could not get warn option.");
			if option == 0 {
				w.delete_mod(mod_name);
			}
		}));
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

			err.show(Some(self.main_menu()));
			return;
		}

		self.imp().mods.borrow_mut().remove(&mod_name.clone());
		let main_menu = self.main_menu();
		main_menu.remove_mod_from_stack(mod_name);
	}

	// endregion

	// region: Editing Mods
	pub fn add_content_to_mod(&self, mod_name : String, content : crate::content::Content) {
		content.create_content(clone!(@weak self as m => move |content_type, subcontent| {
			let curr_mod = m.get_mod(mod_name).expect("Could not get mod of given name.");
			// TODO: Write this.

			let mod_id : String = curr_mod.id();
			let id : i32 = content_data.len();
			let args = crate::content::get_subcontent_args(content.xml_definition(), content_type, subcontent);

			let content_id = format!("{}_{}", id, mod_id.to_string());
			let new_content_data = ContentData::new(id, content_id.clone());
			for s in subcontent {
				s.write_to_mod(content_id.clone());
			}
			
			curr_mod.add_content(new_content_data);
		}));
	}
	// endregion
}