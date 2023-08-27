use gtk::subclass::prelude::*;
use gtk::{prelude::*, glib, Window, CompositeTemplate, gio, ResponseType, StyleContext, Stack, Button, Widget, StackPage};
use glib::Object;

use crate::content::{GameContent, self};
use crate::templates::selector::Selector;

mod imp {

    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource="/templates/windows/content_creation.ui")]
    pub struct ContentCreationDialog {
        #[template_child(id="content_stack")]
        pub content_stack : TemplateChild<Stack>,
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

        let button = this.add_button("Create", ResponseType::Ok).downcast::<Button>().expect("Could not get button.");
        button.connect_clicked(|button| {
            let window_parent = button.ancestor(Window::static_type()).expect("Could not get ancestor.").downcast::<ContentCreationDialog>().expect("Could not get Content Creation Dialog.");

            let pages = window_parent.imp().content_stack.pages();
            let current_page = window_parent.imp().content_stack.visible_child().expect("No selected page.");

            let current_selector = current_page.downcast::<Selector>().expect("Could not get selector.");
            current_selector.selected_callback();
        });
        this
    }

    pub fn add_game_type(&self, game : GameContent) {
        let selector = Selector::new();

        for content_type in game.content_categories {
            selector.add_selection(content_type.name, |_| -> Option<glib::Value> {
                (content_type.open_window)();
                None
            });
        }

        //let model = gio::ListStore::new();
        //let column_view = ColumnView::new();
        // TODO: Custom signal for the page? 
        self.imp().content_stack.add_titled(&selector, Some(game.game_id), game.name);
    }
}