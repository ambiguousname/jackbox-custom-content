use crate::content::ContentCategory;
use gtk::{Window, prelude::*, subclass::prelude::*, glib, CompositeTemplate};

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource="/content/quiplash3/categories/round_prompt.ui")]
    pub struct QuiplashRoundPrompt {}

    #[glib::object_subclass]
    impl ObjectSubclass for QuiplashRoundPrompt {
        const NAME : &'static str = "JCCQuiplashRoundPrompt";
        type Type = super::QuiplashRoundPrompt;
        type ParentType = gtk::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for QuiplashRoundPrompt {}
    impl WidgetImpl for QuiplashRoundPrompt {}
    impl WindowImpl for QuiplashRoundPrompt {}
}

glib::wrapper! {
    pub struct QuiplashRoundPrompt(ObjectSubclass<imp::QuiplashRoundPrompt>) @extends gtk::Window, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl QuiplashRoundPrompt {
    fn new() -> Self {
        glib::Object::new()
    }
}

// TODO: Modify so this is static (using round_prompt.ui)
fn prompt_window() -> Window {
    QuiplashRoundPrompt::new().into()
}

pub const QUIPLASH_PROMPT : ContentCategory = ContentCategory {
    name: "Quiplash Round Prompt",
    open_window : prompt_window,
};