use std::{any::Any, fmt::Debug};

pub mod manifest;

// TODO: How does this work for:
// Making new content
// Editing
// Loading

/// If we have a game folder (e.g., Quiplash3), there's chunks that belong to each [`super::ContentWindow`].
/// [`Subcontent`] is a way to organize each chunk. They indicate how to load and write the chunk.
/// Each [`Subcontent`] represents one file or folder, but it doesn't have to represent everything that it contains.
/// Depending on how [`Subcontent::write_to_mod`] is implemented, it could merge or replace existing chunks.
pub trait Subcontent : Debug {
	/// Called when the Subcontent should be written to the mod folder.
	fn write_to_mod(&self, id: String, args : Vec<&'static str>) -> std::io::Result<()>;
	/// Called when the Subcontent should be written to the game folder.
	fn write_to_game(&self);
	/// Called when reading the mod folder.
	fn load_from_dir(&self);

	fn as_any(&self) -> &dyn Any;
}

impl dyn Subcontent   {
	pub fn downcast_ref<T : Subcontent + Any>(&self) -> Option<&T> {
		self.as_any().downcast_ref::<T>()
	}
}