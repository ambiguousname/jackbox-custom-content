
use gtk::{Window, gio::Settings, glib::derived_properties, Switch, AlertDialog};

use std::cell::OnceCell;

use super::mainmenu::MainMenuWindow;

use glib::Properties;

use crate::quick_template;
quick_template!(PreferencesWindow, "/templates/preferences/preferences.ui", Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager),

	#[derive(CompositeTemplate, Default, Properties)]
	props handlers struct {
		#[property(set)]
		pub app_settings : OnceCell<Settings>,

		#[template_child(id="dark_mode")]
		pub dark_mode : TemplateChild<Switch>,

		#[template_child(id="folder_label")]
		pub folder_label : TemplateChild<gtk::Inscription>,
	}
);

// use gtk::{CompositeTemplate, glib::{self, Object, Properties}, prelude::*, subclass::prelude::*};
// mod imp {
//     use super::*;
//     #[derive(Default, CompositeTemplate, Properties)]
//     #[template(resource = "/templates/preferences/preferences.ui")]
//     #[properties(wrapper_type = super::PreferencesWindow)]
//     pub struct PreferencesWindow {
//         #[property(set)]
//         pub app_settings: OnceCell<Settings>,
//         #[template_child(id = "dark_mode")]
//         pub dark_mode: TemplateChild<Switch>,
//         #[template_child(id = "folder_label")]
//         pub folder_label: TemplateChild<gtk::Inscription>,
//     }
//     #[glib::object_subclass]
//     impl ObjectSubclass for PreferencesWindow {
//         const NAME: &'static str = "JCCPreferencesWindow";
//         type Type = super::PreferencesWindow;
//         type ParentType = Window;
//         fn class_init(klass: &mut Self::Class) {
//             klass.bind_template();
//             klass.bind_template_instance_callbacks();
//         }
//         fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
//             obj.init_template();
//         }
//     }
// }

// glib::wrapper! {
// 	pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>) @extends Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
// }


#[derived_properties]
impl ObjectImpl for imp::PreferencesWindow {}
impl WidgetImpl for imp::PreferencesWindow {}
impl WindowImpl for imp::PreferencesWindow {}

#[gtk::template_callbacks]
impl PreferencesWindow {
	pub fn new(parent: &impl IsA<Window>, settings : &Settings) -> Self {
		let this : PreferencesWindow = Object::builder()
		.property("transient-for", parent)
		.property("hide-on-close", true)
		.property("app-settings", settings)
		.build();

		this.init_prefs();
		this
	}

	fn settings(&self) -> &Settings {
		self.imp().app_settings.get().expect("Could not get app settings.")
	}

	fn default_settings(&self) -> gtk::Settings {
		gtk::Settings::default().expect("Could not get default settings.")
	}

	fn update_folder_label(&self, string: String) {
		self.imp().folder_label.set_text(Some(string.as_str()));
		self.imp().folder_label.set_tooltip_text(Some(&string));
	}

	fn init_prefs(&self) {
		let settings = self.settings();

		let dark_mode = settings.boolean("dark-mode");
		self.imp().dark_mode.set_active(dark_mode);
		self.handle_dark_mode(dark_mode);

		let folder_str = settings.string("game-folder");
		self.update_folder_label(folder_str.to_string());
	}

	#[template_callback]
	fn handle_close_prefs(&self) {
		self.close();
	}

	#[template_callback]
	fn handle_dark_mode(&self, val : bool) -> bool {
		let result = self.settings().set_boolean("dark-mode", val);
		self.default_settings().set_gtk_application_prefer_dark_theme(val);

		if result.is_err() {
			let dlg = AlertDialog::builder()
			.message("Could not set dark mode preferences.")
			.detail(result.err().unwrap().to_string())
			.build();
			dlg.show(Some(self));
		}
		false
	}

	#[template_callback]
	fn handle_folder_set(&self) {
		let parent : MainMenuWindow = self.transient_for().and_downcast().expect("Could not get parent.");

		parent.show_folder_selection(self, Some(glib::clone!(@weak self as window => move |result : String| {
			window.update_folder_label(result);
		})));
	}
}