mod content_view;
mod content_creation;

use std::{cell::{RefCell, OnceCell}, vec::Vec};

// Template construction:
use gtk::{Application, Box, Button, Stack, gio, Window, Entry};
use gio::Settings;

use content_creation::ContentCreationDialog;
use content_view::ContentList;
use crate::content::GameContent;
use crate::quick_template;

mod folder_selection;

quick_template!(MainMenuWindow, "/templates/mainmenu/mainmenu.ui", gtk::ApplicationWindow, (gtk::Window, gtk::Widget), (gio::ActionGroup, gio::ActionMap, gtk::Native, gtk::Root, gtk::ShortcutManager), handlers struct {
	// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
	#[template_child(id="mod_selection")]
	pub mod_selection : TemplateChild<gtk::Paned>,
	
	#[template_child(id="mod_stack")]
	pub mod_stack : TemplateChild<Stack>,
	
	#[template_child(id="start_file_selection")]
	pub folder_choose : TemplateChild<Button>,
	#[template_child(id="folder_box")]
	pub folder_box : TemplateChild<Box>,

	#[template_child(id="new_content")]
	pub new_content : TemplateChild<Button>,
	pub content_creation_dialog: RefCell<ContentCreationDialog>,

	pub mod_creation_dialog: RefCell<Window>,

	pub config : OnceCell<Settings>,
});

impl ObjectImpl for imp::MainMenuWindow {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();
		// Not working for whatever reason with the mainmenu.ui property xml.
		obj.imp().mod_selection.set_shrink_start_child(false);
		obj.imp().mod_selection.set_shrink_end_child(false);
		obj.setup_stack();

		obj.setup_config();

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
	pub fn new(app: &Application) -> Self {
		Object::builder().property("application", app).build()
	}
	
	fn config(&self) -> &Settings {
		self.imp().config.get().expect("Could not get config.")
	}
	
	// region: Public content management
	
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

	// region: Basic setup
	fn setup_stack(&self) {
		let stack = self.imp().mod_stack.clone();
		let all = ContentList::new();
		stack.add_titled(&all, Some("all"), "All");
	}

	fn setup_add_content(&self) {
		let dialog = ContentCreationDialog::new(self);
		self.imp().content_creation_dialog.replace(dialog); 
	}

	fn setup_add_mod_creation(&self) {
		let entry = Entry::builder()
		.placeholder_text("Mod Name")
		.build();

		let submit = Button::builder()
		.label("Ok")
		.build();

		let dlg = Window::builder()
		.child(&entry)
		.child(&submit)
		.transient_for(self)
		.hide_on_close(true)
		.build();
		self.imp().mod_creation_dialog.replace(dlg);
	}

	#[template_callback]
	fn handle_create_content_clicked(&self, _button: &Button) {
		let d = self.imp().content_creation_dialog.borrow();
		d.present();
	}

	#[template_callback]
	fn handle_new_mod(&self, _button : &Button) {
		self.imp().mod_creation_dialog.borrow().present();
	}

	fn reset_config(&self) {
		self.config().reset("game-folder");
	}

	fn setup_config(&self) {
		let cfg = Settings::new(crate::APP_ID);
		self.imp().config.set(cfg).expect("Could not set config.");
	}
	// endregion

}