use gtk::{ListBox, EditableLabel, glib::{derived_properties, clone}, ListBoxRow};
use glib::{Object, Properties};
use std::cell::RefCell;

use crate::quick_template;

quick_template!(EditableList, "/templates/editable_list.ui", gtk::Box, (gtk::Widget), (),
    #[derive(Default, CompositeTemplate, Properties)]
    #[properties(wrapper_type=super::EditableList)]
    handlers struct {
        #[template_child(id="item-list")]
        pub item_list : TemplateChild<ListBox>,

        #[property(get, set)]
        pub items : RefCell<Vec<String>>,
    }
);

#[derived_properties]
impl ObjectImpl for imp::EditableList {}
impl WidgetImpl for imp::EditableList {}
impl BoxImpl for imp::EditableList {}

#[gtk::template_callbacks]
impl EditableList {
    pub fn ensure_all_types() {
        EditableList::ensure_type();
    }

    pub fn new() -> Self {
        Object::new()
    }

    pub fn items_mut(&self) -> std::cell::RefMut<'_,Vec<String> > { 
        self.imp().items.borrow_mut()
    }

    pub fn has_items(&self) -> bool {
        self.imp().item_list.first_child().is_some()
    }

    pub fn value(&self) -> Vec<String> {
        let mut vec = Vec::<String>::new();

        let mut curr_child = self.imp().item_list.first_child();
        while curr_child.is_some() {
            let row = curr_child.clone().and_downcast::<ListBoxRow>().expect("Could not get ListBoxRow.");
            let label = row.child().and_downcast::<EditableLabel>().expect("Could not get EditableLabel.");
            vec.push(label.text().to_string());

            curr_child = curr_child.unwrap().next_sibling();
        }

        vec
    }

    // Super hacky way of doing this, but I don't want to have to manage two separate lists with a bunch of signals.
    // Just connect every signal that updates the list to this:
    pub fn update_items(&self) {
        self.set_items(self.value());
    }

    #[template_callback]
    fn handle_new_list_item(&self) {
        let editable = EditableLabel::new("Double click to edit");
        editable.connect_changed(clone!(@weak self as w => move |_| {
            w.update_items();
        }));
        
        self.update_items();

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
            self.update_items();
        }
    }
}