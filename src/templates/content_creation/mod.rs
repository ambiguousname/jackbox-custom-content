use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Window, CompositeTemplate, gio, ResponseType};
use glib::Object;

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource="/templates/windows/content_creation.ui")]
    pub struct ContentCreationDialog {
        
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentCreationDialog {
        const NAME: &'static str = "JCCContentCreationDialog";
		type Type = super::ContentCreationDialog;
		type ParentType = gtk::Dialog;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ContentCreationDialog {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WidgetImpl for ContentCreationDialog {}
	impl WindowImpl for ContentCreationDialog {}
    impl DialogImpl for ContentCreationDialog {}
}

glib::wrapper! {
    pub struct ContentCreationDialog(ObjectSubclass<imp::ContentCreationDialog>) @extends gtk::Dialog, gtk::Window, gtk::Widget,
	@implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl ContentCreationDialog {
    pub fn new(parent: &impl IsA<Window>) -> Self {
        let this : Self = Object::builder()
        .property("transient-for", parent)
        .property("hide-on-close", true)
        .property("use-header-bar", 1)
        .build();

        this.add_button("Create", ResponseType::Ok);
        this
    }
}