use std::collections::HashMap;

use crate::{content::{subcontent::{manifest::ManifestItem, Subcontent}, Content, ContentWindow, ContentWindowExt, ContentWindowImpl}, quick_template};

mod prompt_util;
use gtk::Notebook;
use prompt_util::QuiplashGenericRoundPrompt;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Quiplash3RoundManifestItem {
    includes_player_name: bool,
    prompt: String,
    safety_quips: Vec<String>,
    us: bool,
    x: bool
}

impl ContentWindowImpl for imp::QuiplashRoundPrompt {
    fn finalize_content(&self, callback : crate::content::ContentCallback) {
        let obj = self.obj();

        let selected = obj.get_selected();
        let map = selected.submit().unwrap();

        let mut subcontent_vec = Vec::new();

        let prompt_text = map.get("Prompt Text").and_then(|text| {
            text.get::<String>().ok()
        }).unwrap();

        let player_name = map.get("Includes Player Name").and_then(|bool_val| {
            bool_val.get::<bool>().ok()
        }).unwrap();

        let quips = map.get("Safety Quips").and_then(|quips| {
            quips.get::<Vec<String>>().ok()
        }).unwrap();

        let us = map.get("Content is US-Specific").and_then(|us| {
            us.get::<bool>().ok()
        }).unwrap();

        let x = map.get("Contains Adult Content").and_then(|x| {
            x.get::<bool>().ok()
        }).unwrap();


        let manifest_data = Quiplash3RoundManifestItem {
            includes_player_name: player_name,
            prompt: prompt_text,
            safety_quips: quips,
            us,
            x
        };

        let quip_manifest = ManifestItem::new(serde_json::to_value(manifest_data).unwrap());
        let quip_box : Box<dyn Subcontent> = Box::new(quip_manifest);
        subcontent_vec.push(quip_box);
        
        let round_str = match obj.get_selected_idx() {
            Some(0) => "Round1",
            Some(1) => "Round2",
            Some(2) => "FinalRound",
            _ => unreachable!("Invalid round selection found"),
        };
        (callback)(round_str.to_string(), subcontent_vec);
    }

    fn load_content(&self, subcontent_type : String, subcontent : Vec<crate::content::SubcontentBox>) -> Result<(), String> {
        let obj = self.obj();
        
        let selected = obj.get_selected();

        let mut values : HashMap<String, glib::Value> = HashMap::new();
        
        let manifest_item = subcontent[0].downcast_ref::<ManifestItem>().expect("Could not get manifest item.");
        let read_manifest = serde_json::from_value::<Quiplash3RoundManifestItem>(manifest_item.content());

        if read_manifest.is_err() {
            return Err(format!("Could not read manifest."));
        }

        let quiplash_manifest = read_manifest.unwrap();
        

        values.insert(String::from("Prompt Text"), quiplash_manifest.prompt.to_value());
        values.insert(String::from("Includes Player Name"), quiplash_manifest.includes_player_name.to_value());
        values.insert(String::from("Safety Quips"), quiplash_manifest.safety_quips.to_value());
        values.insert(String::from("Content is US-Specific"), quiplash_manifest.us.to_value());
        values.insert(String::from("Contains Adult Content"), quiplash_manifest.x.to_value());

        selected.update_form(values);

        obj.set_selected_idx(match subcontent_type.as_str() {
            "Round1" => Some(0),
            "Round2" => Some(1),
            "FinalRound" => Some(2),
            _ => None,
        });
        Ok(())
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

    fn set_selected_idx(&self, idx : Option<u32>) {
        self.imp().round_select.set_current_page(idx);
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