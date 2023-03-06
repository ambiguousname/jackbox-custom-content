use std::cell::RefCell;

use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, prelude::*};
use gtk::{CheckButton};
use glib::{Object, Binding, BindingFlags};

use super::contentobj::ContentObject;

mod imp {
	use super::*;

	#[derive(Default, CompositeTemplate)]
	#[template(resource="/templates/widgets/contentrow.ui")]
	pub struct ContentRow {
		#[template_child(id="enabled")]
		pub enabled_button: TemplateChild<CheckButton>,
		pub bindings: RefCell<Vec<Binding>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ContentRow {
		const NAME: &'static str = "JCCContentRow";
		type Type = super::ContentRow;
		type ParentType = gtk::Box;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
	}

	impl ObjectImpl for ContentRow {}
    impl WidgetImpl for ContentRow {}
    impl BoxImpl for ContentRow {}
}

glib::wrapper!{
	pub struct ContentRow(ObjectSubclass<imp::ContentRow>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ContentRow {
	pub fn new() -> Self {
		Object::builder().build()
	}

	pub fn bind(&self, content_object: &ContentObject){
		let enabled = self.imp().enabled_button.get();

		let mut bindings = self.imp().bindings.borrow_mut();

		let enabled_binding = content_object.bind_property("enabled", &enabled, "active")
		.flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
		.build();

		bindings.push(enabled_binding);	
	}

	pub fn unbind(&self) {
		for binding in self.imp().bindings.borrow_mut().drain(..) {
			binding.unbind();
		}
	}
}