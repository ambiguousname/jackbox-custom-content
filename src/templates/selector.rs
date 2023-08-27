use gtk::subclass::prelude::*;
use gtk::{glib, Button, prelude::*};
use std::cell::Cell;

mod imp {

    use super::*;

    #[derive(Default)]
    pub struct Selector {
        pub current_select : Cell<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Selector {
        const NAME : &'static str = "JCCSelector";
        type Type = super::Selector;
        type ParentType = gtk::Box;
    }

    impl ObjectImpl for Selector {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().add_css_class("selector");
        }
    }
    impl WidgetImpl for Selector {}
    impl BoxImpl for Selector {}
}

glib::wrapper! {
    pub struct Selector(ObjectSubclass<imp::Selector>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Selector {
    pub fn new() -> Self {
        glib::Object::new(&[("orientation", &gtk::Orientation::Vertical)])
    }

    pub fn add_selection(&self, name : &str) {
        let button = Button::builder()
        .label(name)
        .build();

        button.connect_clicked(move |this| {
            this.add_css_class("highlight");
        });

        self.append(&button);
    }
}