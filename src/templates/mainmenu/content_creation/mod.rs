use gtk::{gio, ListBox, Stack, Button, Window};
use glib::clone;
use gio::{SimpleAction, SimpleActionGroup};

use std::cell::RefCell;

use crate::{content::{GameContent, game_list::GameList}, quick_template};

quick_template!(ContentCreationDialog, "/templates/mainmenu/content_creation/content_creation.ui", gtk::Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager), handlers struct {
    // #[template_child(id="game_select_stack")]
    // pub content_stack : TemplateChild<Stack>,

    pub action_group : RefCell<Option<SimpleActionGroup>>,
});

impl ObjectImpl for imp::ContentCreationDialog {
    fn constructed(&self) {
        self.parent_constructed();
        
        let obj = self.obj();
        obj.setup_action_group();
    }
}

impl WidgetImpl for imp::ContentCreationDialog {}
impl WindowImpl for imp::ContentCreationDialog {}

#[gtk::template_callbacks]
impl ContentCreationDialog {
    pub fn new(parent: &impl IsA<Window>) -> Self {
		ContentCreationDialog::ensure_all_types();
        let this : Self = Object::builder()
        .property("transient-for", parent)
        .property("hide-on-close", true)
        .build();
        this
    }

    pub fn ensure_all_types() {
        ContentCreationDialog::ensure_type();
        GameList::ensure_all_types();
    }

	#[template_callback]
    fn handle_create_clicked(&self, _button : &Button) {
        // let current_page = self.imp().content_stack.visible_child().expect("No selected page.");

        // let selector : ListBox = current_page.downcast::<ListBox>().expect("Could not get ListBox.");

        // let row = selector.selected_row();
        // if row.is_none() {
        //     return;
        // }

        // let selected = row.unwrap().child().expect("Could not get child.");
        // let window_name : String = selected.property("label");
        // let action_name = format!("{}-open-window", window_name);
        
        // self.action_group().activate_action(&action_name, None);
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

        selector.set_hexpand(true);

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

        let row = selector.row_at_index(0).expect("Could not get first row.");

        selector.select_row(Some(&row));

        //let model = gio::ListStore::new();
        //let column_view = ColumnView::new();
        // TODO: Custom signal for the page? 
        // self.imp().content_stack.add_titled(&selector, Some(game.game_id), game.name);
    }
}