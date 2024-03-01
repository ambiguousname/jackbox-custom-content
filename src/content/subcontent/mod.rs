pub mod manifest;

pub trait Subcontent {
	/// Called when the Subcontent should be written to the mod folder.
	fn write_to_mod(&self);
	/// Called when the Subcontent should be written to the game folder.
	fn write_to_game(&self);
}
