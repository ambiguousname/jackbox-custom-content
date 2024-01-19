use crate::{quick_template, content::{ContentWindow, Content, ContentWindowImpl, ContentWindowExt}, templates::content_util::form_manager::FormManager};

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

        let selected = self.obj().get_selected();
        let map = selected.submit().unwrap();
        
        if callback.is_some() {
            callback.unwrap()("This is a test".to_string());
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

    fn get_selected(&self) -> QuiplashGenericRoundPrompt {
        let idx = self.imp().round_select.current_page();
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