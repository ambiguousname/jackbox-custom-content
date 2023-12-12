pub mod contentcol;
pub mod contentobj;

use contentobj::ContentObject;
use contentcol::ContentCol;

use gtk::{ColumnView, glib::derived_properties, ColumnViewColumn, gio, SignalListItemFactory, ListItem, SingleSelection};

use std::cell::RefCell;

use crate::quick_template;

quick_template!(ContentList, "/templates/mainmenu/content_view/contentlist.ui", gtk::Box, (gtk::Widget), (gtk::Orientable), props struct {
    #[template_child(id="column_view")]
    pub column_view : TemplateChild<ColumnView>,

    #[property(get, set)]
    pub model : RefCell<Option<gio::ListStore>>,
});

#[derived_properties]
impl ObjectImpl for imp::ContentList {}
impl WidgetImpl for imp::ContentList {}
impl BoxImpl for imp::ContentList {}

impl ContentList {
    pub fn new() -> Self {
        // We can clone the model however we want, the data stays the same.
        let model = gio::ListStore::new::<ContentObject>(); 
        let list : SingleSelection = SingleSelection::new(Some(model.clone()));
        // model.append(&ContentObject::new(false));
        /*// Uncomment to show:
        let view_clone = view.clone();

        model.append(&ContentObject::new(true));

        println!("{} {}", view_clone.model().unwrap().item(0).and_downcast::<ContentObject>().unwrap().enabled(), model.item(0).and_downcast::<ContentObject>().unwrap().enabled());
        */
        
        let this : ContentList = Object::builder()
        .property("model", model)
        .build();

        this.imp().column_view.set_model(Some(&list));
        this.setup_factory();

        this
    }

    fn setup_factory(&self) {
        let columns = self.imp().column_view.columns();
        let len = columns.n_items();

        for i in 0..len {
			let column = columns.item(i).and_downcast::<ColumnViewColumn>().expect("Column should be `ColumnViewColumn`.");
			
			let factory = SignalListItemFactory::new();
			factory.connect_setup(move |_, list_item| {
				let widget = gtk::Label::new(Some("Test"));
				let content_row = ContentCol::new(gtk::Widget::from(widget));
				list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
				.set_child(Some(&content_row));
			});

			factory.connect_bind(move |_, list_item| {
				let content_object = list_item.downcast_ref::<ListItem>()
					.expect("Should be ListItem")
					.item()
					.and_downcast::<ContentObject>()
					.expect("Item should be `ContentObject`.");
	
				let content_row = list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
				.child()
				.and_downcast::<ContentCol>().expect("Child should be `ContentCol`.");
	
				content_row.bind(&content_object);
			});
	
			factory.connect_unbind(move |_, list_item| {
				let content_row = list_item.downcast_ref::<ListItem>().expect("Should be `ListItem`.")
				.child()
				.and_downcast::<ContentCol>().expect("Child should be `ContentCol`.");
	
				content_row.unbind();
			});
			
			column.set_factory(Some(&factory));
		}
    }
}