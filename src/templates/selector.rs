use gtk::subclass::prelude::*;
use gtk::{glib, prelude::*, CssProvider};
use std::cell::RefCell;
use std::sync::Once;
use glib::subclass::Signal;

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

mod imp {

    use super::*;

    #[derive(Default)]
    pub struct Selector {
        pub current_select : RefCell<Option<SelectorButton>>,
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

    pub fn add_selection<F>(&self, name : &str, callback : F) 
    where F : Fn(&[glib::Value]) -> Option<glib::Value> + Send + Sync + 'static
    {
        let button = SelectorButton::new(name);

        button.connect("selected", false, callback);

        if (self.first_child().is_none()) {
            button.add_css_class("highlight");
            self.imp().current_select.replace(Some(button.clone()));
        }

        self.append(&button);
    }

    pub fn get_selected(&self) -> glib::GString {
        let b = self.imp().current_select.borrow().clone().expect("Could not get currently selected.");
        b.property::<glib::GString>("label")
    }

    pub fn selected_callback(&self) {
        self.imp().current_select.borrow().clone().expect("Could not get currently selected").emit_by_name::<()>("selected", &[&self]);
    }
}

// region: Selector Button custom definition

mod button_imp {
    use std::sync::OnceLock;

    use super::*;

    #[derive(Default)]
    pub struct SelectorButton {}

    #[glib::object_subclass]
    impl ObjectSubclass for SelectorButton {
        const NAME : &'static str = "JCCSelectorButton";
        type Type = super::SelectorButton;
        type ParentType = gtk::Button;
    }
    impl ObjectImpl for SelectorButton{
        fn constructed(&self) {
            self.parent_constructed();
            self.obj().style_context().add_provider(&Selector::grab_css(), gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        }
        
        fn signals() -> &'static [Signal] {
            static SIGNALS : OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("selected").param_types([Selector::static_type()]).build()]
            })
        }
    }
    impl WidgetImpl for SelectorButton{}
    impl ButtonImpl for SelectorButton{
        fn clicked(&self) {
            self.parent_clicked();
            let this = self.obj();
            let selector_parent = this.parent().expect("Could not get button parent.").downcast::<Selector>().expect("Could not get Selector parent.");
            let prev_selected = selector_parent.imp().current_select.take().expect("Could not get selected button.");
            prev_selected.remove_css_class("highlight");
            selector_parent.imp().current_select.replace(Some(this.clone()));
            this.add_css_class("highlight");
        }
    }
}

glib::wrapper! {
    pub struct SelectorButton(ObjectSubclass<button_imp::SelectorButton>) @extends gtk::Box, gtk::Widget, @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl SelectorButton {
    fn new(name : &str) -> Self {
        glib::Object::new(&[("label", &name)])
    }
}
// endregion