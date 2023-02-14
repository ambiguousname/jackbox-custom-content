use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Button, Entry, prelude::*};

mod imp {
	use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(file="filebrowse.ui")]
    pub struct FileBrowseWidget {
        #[template_child(id="browse_button")]
        pub button: TemplateChild<Button>,
        #[template_child(id="folder_location")]
        pub entry: TemplateChild<Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileBrowseWidget {
        const NAME: &'static str = "FileBrowseWidget";
        type Type = super::FileBrowseWidget;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FileBrowseWidget {

    }
    impl WidgetImpl for FileBrowseWidget {}
    impl BoxImpl for FileBrowseWidget {}
}

glib::wrapper!{
	pub struct FileBrowseWidget(ObjectSubclass<imp::FileBrowseWidget>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl FileBrowseWidget {
	pub fn new() -> Self {
		glib::Object::new(&[])
	}
}