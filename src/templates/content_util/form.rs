// Form object for utility functions like not allowing submission if this form is not completed.
use std::cell::RefCell;

// FIXME: An interface would be ideal, but I can't force myself through that right now.
use gtk::glib::{ObjectExt, derived_properties, Properties};

use crate::quick_object;

quick_object!(FormObject, gtk::Widget, (), (gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget), 
	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::FormObject)]
	struct {
		#[property(get, set)]
		pub required : RefCell<bool>,
	}
);

#[derived_properties]
impl ObjectImpl for imp::FormObject {}
impl WidgetImpl for imp::FormObject {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::FormObject>> Sealed for T {}
}

pub trait FormObjectExt : IsA<FormObject> + sealed::Sealed + 'static {
	fn is_required(&self) -> bool {
		self.property("required")
	}

	fn set_required(&self, required : bool) {
		self.set_property("required", required);
	}
}

impl<O: IsA<FormObject>> FormObjectExt for O {}

pub trait FormObjectImpl : WidgetImpl {}

unsafe impl<T: FormObjectImpl> IsSubclassable<T> for FormObject {}