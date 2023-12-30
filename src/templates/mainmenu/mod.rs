mod content_view;
mod content_creation;

use std::{cell::{RefCell, OnceCell}, vec::Vec};

// Template construction:
use gtk::{Application, Box, Button, Stack, StackSwitcher, gio::{self, ActionEntry, Settings}, Window, AlertDialog, AboutDialog};

use content_creation::ContentCreationDialog;
use crate::quick_template;

use super::preferences::PreferencesWindow;

mod folder_selection;
mod mod_editor;

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
		pub content_creation_dialog: RefCell<Option<ContentCreationDialog>>,

		pub mod_creation_dialog: RefCell<Option<Window>>,

		pub preferences_window : RefCell<Option<PreferencesWindow>>,
		pub config : OnceCell<Settings>,
	}
);

impl ObjectImpl for imp::MainMenuWindow {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();

		obj.setup_actions();

		obj.setup_config();

		obj.setup_add_content();

		obj.setup_mod_editor();

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
			window.imp().preferences_window.borrow().clone().expect("Could not get prefs window").present();
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
		self.imp().new_content.set_visible(visible);
	}
	// endregion

	// region: Mods config
	fn config(&self) -> &Settings {
		self.imp().config.get().expect("Could not get config.")
	}

	// Remove the _ if this ends up getting used.
	fn _reset_config(&self) {
		self.config().reset("game-folder");
	}

	fn setup_config(&self) {
		let cfg = Settings::new(crate::APP_ID);
		self.imp().config.set(cfg.clone()).expect("Could not set initial config.");

		let prefs_window = PreferencesWindow::new(self, &cfg);
		self.imp().preferences_window.replace(Some(prefs_window));
	}
	// endregion

	// region: Content creation

	fn setup_add_content(&self) {
		let dialog = ContentCreationDialog::new(self);
		self.imp().content_creation_dialog.replace(Some(dialog));
	}

	#[template_callback]
	fn handle_create_content_clicked(&self) {
		let d = self.imp().content_creation_dialog.borrow().clone().expect("Could not get content creation dialog.");
		d.present();
	}
	// endregion

	// region: Misc Template Callbacks
	#[template_callback]
	fn handle_new_mod(&self) {
		self.new_mod();
	}
	// endregion
}