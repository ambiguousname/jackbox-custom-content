use crate::{quick_object, templates::content_util::form::FormObjectExt};
use super::form::FormObject;

use std::cell::RefCell;

quick_object!(FormManager, gtk::Box, (gtk::Widget), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget), 
	#[derive(Default)]
	// #[properties(wrapper_type=super::FormManager)]
	struct {
		pub form_objects : RefCell<Vec<FormObject>>,
	}
);

impl ObjectImpl for imp::FormManager {}
impl WidgetImpl for imp::FormManager {}
impl BoxImpl for imp::FormManager {}

impl FormManager {
	pub fn add_form_object(&self, form_object : FormObject) {
		self.imp().form_objects.borrow_mut().push(form_object);
	}

	pub fn submit(&self) -> bool {
		let objects = self.imp().form_objects.borrow();

		for obj in objects.iter() {
			if obj.required() {
				if !obj.is_valid() {
					return false;
				}
			}
		}
		true
	}

	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		FormManager::ensure_type();
	}
}