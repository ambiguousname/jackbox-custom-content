use gtk::glib::Properties;

use crate::quick_template;
use super::{form::{FormObject, FormObjectImpl, FormObjectExt}, labelled_box::{LabelledBox, LabelledBoxImpl}};
use std::cell::RefCell;

quick_template!(FormFilebrowse, "/templates/content_util/form_filebrowse.ui", LabelledBox, (gtk::Box, gtk::Widget), (;FormObject), 
	#[derive(Default, CompositeTemplate, Properties)]
	#[properties(wrapper_type=super::FormFilebrowse)]
	struct {
		#[property(get, set)]
		pub file : RefCell<Option<gtk::gio::File>>,
	}
);

impl ObjectImpl for imp::FormFilebrowse {
	fn constructed(&self) {
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