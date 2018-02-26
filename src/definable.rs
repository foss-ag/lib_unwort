// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::fmt::Display;

/// To save more than just the simple representation of something (f.i. a word)
/// it is often advisable to give it a definition, inherent to this thing.
/// This does not mean, that the definition is always available, but just that
/// it is possible to assign it to this instance.
pub trait Definable {
	/// The type of the definition. It must be displayable.
	type Def: Display; 

	/// Check if a definition is available (yet).
	fn has_definition(&self) -> bool {
		self.definition().is_some()
	}

	/// Get the meaning of this instance.
	fn definition(&self) -> Option<&Self::Def>;

	/// Give the instance a different meaning. If it already has a meaning,
	/// it cannot be removed using this function.
	/// XXX: At the moment there is no way to remove meaning. Is that ok?
	fn set_definition(&mut self, definition: &Self::Def);
}
