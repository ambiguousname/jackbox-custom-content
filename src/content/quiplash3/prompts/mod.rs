use std::sync::OnceLock;

use crate::{content::{subcontent::{manifest::Manifest, Subcontent}, Content, ContentWindow, ContentWindowExt, ContentWindowImpl}, quick_template};

mod prompt_util;
use gtk::Notebook;
use prompt_util::QuiplashGenericRoundPrompt;

// TODO: Transfer prompt data across notebooks?
quick_template!(QuiplashRoundPrompt, "/content/quiplash3/prompts/round_prompt.ui", ContentWindow, (gtk::Window, gtk::Widget, Content), (gtk::Native, gtk::Root, gtk::ShortcutManager),
    #[derive(Default, CompositeTemplate)]
    handlers struct {
        #[template_child(id="round_select")]
        pub round_select : TemplateChild<Notebook>,
    }
);

impl ObjectImpl for imp::QuiplashRoundPrompt {}
impl WidgetImpl for imp::QuiplashRoundPrompt {}
impl WindowImpl for imp::QuiplashRoundPrompt {}
impl ContentWindowImpl for imp::QuiplashRoundPrompt {
    fn finalize_content(&self, callback : Option<crate::content::ContentCallback>) {
        let obj = self.obj();

        let selected = obj.get_selected();
        let map = selected.submit().unwrap();

        let mut subcontent_vec = Vec::new();

        let prompt_text = map.get("Prompt Text").and_then(|text| {
            text.get::<String>().ok()
        }).unwrap();

        let quip_manifest = Manifest::new(
            match obj.get_selected_idx() {
                Some(0) => Some("Quiplash3Round1Question.jet".to_string()),
                Some(1) => Some("Quiplash3Round2Question.jet".to_string()),
                Some(2) => Some("Quiplash3FinalQuestion.jet".to_string()),
                _ => None
            }
        );
        let quip_box : Box<dyn Subcontent> = Box::new(quip_manifest);
        subcontent_vec.push(quip_box);
        
        if callback.is_some() {
            callback.unwrap()(subcontent_vec);
        }
    }
}

#[gtk::template_callbacks]
impl QuiplashRoundPrompt {
    // This is here for visibility by the automated build/content_list.rs function.
    pub fn ensure_all_types() {
        QuiplashGenericRoundPrompt::ensure_all_types();
        QuiplashRoundPrompt::ensure_type();
    }

    fn get_selected_idx(&self) -> Option<u32> {
        self.imp().round_select.current_page()
    }

    fn get_selected(&self) -> QuiplashGenericRoundPrompt {
        let idx = self.get_selected_idx();
        self.imp().round_select.nth_page(idx).and_downcast::<QuiplashGenericRoundPrompt>().expect("Could not get QuiplashGenericRoundPrompt.")
    }

    #[template_callback]
    pub fn handle_create_clicked(&self) {
        // Put a call to ContentWindowImpl, with a stored callback (as explained in content/mod.rs)

        
        if self.get_selected().is_valid() {
            self.finalize_content();
        }
    }
}