use crate::quick_template;

quick_template!(EditableList, "/templates/editable_list.ui", gtk::Frame, (gtk::Widget), ());

impl ObjectImpl for imp::EditableList {}
impl WidgetImpl for imp::EditableList {}
impl FrameImpl for imp::EditableList {}
