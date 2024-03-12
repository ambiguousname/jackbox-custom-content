use gtk::{glib::{Properties, clone, derived_properties}, FileDialog, gio::Cancellable, Window};

use crate::quick_template;
use super::{form::{FormObject, FormObjectImpl, FormObjectExt}, labelled_box::{LabelledBox, LabelledBoxImpl}};
use std::cell::RefCell;

quick_template!(FormFilebrowse, "/templates/content_util/form_filebrowse.ui", LabelledBox, (gtk::Box, gtk::Widget), (;FormObject), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::FormFilebrowse)]
	handlers struct {
		#[template_child(id="inscription")]
		pub inscription : TemplateChild<gtk::Inscription>,

		#[property(get, set)]
		pub filters : RefCell<Option<gtk::gio::ListModel>>,
		#[property(get)]
		pub file : RefCell<Option<gtk::gio::File>>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::FormFilebrowse {
	fn constructed(&self) {
		self.parent_constructed();
		let obj = self.obj();
		let self_box = obj.upcast_ref::<LabelledBox>();
		self_box.set_value_property("file");
	}
}
impl WidgetImpl for imp::FormFilebrowse {}
impl BoxImpl for imp::FormFilebrowse {}
impl FormObjectImpl for imp::FormFilebrowse {
	fn display_error(&self, error : Option<super::form::FormError>) {
		let obj = self.obj();
		let self_box = obj.upcast_ref::<LabelledBox>();
		self_box.display_error(error);
	}

	fn value(&self) -> glib::Value {
		let obj = self.obj();
		let self_box = obj.upcast_ref::<LabelledBox>();
		self_box.value()
	}

	fn set_value(&self, value: glib::Value) {
		let obj = self.obj();
		let self_box = obj.upcast_ref::<LabelledBox>();
		self_box.set_value(value);
	}

	fn is_valid(&self) -> bool {
		let obj = self.obj();
		let self_box = obj.upcast_ref::<LabelledBox>();
		self_box.is_valid()
	}
}
impl LabelledBoxImpl for imp::FormFilebrowse {
	// Return ourselves as a widget, basically. Since we just want access to the file widget.
	fn get_value_obj(&self) -> gtk::Widget {
		self.obj().clone().upcast::<gtk::Widget>()
	}
}

#[gtk::template_callbacks]
impl FormFilebrowse {
	pub fn ensure_all_types() {
		FormFilebrowse::ensure_type();
	}

	pub fn update_inscription(&self) {
		let file = self.imp().file.borrow().clone();
		let mut file_path = None::<&str>;
		let pth;
		if file.is_some() {
			pth = file.unwrap().path().unwrap();
			// .map(|p| {&p.to_string()});
			file_path = file_path.or(pth.to_str());
		}

		let inscr = self.imp().inscription.clone();		
		inscr.set_text(file_path);
		inscr.set_tooltip_text(file_path);
	}

	#[template_callback]
	fn handle_browse(&self) {
		let mut file_chooser = FileDialog::builder();

		let filters = self.imp().filters.borrow().clone();
		if filters.is_some() {
			file_chooser = file_chooser.filters(&filters.unwrap());
		}
		let file_chooser = file_chooser.build();
		let parent = self.ancestor(Window::static_type()).and_downcast::<Window>().unwrap();
		file_chooser.open(Some(&parent), None::<&Cancellable>, clone!(@weak self as f => move |res| {
			f.imp().file.replace(res.ok());
			f.update_inscription();
		}));
	}

	#[template_callback]
	fn handle_clear(&self) {
		self.imp().file.replace(None);
		self.update_inscription();
	}
}