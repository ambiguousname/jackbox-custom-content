use gtk::{gio::{self, ListStore}, Button, Window, SingleSelection, TreeListRow, AlertDialog};
use glib::clone;

use std::cell::OnceCell;

use crate::{quick_template, content::Content};

mod game_list;

use self::game_list::GameListItem;

quick_template!(ContentCreationDialog, "/templates/mainmenu/content_creation/content_creation.ui", gtk::Window, (gtk::Widget), (gtk::Native, gtk::Root, gtk::ShortcutManager),
    #[derive(Default, CompositeTemplate)]
    handlers struct {
        // #[template_child(id="game_select_stack")]
        // pub content_stack : TemplateChild<Stack>,

        #[template_child(id="content_select_model")]
        pub content_select_model : TemplateChild<gtk::SingleSelection>,

        #[template_child(id="game_select_model")]
        pub game_select_model : TemplateChild<gtk::SingleSelection>,

        pub tree_select_model : OnceCell<gtk::TreeListModel>,
    }
);

impl ObjectImpl for imp::ContentCreationDialog {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_switch();
        obj.setup_model();
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
        GameListItem::ensure_all_types();
        Content::ensure_all_types();
        ContentCreationDialog::ensure_type();
    }

    fn setup_model(&self) {
		let data : ListStore = gtk::Builder::from_resource("/content/content_list.ui").object("content_list").expect("Could not get store.");
		let tree = gtk::TreeListModel::new(data, false, true, |item| {
			let party_pack : GameListItem = item.clone().downcast().expect("Could not get party pack item.");

			if party_pack.children().is_some() {
				party_pack.children()
			} else {
				None
			}
		});
		self.imp().game_select_model.set_model(Some(&tree));
	}

    fn setup_switch(&self) {
        let game_select = self.imp().game_select_model.clone();
        game_select.connect_selection_changed(clone!(@weak self as window => move |selection, _, _| {
            window.switch(selection);
        }));
    }

    fn switch(&self, selection : &SingleSelection) {
        let row : TreeListRow = selection.selected_item().and_downcast().expect("Could not get TreeListRow.");
        let item : GameListItem = row.item().and_downcast().expect("Could not get GameListItem");
        if item.content().is_some() {
            self.imp().content_select_model.set_model(Some(&item.content().unwrap()));
        } else {
            self.imp().content_select_model.set_model(None::<& gio::ListModel>);
        }
    }

	#[template_callback]
    fn handle_create_clicked(&self, _button : &Button) {
        let current_option = self.imp().content_select_model.selected_item();
        if current_option.is_none() {
            let dlg = AlertDialog::builder()
            .message("Could not create content.")
            .detail("Try selecting a game from the left.")
            .build();
            dlg.show(Some(self));
            return;
        }

        let current_selection : Content = current_option.and_downcast().expect("Could not get selected.");
        current_selection.create_content(Some(self));
    }
}