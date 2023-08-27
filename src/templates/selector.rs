use gtk::subclass::prelude::*;
use gtk::{glib, Button, prelude::*, CssProvider};
use std::cell::Cell;
use std::sync::Once;

mod imp {

    use super::*;

    #[derive(Default)]
    pub struct Selector {
        pub current_select : Cell<Option<Button>>,
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


const CSS : &str  = 
"
.selector .text-button {
    border: 0.1px solid;
    box-shadow: none;
}

.selector .text-button.highlight {
    background: @theme_selected_bg_color;
    color: @theme_selected_fg_color; /* Using built-in GTK theme colors. https://github.com/surajmandalcell/Gtk-Theming-Guide/blob/master/creating_gtk_themes.md */
}

";

thread_local! {
    static CSS_DAT : CssProvider = CssProvider::new();
}
static CSS_INIT : Once = Once::new();

impl Selector {
    pub fn new() -> Self {
        glib::Object::new(&[("orientation", &gtk::Orientation::Vertical)])
    }

    fn grab_css() -> CssProvider {
        CSS_INIT.call_once(|| {
            CSS_DAT.with(|provider| {
                provider.load_from_data(CSS.as_bytes());
                println!("Selector CSS loaded.");
            });
        });
        CSS_DAT.with(|provider| {
            provider.clone()
        })
    }

    pub fn add_selection(&self, name : &str) {
        let button = Button::builder()
        .label(name)
        .build();

        button.style_context().add_provider(&Selector::grab_css(), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

        button.connect_clicked(move |this| {
            let selector_parent = this.parent().expect("Could not get button parent.").downcast::<Selector>().expect("Could not get Selector parent.");
            let prev_selected = selector_parent.imp().current_select.take().expect("Could not get selected button.");
            prev_selected.remove_css_class("highlight");
            selector_parent.imp().current_select.replace(Some(this.clone()));
            this.add_css_class("highlight");
        });

        if (self.first_child().is_none()) {
            button.add_css_class("highlight");
            self.imp().current_select.replace(Some(button.clone()));
        }

        self.append(&button);
    }
}