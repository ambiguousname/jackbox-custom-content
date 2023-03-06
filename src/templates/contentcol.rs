use std::cell::RefCell;

use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*};
use gtk::{Widget};
use glib::{Object, Binding, BindingFlags, ParamSpec, ParamSpecObject, once_cell};
use once_cell::sync::Lazy;

use super::contentobj::ContentObject;

mod imp {
	use super::*;

	#[derive(Default)]
	pub struct ContentCol {
		pub child_widget: RefCell<Option<Widget>>,
		pub bindings: RefCell<Vec<Binding>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for ContentCol {
		const NAME: &'static str = "JCCContentCol";
		type Type = super::ContentCol;
		type ParentType = gtk::Box;
	}

	impl ObjectImpl for ContentCol {
		fn properties() -> &'static [glib::ParamSpec] {
			static PROPERTIES : Lazy<Vec<ParamSpec>> = Lazy::new(|| {
				vec![
					ParamSpecObject::builder::<Widget>("child-widget").build(),
				]
			});
			PROPERTIES.as_ref()
		}

		fn set_property(&self, _id: usize, _value: &glib::Value, _pspec: &ParamSpec) {
			match _pspec.name() {
				"child-widget" => {
					let input_value = _value.get().expect("Value should be of type `Widget`.");
					self.child_widget.borrow_mut().replace(input_value);
					self.obj().append(&self.obj().child_widget());
				},
				_ => unimplemented!(),
			}
		}

		fn property(&self, _id: usize, _pspec: &ParamSpec) -> glib::Value {
			match _pspec.name() {
				"child-widget" => self.child_widget.borrow().clone().expect("Could not get mut child_widget.").to_value(),
				_ => unimplemented!(),
			}
		}
	}
    impl WidgetImpl for ContentCol {}
    impl BoxImpl for ContentCol {}
}

glib::wrapper!{
	pub struct ContentCol(ObjectSubclass<imp::ContentCol>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ContentCol {
	pub fn new(child_widget : Widget) -> Self {
		Object::builder()
		.property("child-widget", child_widget)
		.build()
	}

	fn child_widget(&self) -> Widget {
		self.imp()
		.child_widget
		.borrow().clone()
		.expect("Could not get child_widget")
	}

	// Connecting content_object data to visible things:
	pub fn bind(&self, content_object: &ContentObject){
		let widget = self.child_widget();

		let mut bindings = self.imp().bindings.borrow_mut();

		/*let binding = content_object.bind_property("enabled", &widget, "active")
		.flags(BindingFlags::SYNC_CREATE | BindingFlags::BIDIRECTIONAL)
		.build();*/

		// bindings.push(binding);	
	}

	pub fn unbind(&self) {
		for binding in self.imp().bindings.borrow_mut().drain(..) {
			binding.unbind();
		}
	}
}