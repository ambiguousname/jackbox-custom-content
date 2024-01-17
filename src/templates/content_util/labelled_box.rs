use std::cell::{Cell, RefCell};

use gtk::{CompositeTemplate, subclass::prelude::*, glib, prelude::*};
use gtk::glib::{Properties, derived_properties};
use crate::templates::content_util::form::{FormObject, FormObjectExt};

mod imp {
	use super::*;

	#[derive(Default, CompositeTemplate, Properties)]
	#[template(resource="/templates/content_util/labelled_box.ui")]
	#[properties(wrapper_type=super::LabelledBox)]
	pub struct LabelledBox {
		// #[template_child]
		// pub label_child : TemplateChild<gtk::Label>,
		// #[template_child]
		// pub entry_child : TemplateChild<gtk::Entry>,

		#[property(get, set)]
		pub label : RefCell<String>,

		// FormObject requirements:
		#[property(get, set)]
		pub required : RefCell<bool>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for LabelledBox {
		const NAME : &'static str = "JCCLabelledBox";
		type Type = super::LabelledBox;
		type ParentType = gtk::Box;
		type Interfaces = (FormObject,);

		fn class_init(klass: &mut Self::Class) {
			klass.bind_template();
		}
	
		fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
			obj.init_template();
		}
	}
}

glib::wrapper! {
	pub struct LabelledBox(ObjectSubclass<imp::LabelledBox>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::ConstraintTarget, gtk::Buildable, FormObject;
}

#[derived_properties]
impl ObjectImpl for imp::LabelledBox {}
impl WidgetImpl for imp::LabelledBox {}
impl BoxImpl for imp::LabelledBox {}

impl LabelledBox {
	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		LabelledBox::ensure_type();
	}
}