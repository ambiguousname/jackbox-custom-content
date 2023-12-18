mod content_view;
mod content_creation;

use std::{cell::{RefCell, RefMut, Ref}, vec::Vec, fs::DirEntry};

// Template construction:
use gtk::{Application, Box, Button, Grid, Stack, StackSwitcher, gio::{self, ActionEntry, Cancellable}, Window, Entry, AlertDialog};
use glib::clone;

use content_creation::ContentCreationDialog;
use crate::{content::GameContent, mod_config::ModsConfig, quick_template};
use content_view::ContentList;

use std::{fs, path::Path};

mod folder_selection;

quick_template!(MainMenuWindow, "/templates/mainmenu/mainmenu.ui", gtk::ApplicationWindow, (gtk::Window, gtk::Widget), (gio::ActionGroup, gio::ActionMap, gtk::Native, gtk::Root, gtk::ShortcutManager), handlers struct {
	// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
	#[template_child(id="mod_selection")]
	pub mod_selection : TemplateChild<gtk::Paned>,
	
	#[template_child(id="mod_stack")]
	pub mod_stack : TemplateChild<Stack>,

	#[template_child(id="mod_stack_sidebar")]
	pub mod_stack_sidebar : TemplateChild<StackSwitcher>,
	
	#[template_child(id="start_file_selection")]
	pub folder_choose : TemplateChild<Button>,
	#[template_child(id="folder_box")]
	pub folder_box : TemplateChild<Box>,

	#[template_child(id="first_new_mod")]
	pub first_new_mod : TemplateChild<Box>,

	#[template_child(id="new_content")]
	pub new_content : TemplateChild<Button>,
	pub content_creation_dialog: RefCell<ContentCreationDialog>,

	pub mod_creation_dialog: RefCell<Window>,

	pub mods_config : RefCell<ModsConfig>,
});

impl ObjectImpl for imp::MainMenuWindow {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();

		obj.setup_actions();

		// Not working for whatever reason with the mainmenu.ui property xml.
		obj.imp().mod_selection.set_shrink_start_child(false);
		obj.imp().mod_selection.set_shrink_end_child(false);
		obj.setup_stack();

		obj.setup_mods_config();

		obj.setup_add_content();

		obj.setup_add_mod_creation();

		obj.setup_folder_selection();
	}
}
impl WidgetImpl for imp::MainMenuWindow {}
impl WindowImpl for imp::MainMenuWindow {}
impl ApplicationWindowImpl for imp::MainMenuWindow {}

#[gtk::template_callbacks]
impl MainMenuWindow {
	pub fn new(app : &Application) -> Self {
		Object::builder().property("application", app).build()
	}

	// region: Action Setup
	fn setup_actions(&self) {
		let new_action = ActionEntry::builder("new")
		.activate(|window : &MainMenuWindow, _, _| {
			window.new_mod();
		})
		.build();

		let delete_action = ActionEntry::builder("delete")
		.activate(|window : &MainMenuWindow, _, _| {
			window.start_mod_deletion();
		})
		.build();

		self.add_action_entries([new_action, delete_action]);
	}
	// endregion
	
	// region: Initial folder/content setup.
	
	pub fn toggle_creation_visibility(&self, visible: bool) {
		self.imp().mod_selection.set_visible(visible);
		self.imp().new_content.set_visible(visible);
	}

	pub fn add_game_info(&self, games : Vec<GameContent>) {
		let d = self.imp().content_creation_dialog.borrow();
		for game in games {
			d.add_game_type(game);
		}
	}
	// endregion

	// region: Mods config
	fn mods_config(&self) -> Ref<'_, ModsConfig> {
		self.imp().mods_config.borrow()
	}

	fn mods_config_mut(&self) -> RefMut<'_, ModsConfig> {
		self.imp().mods_config.borrow_mut()
	}

	// Remove the _ if this ends up getting used.
	fn _reset_mods_config_settings(&mut self) {
		self.mods_config_mut().reset();
	}

	fn setup_mods_config(&self) {
		self.mods_config_mut().initialize();
	}
	// endregion

	// region: Mod management
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

	fn new_mod(&self) {
		self.imp().mod_creation_dialog.borrow().present();
	}

	#[template_callback]
	fn handle_new_mod(&self, _button : &Button) {
		self.new_mod();
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

	fn add_mod_to_stack(&self, name : String, mod_list : &ContentList) {
		self.imp().mod_stack.add_titled(mod_list, Some(name.as_str()), name.as_str());
		self.imp().mod_stack.set_visible_child_name(name.as_str());
	}

	pub fn add_mod(&self, name : String) {
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

	pub fn load_mod_from_dir(&self, dir : DirEntry) {
		let result = ContentList::from_folder(dir);

		let mod_list = result.unwrap();
		self.add_mod_to_stack(mod_list.name(), &mod_list);
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

	fn start_mod_deletion(&self) {
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
	// endregion

	// region: Content creation

	fn setup_add_content(&self) {
		let dialog = ContentCreationDialog::new(self);
		self.imp().content_creation_dialog.replace(dialog); 
	}

	#[template_callback]
	fn handle_create_content_clicked(&self, _button: &Button) {
		let d = self.imp().content_creation_dialog.borrow();
		d.present();
	}
	// endregion
}