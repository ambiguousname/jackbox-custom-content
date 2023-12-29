use crate::{quick_template, content::Content};
use glib::{Properties, derived_properties};
use gtk::gio::{ListModel, ListStore};
use std::cell::{RefCell, OnceCell};

// Way to specify no extension? For subclass GObject directly.

mod game_imp {

	use super::*;

	#[derive(Default, Properties)]
	#[properties(wrapper_type=super::GameListItem)]
	pub struct GameListItem {
		// The display name of the item.
		#[property(get, set)]
		pub title : OnceCell<String>,

		#[property(get, set)]
		pub children : RefCell<Option<ListModel>>,

		#[property(get, set)]
		pub content : RefCell<Option<ListModel>>,
	}

	#[glib::object_subclass]
	impl ObjectSubclass for GameListItem {
		const NAME: &'static str = "JCCGameListItem";
		type Type = super::GameListItem;
		type ParentType = Object;
	}

	#[derived_properties]
	impl ObjectImpl for GameListItem {
		
	}
}

glib::wrapper! {
	pub struct GameListItem(ObjectSubclass<game_imp::GameListItem>);
}

impl GameListItem {
	pub fn ensure_all_types() {
		GameListItem::ensure_type();
	}
}

quick_template!(GameList, "/templates/mainmenu/content_creation/game_list.ui", gtk::Box, (gtk::Widget), (),
	#[derive(CompositeTemplate, Default)]
	struct {
		#[template_child(id="game_select_model")]
		pub model: TemplateChild<gtk::SingleSelection>,
	}
);

impl ObjectImpl for imp::GameList {
	fn constructed(&self) {
		self.parent_constructed();
		
		let obj = self.obj();
		obj.setup_model();
	}
}
impl WidgetImpl for imp::GameList {}
impl BoxImpl for imp::GameList {}

impl GameList {
	pub fn ensure_all_types() {
		GameList::ensure_type();
		GameListItem::ensure_all_types();
		Content::ensure_all_types();
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
		self.imp().model.set_model(Some(&tree));
	}
}