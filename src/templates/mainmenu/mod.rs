mod content_view;
mod content_creation;

use std::{cell::{RefCell, OnceCell}, vec::Vec};

// Template construction:
use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Application, CompositeTemplate, gio, Box, Button, Stack};
use glib::{clone, Object};
use gio::Settings;

use content_creation::ContentCreationDialog;
use content_view::ContentList;
use crate::content::GameContent;

mod folder_selection;

// region: Boilerplate definitions
mod imp {

use super::*;

	#[derive(Default, CompositeTemplate)]
	// TODO: Move content columns to their own template.
	#[template(resource="/templates/mainmenu/mainmenu.ui")]
	pub struct MainMenuWindow {
		// Important lesson: unless you specify templates in the struct definition here, you'll get an error.
		#[template_child(id="mod_selection")]
		pub mod_selection : TemplateChild<gtk::Paned>,
		#[template_child(id="content_stack")]
		pub content_stack : TemplateChild<Stack>,
		
		#[template_child(id="start_file_selection")]
		pub folder_choose : TemplateChild<Button>,
		#[template_child(id="folder_box")]
		pub folder_box : TemplateChild<Box>,

		#[template_child(id="new_content")]
		pub new_content : TemplateChild<Button>,
		pub content_creation_dialog: RefCell<Option<ContentCreationDialog>>,

		pub config : OnceCell<Settings>,
	}

	// region: Boring Subclass Defs
	#[glib::object_subclass]
	impl ObjectSubclass for MainMenuWindow {
		const NAME: &'static str = "JCCMainMenuWindow";
		type Type = super::MainMenuWindow;
		type ParentType = gtk::ApplicationWindow;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
	}

	impl ObjectImpl for MainMenuWindow {
		fn constructed(&self) {
			self.parent_constructed();

			let obj = self.obj();
			// Not working for whatever reason with the mainmenu.ui property xml.
			obj.imp().mod_selection.set_shrink_start_child(false);
			obj.imp().mod_selection.set_shrink_end_child(false);
			obj.setup_stack();

			obj.setup_config();

			obj.setup_add_content();

			obj.setup_folder_selection();
		}
	}
    impl WidgetImpl for MainMenuWindow {}
	impl WindowImpl for MainMenuWindow {}
	impl ApplicationWindowImpl for MainMenuWindow {}
	// endregion
}

glib::wrapper! {
	pub struct MainMenuWindow(ObjectSubclass<imp::MainMenuWindow>) @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
	@implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}
// endregion

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
		let d = self.imp().content_creation_dialog.borrow().clone().expect("Could not get dialog.");
		for game in games {
			d.add_game_type(game);
		}
	}
	// endregion

	// region: Basic setup
	fn setup_stack(&self) {
		let stack = self.imp().content_stack.clone();
		let all = ContentList::new();
		stack.add_titled(&all, Some("all"), "All");
	}

	fn setup_add_content(&self) {
		let dialog = ContentCreationDialog::new(self);


		self.imp().content_creation_dialog.replace(Some(dialog)); 
		
		self.imp().new_content.connect_clicked(clone!(@weak self as window => move |_| {
			let d = window.imp().content_creation_dialog.borrow().clone().expect("Could not get content creation dialog.");
			d.present();
		}));
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