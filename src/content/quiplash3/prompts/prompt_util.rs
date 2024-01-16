use std::cell::Cell;

use crate::{quick_template, templates::{editable_list::EditableList, content_util::labelled_entry::LabelledEntry}};
use glib::{derived_properties, Properties};
use gtk::glib::once_cell::sync::OnceCell;

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
        EditableList::ensure_type();
        LabelledEntry::ensure_all_types();
        QuiplashGenericRoundPrompt::ensure_type();
    }
}