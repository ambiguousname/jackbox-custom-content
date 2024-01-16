use crate::quick_object;

quick_object!(Entry, gtk::Box, (gtk::Widget), (gtk::Root, gtk::Buildable), 
	#[derive(Default)]
	struct {

	}
);

impl ObjectImpl for imp::Entry {}
impl WidgetImpl for imp::Entry {}
impl BoxImpl for imp::Entry {}