use std::collections::HashMap;
use serde_json::Value;

struct Content
where
{
    id: u32,
    // This is to easily convert to and from JSON/JET format.
    values: Value,
}

// Trait for each of the different content categories (Like Quiplash3Round1Question or Quiplash3SafetyQuips).
// Defines functions specific to each category, and what to do in those cases.
pub trait ContentCategory {
    fn load_content() -> Vector<&Content>;
    fn save_as_json(content: &Content) -> String;
}

struct ContentWindow<T>
where
    T: Fn(&self)
{
    // Name of the window:
    name: String,
    // List of content types that can be edited in the content window.
    // So for Quiplash 3, this would be Quiplash3FinalQuestion, Quiplash3Round1Question, Quiplash3Round2Question, and Quiplash3SafetyQuips.
    // Each "allowed_content" has an associated procedure for loading and displaying (hence a HashMap).
    allowed_content: HashMap<String, &ContentList>,

}