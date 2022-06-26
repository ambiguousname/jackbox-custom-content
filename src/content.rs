use serde_json::Value;

pub struct Content
{
    id: u32,
    // This is to easily convert to and from JSON/JET format.
    values: Value,
}

// Trait for each of the different content categories (Like Quiplash3Round1Question or Quiplash3SafetyQuips).
// Defines functions specific to each category, and what to do in those cases.
pub trait ContentCategory {
    const CONTENT_NAME : String;
    fn load_content() -> Vec<Content>;
    fn save_as_json(content: &Content) -> String;
}

pub struct ContentWindow
{
    // Name of the window:
    name: String,
    // List of content types that can be edited in the content window.
    // So for Quiplash 3, this would be Quiplash3FinalQuestion, Quiplash3Round1Question, Quiplash3Round2Question, and Quiplash3SafetyQuips.
    // Each "allowed_content" has an associated procedure for loading and displaying (hence a HashMap).
    allowed_content: Vec<String>,
}

pub fn test(){
    println!("Hello!");
}