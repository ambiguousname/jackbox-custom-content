use gtk::{ListBox, EditableLabel};
use glib::Object;

use crate::quick_template;

quick_template!(EditableList, "/templates/editable_list.ui", gtk::Box, (gtk::Widget), (),
    #[derive(Default, CompositeTemplate)]
    handlers struct {
        #[template_child(id="item-list")]
        pub item_list : TemplateChild<ListBox>,
    }
);

impl ObjectImpl for imp::EditableList {}
impl WidgetImpl for imp::EditableList {}
impl BoxImpl for imp::EditableList {}

#[gtk::template_callbacks]
impl EditableList {
    pub fn new() -> Self {
        Object::new()
    }

    #[template_callback]
    fn handle_new_list_item(&self) {
        let editable = EditableLabel::new("Double click to edit");
        self.imp().item_list.append(&editable);
    }

    #[template_callback]
    fn handle_remove_list_item(&self) {
        let item_list = self.imp().item_list.clone();
        let selected = item_list.selected_row();
        let row = selected.or(item_list.last_child().and_downcast());
        if row.is_some() {
            let list_box_row = row.unwrap();
            item_list.remove(&list_box_row);
        }
    }
}