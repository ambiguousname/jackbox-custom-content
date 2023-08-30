use gtk::{prelude::*, subclass::prelude::*, glib, Window, CompositeTemplate, gio, ListBox, Stack, Button};
use glib::{clone, Object};
use gio::{SimpleAction, SimpleActionGroup};

use std::cell::RefCell;

use crate::content::GameContent;

mod imp {

    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource="/templates/mainmenu/content_creation/content_creation.ui")]
    pub struct ContentCreationDialog {
        #[template_child(id="game_select_stack")]
        pub content_stack : TemplateChild<Stack>,

        pub action_group : RefCell<Option<SimpleActionGroup>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ContentCreationDialog {
        const NAME: &'static str = "JCCContentCreationDialog";
		type Type = super::ContentCreationDialog;
		type ParentType = gtk::Window;

		fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
			klass.bind_template_instance_callbacks();
        }
    
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ContentCreationDialog {
        fn constructed(&self) {
            self.parent_constructed();
            
            let obj = self.obj();
            obj.setup_action_group();
        }
    }
    impl WidgetImpl for ContentCreationDialog {}
	impl WindowImpl for ContentCreationDialog {}
}

glib::wrapper! {
    pub struct ContentCreationDialog(ObjectSubclass<imp::ContentCreationDialog>) @extends gtk::Window, gtk::Widget,
	@implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[gtk::template_callbacks]
impl ContentCreationDialog {
    pub fn new(parent: &impl IsA<Window>) -> Self {
        let this : Self = Object::builder()
        .property("transient-for", parent)
        .property("hide-on-close", true)
        .build();
        this
    }

	#[template_callback]
    fn handle_create_clicked(&self, _button : &Button) {
        let current_page = self.imp().content_stack.visible_child().expect("No selected page.");

        let selector : ListBox = current_page.downcast::<ListBox>().expect("Could not get ListBox.");

        let row = selector.selected_row();
        if row.is_none() {
            return;
        }

        let selected = row.unwrap().child().expect("Could not get child.");
        let window_name : String = selected.property("label");
        let action_name = format!("{}-open-window", window_name);
        
        self.action_group().activate_action(&action_name, None);
    }

    fn setup_action_group(&self) {
        let action_group = SimpleActionGroup::new();
        self.imp().action_group.replace(Some(action_group));
    }

    fn action_group(&self) -> SimpleActionGroup {
        self.imp().action_group.borrow().clone().expect("Could not get action group.")
    }

    pub fn add_game_type(&self, game : GameContent) {
        let selector = ListBox::new();
        selector.set_selection_mode(gtk::SelectionMode::Single);

        for content_type in game.content_categories {
            // TODO: Fix so you can only add one bit of custom content at a time.
            let option = gtk::Label::new(Some(content_type.name));
            let open = content_type.open_window;

            let action_name = format!("{}-open-window", content_type.name);
            let window_action = SimpleAction::new(&action_name, None);
            window_action.connect_activate(clone!(@weak self as window => move |_, _| {
                let content_window = open();
                content_window.set_property("transient-for", window);
                content_window.present();
            }));
            self.action_group().add_action(&window_action);
            
            selector.append(&option);
        }

        //let model = gio::ListStore::new();
        //let column_view = ColumnView::new();
        // TODO: Custom signal for the page? 
        self.imp().content_stack.add_titled(&selector, Some(game.game_id), game.name);
    }
}