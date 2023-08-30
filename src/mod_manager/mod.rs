// TODO:
// 1. Store mod folder locations in settings. 
// 2. Load mod data from settings.
// 3. Save lists of mod data.

use crate::content::ContentData;

pub struct JackboxMod {
    pub name : String,
    id : String,
    content_list : Box<Vec<ContentData>>,
}

impl JackboxMod {
    fn get_id(&self) -> String {
        self.id.clone()
    }
}