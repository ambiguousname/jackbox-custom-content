use crate::quick_template;

use gtk::{Window, gio::Settings, glib::derived_properties, Switch, AlertDialog};

use std::cell::OnceCell;

use super::mainmenu::MainMenuWindow;

quick_template!(PreferencesWindow, "/templates/preferences/preferences.ui", Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager), props handlers struct {
	#[property(set)]
	pub app_settings : OnceCell<Settings>,

	#[template_child(id="dark_mode")]
	pub dark_mode : TemplateChild<Switch>,

	#[template_child(id="folder_label")]
	pub folder_label : TemplateChild<gtk::Label>,
});

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
		let mut string_label = string.clone();
		const MAX_LENGTH : usize = 40;
		if string_label.len() > MAX_LENGTH {
			string_label = format!("{}...", string_label.get(..MAX_LENGTH).expect("Could not get shortened string."));
		}
		self.imp().folder_label.set_label(string_label.as_str());
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