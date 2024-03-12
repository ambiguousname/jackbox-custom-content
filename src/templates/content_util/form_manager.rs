use gtk::{glib::Value, CssProvider};

use crate::{quick_object, templates::content_util::form::FormObjectExt};
use super::{form::FormObject, labelled_box::LabelledBox, form_checkbox::FormCheckbox, form_filebrowse::FormFilebrowse};

use std::{cell::RefCell, collections::HashMap};

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

	/// "Submit" the form by getting all the values from each of the form objects, mapped from their labels to their values.
	/// TODO: Might want to change to something like IDs. But I'd ideally like to use the Buildable ID system, which you can't really access.
	pub fn submit(&self) -> Option<HashMap<String, Value>> {
		let objects = self.imp().form_objects.borrow();

		let mut map = HashMap::new();
		for obj in objects.iter() {
			if obj.required() {
				if !obj.is_valid() {
					return None;
				}
			}
			map.insert(obj.label(), obj.value());
		}
		Some(map)
	}

	pub fn update_values(&self, values : HashMap<String, Value>)  {
		let objects = self.imp().form_objects.borrow();
		for obj in objects.iter() {
			let label = obj.label();
			if values.contains_key(&label) {
				obj.set_value(values.get(&label).expect("Could not get value associated with label.").clone());
			}
		}
	}
	
	pub fn is_valid(&self) -> bool {
		let objects = self.imp().form_objects.borrow();

		let mut is_valid = true;
		for obj in objects.iter() {
			if obj.required() {
				// TODO: Hook up more complex object validation to errors (i.e., ContentWindow specific validation)? Rather than this more roundabout way.
				if !obj.is_valid() {
					is_valid = false;
					obj.display_error(Some(super::form::FormError::INVALID));
				} else {
					obj.display_error(None);
				}
			}
		}
		return is_valid;
	}


	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
        LabelledBox::ensure_all_types();
		FormCheckbox::ensure_all_types();
		FormFilebrowse::ensure_all_types();
		FormManager::ensure_type();
	}
}