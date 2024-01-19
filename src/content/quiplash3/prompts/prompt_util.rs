use std::{cell::Cell, collections::HashMap};

use crate::{quick_template, templates::{editable_list::EditableList, content_util::{labelled_box::LabelledBox, form_manager::FormManager}}};
use glib::{derived_properties, Properties};

quick_template!(QuiplashGenericRoundPrompt, "/content/quiplash3/prompts/generic_round_prompt.ui", gtk::Box, (gtk::Widget), (),
    #[derive(CompositeTemplate, Default, Properties)]
    #[properties(wrapper_type=super::QuiplashGenericRoundPrompt)]
    struct {
        #[property(get, set)]
        pub final_round : Cell<bool>,

        #[template_child(id="filter_text")]
        pub filter_text : TemplateChild<gtk::Box>,
        #[template_child(id="filter_ogg")]
        pub filter_ogg : TemplateChild<gtk::Box>,
        #[template_child(id="filter_transcript")]
        pub filter_transcript : TemplateChild<gtk::Box>,

        #[template_child(id="form_manager")]
        pub form_manager : TemplateChild<FormManager>,
    }
);

#[derived_properties]
impl ObjectImpl for imp::QuiplashGenericRoundPrompt {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();

        obj.bind_property::<gtk::Box>("final-round", obj.imp().filter_ogg.as_ref(), "visible").invert_boolean().sync_create().build();
        obj.bind_property::<gtk::Box>("final-round", obj.imp().filter_text.as_ref(), "visible").invert_boolean().sync_create().build();
        obj.bind_property::<gtk::Box>("final-round", obj.imp().filter_transcript.as_ref(), "visible").invert_boolean().sync_create().build();

        // let final_round = self.final_round.get();
        
        // let is_final = final_round.is_some_and(|b| *b);
        // println!("{} {}", is_final, final_round.is_some());
        // if is_final {
        //     self.filter_ogg.set_visible(false);
        //     self.filter_text.set_visible(false);
        //     self.filter_transcript.set_visible(false);
        // }
    }
}
impl WidgetImpl for imp::QuiplashGenericRoundPrompt {}
impl BoxImpl for imp::QuiplashGenericRoundPrompt {}

impl QuiplashGenericRoundPrompt {
    pub fn ensure_all_types() {
        FormManager::ensure_all_types();
        QuiplashGenericRoundPrompt::ensure_type();
    }

    pub fn submit(&self) -> Option<HashMap<String, glib::Value>> {
        self.imp().form_manager.submit()
    }

    pub fn is_valid(&self) -> bool {
        self.imp().form_manager.is_valid()
    }
}