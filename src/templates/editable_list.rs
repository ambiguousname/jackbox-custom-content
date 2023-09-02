use gtk::{ListBox, EditableLabel};

use crate::quick_template;

quick_template!(EditableList, "/templates/editable_list.ui", gtk::Frame, (gtk::Widget), (), handlers struct {
    #[template_child(id="item-list")]
    pub item_list : TemplateChild<ListBox>,
});

impl ObjectImpl for imp::EditableList {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
    }
}
impl WidgetImpl for imp::EditableList {}
impl FrameImpl for imp::EditableList {}

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
}