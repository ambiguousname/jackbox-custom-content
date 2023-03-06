use std::cell::RefCell;

use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*};
use gtk::{Widget};
use glib::{Object, Binding, BindingFlags, ParamSpec, ParamSpecObject, once_cell};
use once_cell::sync::Lazy;

use super::contentobj::ContentObject;

// region: Initial ContentCol definitions (Contains definitions for adding properties)
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

	// region: Property creation

	impl ObjectImpl for ContentCol {}
    impl WidgetImpl for ContentCol {}
    impl BoxImpl for ContentCol {}
	// endregion
}

glib::wrapper!{
	pub struct ContentCol(ObjectSubclass<imp::ContentCol>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

// endregion

impl ContentCol {
	pub fn new(child_widget : Widget) -> Self {
		let o : Self = Object::builder().build();
		o.set_child_widget(child_widget);
		o.append(&o.child_widget());
		o
	}

	fn child_widget(&self) -> Widget {
		self.imp()
		.child_widget
		.borrow().clone()
		.expect("Could not get child_widget")
	}

	fn set_child_widget(&self, value : Widget) {
		self.imp()
		.child_widget
		.borrow_mut()
		.replace(value);
	}

	// region: Bindings for connecting content_object data to visible things:
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
	// endregion
}