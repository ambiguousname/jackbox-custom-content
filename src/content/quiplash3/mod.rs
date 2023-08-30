mod categories;

use categories::{round_prompt::QUIPLASH_PROMPT, safety_quip::QUIPLASH_SAFETY};

use super::{GameContent, ContentCategory};

pub const QUIPLASH_CATEGORIES : [ContentCategory; 2] = [QUIPLASH_PROMPT, QUIPLASH_SAFETY];

pub const GAME_INFO: GameContent = GameContent {
    game_id: "Quiplash3",
    name: "Quiplash 3",
    content_categories: &QUIPLASH_CATEGORIES,
};