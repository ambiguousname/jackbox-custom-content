use crate::MainMenuWindow;

use gtk::{subclass::prelude::*, prelude::*, glib, Grid, Entry, Button, Window, Stack, AlertDialog, gio::Cancellable};
use std::{fs::{self, DirEntry}, path::Path};
use glib::clone;

use super::content_view::ContentList;

impl MainMenuWindow {
	// region: Setup
	pub(super) fn setup_mod_editor(&self) {
		self.setup_stack();
		
		self.setup_add_mod_creation();

		self.setup_mod_toolbar();
	}

	fn setup_mod_toolbar(&self) {

	}

	fn setup_add_mod_creation(&self) {
		let grid = Grid::builder()
		.build();

		let entry = Entry::builder()
		.hexpand(true)
		.vexpand(true)
		.placeholder_text("Mod Name")
		.build();
		grid.attach(&entry, 0, 0, 2, 1);

		let submit = Button::builder()
		.hexpand(true)
		.vexpand(true)
		.label("Ok")
		.build();
		grid.attach(&submit, 0, 1, 1, 1);

		submit.connect_clicked(clone!(@weak self as window => move |this| {
			this.ancestor(Window::static_type()).and_downcast::<Window>().expect("Could not get window.").close();
			window.add_mod(entry.text().to_string());
			entry.set_text("");
		}));

		let cancel = Button::builder()
		.label("Cancel")
		.build();
		grid.attach(&cancel, 1, 1, 1, 1);
		
		cancel.connect_clicked(|this| {
			this.ancestor(Window::static_type()).and_downcast::<Window>().expect("Could not get window.").close();
		});

		let dlg = Window::builder()
		.child(&grid)
		.transient_for(self)
		.hide_on_close(true)
		.build();

		self.imp().mod_creation_dialog.replace(dlg);
	}

	fn stack_changed(stack : &Stack) {
		let window : MainMenuWindow = stack.ancestor(MainMenuWindow::static_type()).and_downcast().expect("Could not get main menu window.");

		let new_name = stack.visible_child_name();
		let mut name = "".to_string();

		if new_name.is_some() {
			name = new_name.unwrap().to_string();
			window.imp().first_new_mod.set_visible(false);
		} else {
			window.imp().first_new_mod.set_visible(true);
		}

		if name != "All" && name != "" {
			window.imp().new_content.set_visible(true);
		} else {
			window.imp().new_content.set_visible(false);
		}
	}

	fn setup_stack(&self) {
		self.imp().mod_stack.connect_notify(Some("visible-child"), |this, _| { MainMenuWindow::stack_changed(this); });

		// TODO: Make an "All" content list that doesn't use traditional mod loading.
		// self.add_mod("All".to_string());
		let mods_folder = Path::new("./mods");

        if !mods_folder.exists() {
            let result = fs::create_dir(mods_folder.clone());
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

	// endregion

	// region: Add Mods

	pub(super) fn new_mod(&self) {
		self.imp().mod_creation_dialog.borrow().present();
	}

	fn add_mod_to_stack(&self, name : String, mod_list : &ContentList) {
		self.imp().mod_stack.add_titled(mod_list, Some(name.as_str()), name.as_str());
		self.imp().mod_stack.set_visible_child_name(name.as_str());
	}

	fn add_mod(&self, name : String) {
		// Create new ContentList:
		let result = ContentList::new(name.clone());
		if result.is_err() {
			let error = result.err().unwrap();
			AlertDialog::builder()
			.message("Could not create mod folder.")
			.detail(error.to_string()).build().show(Some(self));
			return;
		}

		let mod_list = result.unwrap();
		self.add_mod_to_stack(name, &mod_list);
	}

	fn load_mod_from_dir(&self, dir : DirEntry) {
		let result = ContentList::from_folder(dir);

		let mod_list = result.unwrap();
		self.add_mod_to_stack(mod_list.name(), &mod_list);
	}

	// endregion

	// region: Mod Deletion

	pub(super) fn start_mod_deletion(&self) {
		let visible_child = self.imp().mod_stack.visible_child_name();
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

		warn.choose(Some(self), Some(&Cancellable::new()), clone!(@weak self as window => move |result| {
			let option = result.expect("Could not get warn option.");
			if option == 0 {
				window.delete_mod(mod_name);
			}
		}));
	}

	fn delete_mod(&self, mod_name : String) {
		let mods_folder = Path::new("./mods");
		let mod_folder = mods_folder.join(mod_name.clone());
		
		let result = fs::remove_dir(mod_folder);

		if result.is_err() {
			let msg = format!("Could not delete mod {mod_name}");
			let err = AlertDialog::builder()
			.message(msg)
			.detail(result.err().unwrap().to_string())
			.build();

			err.show(Some(self));
			return;
		}

		let child = self.imp().mod_stack.child_by_name(mod_name.as_str()).expect("Could not get child.");

		// Select the next thing:
		if (self.imp().mod_stack.pages().n_items() - 1 > 0) {
			self.imp().mod_stack.set_visible_child(&child.next_sibling().or(child.prev_sibling()).unwrap());
		}

		// FIXME: When using StackSidebar, creates a segfault. This is a known issue: https://gitlab.gnome.org/GNOME/gtk/-/issues/5917
		// We're using a stackswitcher for now, but a StackSidebar would look better (whenever this gets fixed)
		// For now the program still runs.
		self.imp().mod_stack.remove(&child);
		
		MainMenuWindow::stack_changed(&self.imp().mod_stack);
	}

	// endregion
}