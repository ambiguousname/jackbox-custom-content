use std::cell::RefCell;

use gtk::glib::{Properties, derived_properties, Value, BorrowedObject};
use gtk::{CompositeTemplate, glib, prelude::*, subclass::prelude::*};
use crate::templates::content_util::form::FormObject;

use super::form::{FormObjectExt, FormObjectImpl};

// use super::form::FormObjectImpl;
mod imp {
	use super::*;

    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type = super::LabelledBox)]
    #[template(resource = "/templates/content_util/labelled_box.ui")]
	pub struct LabelledBox {
		#[property(get, set)]
		pub value_property : RefCell<String>,

		// Attached label properties:
		#[property(get, set)]
		pub label_hexpand : RefCell<bool>,
		#[property(get, set)]
		pub label_vexpand : RefCell<bool>,
		#[property(get, set, default=3)]
		pub label_valign : RefCell<i32>,
		#[property(get, set)]
		pub label_halign : RefCell<i32>,
		#[property(get, set)]
		pub label_xalign : RefCell<f32>,
		#[property(get, set)]
		pub label_yalign : RefCell<f32>,
		#[property(get, set)]
		pub label_mnemonic_widget : RefCell<Option<gtk::Widget>>,
		
		// For errors:
		#[template_child(id="label_child")]
		pub label_child : TemplateChild<gtk::Label>,

		// FormObject requirements:
		#[property(get, set)]
		pub required : RefCell<bool>,
		
		#[property(get, set)]
		pub label : RefCell<String>,
	}

	#[repr(C)]
	pub struct LabelledBoxClass<T: ObjectSubclass> {
		parent_class : <T::ParentType as ObjectType>::GlibClassType,
		pub get_value_obj : fn(&super::LabelledBox) -> gtk::Widget,
	}

	unsafe impl<T: ObjectSubclass> ClassStruct for LabelledBoxClass<T> {
		type Type = T;
	}

	impl LabelledBox {
		fn get_value_obj(this : &super::LabelledBox) -> gtk::Widget {
			this.last_child().unwrap()
		}
	}

	#[glib::object_subclass]
    impl ObjectSubclass for LabelledBox {
        const NAME: &'static str = "JCCLabelledBox";
        type Type = super::LabelledBox;
        type ParentType = gtk::Box;
        type Interfaces = (FormObject,);
		type Class = LabelledBoxClass<Self>;
        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
			klass.get_value_obj = LabelledBox::get_value_obj;
        }
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
}

glib::wrapper! {
	pub struct LabelledBox(ObjectSubclass<imp::LabelledBox>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, FormObject;
}

#[derived_properties]
impl ObjectImpl for imp::LabelledBox {}
impl WidgetImpl for imp::LabelledBox {
	fn realize(&self) {
		self.parent_realize();
		self.obj().construct_form_obj();
		
		let value_obj = self.obj().value_obj();
		// Clear error when the property we're monitoring changes:
		value_obj.connect_notify(Some(&self.value_property.borrow().clone()), move |child, _| {
			let parent = child.ancestor(LabelledBox::static_type()).and_downcast::<LabelledBox>().expect("Could not get parent.");
			parent.display_error(None);
		});
	}
}
impl BoxImpl for imp::LabelledBox {}
impl FormObjectImpl for imp::LabelledBox {
	fn is_valid(&self) -> bool {
		self.obj().is_valid()
	}

	fn value(&self) -> Value {
		self.obj().value()
	}
	fn display_error(&self, error : Option<super::form::FormError>) {
		self.obj().display_error(error);
	}
}

impl LabelledBox {
	pub fn ensure_all_types() {
		FormObject::ensure_all_types();
		LabelledBox::ensure_type();
	}

	pub fn is_valid(&self) -> bool {
		let property = self.value();

		let prop_type = property.value_type();

		// Not easily created as a constant:
		let static_str_vec = Vec::<String>::static_type();
		if prop_type.is_a(static_str_vec) {
			return property.get::<Vec::<String>>().unwrap().is_empty();
		}

		return match prop_type {
			// For checkboxes requiring an acknowledgement or something.
			// Will probably never happen 
			glib::Type::BOOL => property.get::<bool>().unwrap() == true,
			// For things like Entries:
			glib::Type::STRING => !property.get::<String>().unwrap().is_empty(),
			_ => false,
		}
	}

	fn value_obj(&self) -> gtk::Widget {
		let klass = self.class().as_ref();
		(klass.get_value_obj)(self)
	}

	pub fn value(&self) -> Value {
		self.last_child().expect("Could not get LabelledBox last child.").property(&self.imp().value_property.borrow().clone())
	}

	pub fn display_error(&self, error : Option<super::form::FormError>) {
		match error {
			Some(super::form::FormError::INVALID) => self.imp().label_child.add_css_class("error"),
			_ => self.imp().label_child.remove_css_class("error"),
		}
	}
}

pub trait LabelledBoxImpl : BoxImpl {
	fn get_value_obj(&self) -> gtk::Widget;
}

unsafe impl<T: LabelledBoxImpl> IsSubclassable<T> for LabelledBox {
	fn class_init(class: &mut glib::Class<Self>) {
		Self::parent_class_init::<T>(class);

		let klass = class.as_mut();

		fn get_value_obj_trampoline<T : ObjectSubclass + LabelledBoxImpl>(obj : &LabelledBox) -> gtk::Widget {
			let this = obj.dynamic_cast_ref::<<T as ObjectSubclass>::Type>().unwrap().imp();
			LabelledBoxImpl::get_value_obj(this)
		}
		
		klass.get_value_obj = get_value_obj_trampoline::<T>;
	}
}