mod content_creation;
mod folder_selection;

use std::{sync::OnceLock, vec::Vec};

// Template construction:
use gtk::{Application, Box, Button, Stack, StackSwitcher, gio::{self, ActionEntry, Settings}, AlertDialog, AboutDialog};

use glib::Object;

use content_creation::ContentCreationDialog;
use crate::{mod_manager::{mod_store::ModStore, ModManager}, quick_template};

use super::preferences::PreferencesWindow;

quick_template!(MainMenuWindow, "/templates/mainmenu/mainmenu.ui", gtk::ApplicationWindow, (gtk::Window, gtk::Widget), (gio::ActionGroup, gio::ActionMap, gtk::Native, gtk::Root, gtk::ShortcutManager),
	#[derive(Default, CompositeTemplate)]
	handlers struct {
		// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
		#[template_child(id="mod_editor")]
		pub mod_editor : TemplateChild<Box>,
		#[template_child(id="mod_toolbar")]
		pub mod_toolbar : TemplateChild<gtk::Box>,

		#[template_child(id="mod_toolbar_name")]
		pub mod_toolbar_name : TemplateChild<gtk::Label>,
		
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
		pub content_creation_dialog: OnceLock<ContentCreationDialog>,

		pub mod_manager : OnceLock<ModManager>,

		pub preferences_window : OnceLock<PreferencesWindow>,
		pub config : OnceLock<Settings>,
	}
);

impl ObjectImpl for imp::MainMenuWindow {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();

		obj.setup_actions();

		obj.setup_stack();

		// Quickly set up our prefs window:
		obj.preferences_window();

		// Same with mod manager:
		obj.mod_manager();

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
			window.mod_manager().new_mod();
		})
		.build();

		let delete_action = ActionEntry::builder("delete")
		.activate(|window : &MainMenuWindow, _, _| {
			window.mod_manager().start_mod_deletion();
		})
		.build();

		let open_action = ActionEntry::builder("dir")
		.activate(|window : &MainMenuWindow, _, _| {
			let result = open::that("mods");
			if result.is_err() {
				let dlg = AlertDialog::builder()
				.message("Could not open mods directory.")
				.detail(result.err().unwrap().to_string())
				.build();
			
				dlg.show(Some(window));
			}
		}).build();

		let prefs_action = ActionEntry::builder("prefs")
		.activate(|window : &MainMenuWindow, _, _| {
			window.preferences_window().present();
		}).build();

		let content_action = ActionEntry::builder("new_content")
		.activate(|window : &MainMenuWindow, _, _| {
			window.handle_create_content_clicked();
		})
		.build();

		let help_action = ActionEntry::builder("help")
		.activate(|window : &MainMenuWindow, _, _| {
			let result = open::that("https://github.com/ambiguousname/jackbox-custom-content/wiki");
			if result.is_err() {
				let dlg = AlertDialog::builder()
				.message("Could not open https://github.com/ambiguousname/jackbox-custom-content/wiki")
				.detail(result.err().unwrap().to_string())
				.build();

				dlg.show(Some(window));
			}
		}).build();

		let about_action = ActionEntry::builder("about")
		.activate(|window : &MainMenuWindow, _, _| {
			let about = AboutDialog::builder()
			.application(&window.application().unwrap())
			.authors(["ambiguousname"])
			.comments("Creates mods for the Jackbox Party Pack 7.\nWith much gratitude to Jackbox Games and the developers of the Jackbox Party Pack 7.\nMade with Rust 2021, GTK 4.12 (gtk-rs 0.7.3), Serde 1.0, and open 5.0.1")
			.copyright("MIT License (c) 2023 ambiguousname")
			.program_name("Jackbox Custom Content")
			.version("2.0.0")
			.website("https://github.com/ambiguousname/jackbox-custom-content")
			.website_label("Source Code")
			.title("About Jackbox Custom Content")
			.license_type(gtk::License::MitX11)
			.build();
			about.present();
		})
		.build();

		self.add_action_entries([new_action, delete_action, open_action, prefs_action, content_action, help_action, about_action]);
	}
	// endregion
	
	// region: Initial folder/content setup.
	
	pub fn toggle_creation_visibility(&self, visible: bool) {
		self.imp().mod_editor.set_visible(visible);
		self.imp().first_new_mod.set_visible(visible);
		self.imp().new_content.set_visible(visible);
	}
	// endregion

	// region: Mod display

	fn mod_manager(&self) -> &ModManager {
		self.imp().mod_manager.get_or_init(|| {
			ModManager::new(self.clone())
		})
	}

	fn setup_stack(&self) {
		self.imp().mod_stack.connect_notify(Some("visible-child"), |this, _| { MainMenuWindow::stack_changed(this); });
		MainMenuWindow::stack_changed(&self.imp().mod_stack);
	}
	
	pub fn add_mod_to_stack(&self, name : String, mod_store : &ModStore) {
		self.imp().mod_stack.add_titled(mod_store, Some(name.as_str()), name.as_str());
		self.imp().mod_stack.set_visible_child_name(name.as_str());
	}

	pub fn remove_mod_from_stack(&self, name : String) {
		let child = self.imp().mod_stack.child_by_name(name.as_str()).expect("Could not get child.");

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

	pub fn visible_mod_stack_name(&self) -> Option<glib::GString> {
		self.imp().mod_stack.visible_child_name()
	}

	fn stack_changed(stack : &Stack) {
		let window : MainMenuWindow = stack.ancestor(MainMenuWindow::static_type()).and_downcast().expect("Could not get main menu window.");

		let new_name = stack.visible_child_name();
		let mut name = "".to_string();

		if new_name.is_some() {
			name = new_name.unwrap().to_string();
			window.imp().first_new_mod.set_visible(false);
			window.imp().mod_editor.set_visible(true);
			window.imp().mod_toolbar_name.set_label(name.as_str());
		} else {
			window.imp().first_new_mod.set_visible(true);
			window.imp().mod_editor.set_visible(false);
		}

		if name != "All" && name != "" {
			window.imp().new_content.set_visible(true);
		} else {
			window.imp().new_content.set_visible(false);
		}
	}

	pub fn add_content_to_mod(&self, content: crate::content::Content) {
		let mod_name = self.visible_mod_stack_name().unwrap();
		let curr_mod = self.mod_manager().get_mod(mod_name.to_string()).expect("Could not find mod.");
		curr_mod.add_content(content);
	}

	// endregion

	// region: Settings config
	fn config(&self) -> &Settings {
		self.imp().config.get_or_init(|| {
			Settings::new(crate::APP_ID)
		})
	}

	// Remove the _ if this ends up getting used.
	fn _reset_config(&self) {
		self.config().reset("game-folder");
	}

	fn preferences_window(&self) -> &PreferencesWindow {
		self.imp().preferences_window.get_or_init(|| {
			PreferencesWindow::new(self, self.config())
		})
	}
	// endregion

	// region: Content creation

	fn content_creation_dialog(&self) -> &ContentCreationDialog {
		self.imp().content_creation_dialog.get_or_init(|| {
			ContentCreationDialog::new(self)
		})
	} 

	#[template_callback]
	fn handle_create_content_clicked(&self) {
		self.content_creation_dialog().present();
	}
	// endregion

	// region: Misc Template Callbacks
	#[template_callback]
	fn handle_new_mod(&self) {
		self.mod_manager().new_mod();
	}
	// endregion
}