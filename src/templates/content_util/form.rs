// Form object for utility functions like not allowing submission if this form is not completed.
use std::{cell::RefCell, borrow::Borrow};

// FIXME: This would be nice to use as an interface. But that's a nightmare to do setup for in gtk-rs, so I'm sticking myself with this for now. 

use gtk::glib::{ObjectExt, derived_properties, Properties, subclass::Signal, once_cell::sync::Lazy};

use crate::quick_object;

use super::form_manager::FormManager;

// FIXME: Stupid hacky workaround to allow for multiple widget types with this form type (without implementing an interface)
quick_object!(FormObject, gtk::Box, (gtk::Widget), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget), 
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::FormObject)]
	struct {
		#[property(get, set)]
		pub required : RefCell<bool>,
	}
);

impl FormObject {
	pub fn ensure_all_types() {
		FormObject::ensure_type();
	}
}

#[derived_properties]
impl ObjectImpl for imp::FormObject {
	fn constructed(&self) {
		self.parent_constructed();

		let obj = self.obj();
		let manager : FormManager = obj.ancestor(FormManager::static_type()).and_downcast().expect("Could not find parent FormManager");
		manager.add_form_object(obj.clone());
	}
	
	fn signals() -> &'static [Signal] {
        static SIGNALS : Lazy<Vec<Signal>> = Lazy::new(|| {
			vec![Signal::builder("error").build()]
		});
		SIGNALS.as_ref()
    }
}
impl WidgetImpl for imp::FormObject {}
impl BoxImpl for imp::FormObject {}

pub trait FormObjectExt : IsA<FormObject> + 'static {
	fn is_required(&self) -> bool {
		self.property("required")
	}

	fn set_required(&self, required : bool) {
		self.set_property("required", required);
	}

	fn verify(&self) -> bool {
		false
	}
}

impl<O: IsA<FormObject>> FormObjectExt for O {}

pub trait FormObjectImpl : BoxImpl {}

unsafe impl<T: FormObjectImpl> IsSubclassable<T> for FormObject {}