
use gtk::{gio::{self, Settings}, glib::{clone, derived_properties}, AlertDialog, DialogError, FileDialog, Switch, Window};

use std::cell::OnceCell;

use super::mainmenu::MainMenuWindow;

use glib::{Object, Properties};

use crate::quick_template;
quick_template!(PreferencesWindow, "/templates/preferences/preferences.ui", Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager),
	#[derive(CompositeTemplate, Default, Properties)]
	#[properties(wrapper_type=super::PreferencesWindow)]
	handlers struct {
		#[property(set)]
		pub app_settings : OnceCell<Settings>,

		#[template_child(id="dark_mode")]
		pub dark_mode : TemplateChild<Switch>,

		#[template_child(id="folder_label")]
		pub folder_label : TemplateChild<gtk::Inscription>,

		#[template_child(id = "mod_folder_label")]
		pub mod_folder_label : TemplateChild<gtk::Inscription>,
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
//         const NAME: &'static str = "CustomBoxPreferencesWindow";
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
		self.imp().folder_label.set_text(Some(&string));
		self.imp().folder_label.set_tooltip_text(Some(&string));
	}

	fn update_mod_folder_label(&self, string: String) {
		self.imp().mod_folder_label.set_text(Some(&string));
		self.imp().mod_folder_label.set_tooltip_text(Some(&string));
	}

	fn init_prefs(&self) {
		let settings = self.settings();

		let dark_mode = settings.boolean("dark-mode");
		self.imp().dark_mode.set_active(dark_mode);
		self.handle_dark_mode(dark_mode);

		let folder_str = settings.string("game-folder");
		self.update_folder_label(folder_str.to_string());

		let mod_folder_str = settings.string("mods-folder");
		self.update_mod_folder_label(mod_folder_str.to_string());
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

		let mut game_folder_str = self.settings().string("game-folder");
		if game_folder_str.is_empty() {
			game_folder_str = glib::GString::from_string_checked("./".into()).unwrap();
		}

		let game_folder = gtk::gio::File::for_path(game_folder_str);

		parent.show_folder_selection(self, game_folder, Some(glib::clone!(
			#[weak(rename_to = window)] self,
			move |result : String| {
				window.update_folder_label(result);
			}
		)));
	}

	fn set_mod_folder(&self, result : Result<gio::File, glib::Error>) -> Result<String, String> {
        if result.is_ok() {
            let folder : gtk::gio::File = result.expect("Could not get file.");

			let path = folder.path().expect("Could not get folder pathname.");
			if (!path.has_root()) {
				return Err("Path does not contain root.".to_string());
			}

			if (!path.exists()) {
				return Err("Path does not exist.".to_string());
			}

            let folder_set = self.settings().set_string("mods-folder", path.to_str().expect("Could not get folder string."));

            if folder_set.is_err() {
                return Err(folder_set.err().unwrap().to_string());
            }

            Ok(path.to_str().unwrap().to_string())
        } else {
            return Err(result.err().unwrap().to_string());
        }
    }

	fn mod_folder_selection(&self, initial_folder : gio::File) {
		let folder_chooser = FileDialog::builder()
        .title("Select the folder to store mods in.")
        .initial_folder(&initial_folder)
        .build();

		let cancel = gio::Cancellable::new();
        folder_chooser.select_folder(Some(self), Some(&cancel), clone!(
            #[weak(rename_to = window)] self,
            move |r| {
            if r.is_err() {
                let err = r.clone().err().unwrap().kind::<DialogError>();
                if err.is_some() {
                    let err_code = err.unwrap();
                    if err_code == DialogError::Cancelled || err_code == DialogError::Dismissed {
                        return;
                    }
                }
            }
            let result = window.set_mod_folder(r);
            if result.is_err() {
                let dlg = AlertDialog::builder()
                .message("Could not set folder for mods.")
                .detail(result.clone().err().unwrap())
                .build();

                dlg.show(Some(&window));
            } else {
				window.update_mod_folder_label(result.unwrap());
            }
        }));
	}

	#[template_callback]
	fn handle_mod_folder_set(&self) {
		let mods_folder = gtk::gio::File::for_path(self.settings().string("mods-folder"));

		self.mod_folder_selection(mods_folder);
	}
}