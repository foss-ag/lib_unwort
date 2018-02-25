// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::ops::Deref;

/// To save more than just the string representation of a word, this type is
/// used. It contains certain characteristics inherent to every word, which
/// are not guaranteed to be known. As per default, only the string
/// representation *must* be available.
pub struct Word {
	/// The string representing the word. The written form.
	raw: String,
	/// The definition text describing the word.
	definition: Option<String>
}

impl Word {
	/// Check if a definition for this word is available.
	pub fn has_defition(&self) -> bool {
		self.definition.is_some()
	}

	/// Get the definition of the word.
	pub fn definition(&self) -> Option<&String> {
		match &self.definition {
			&Some(ref d) => Some(&d),
			&None => None
		}
	}

	/// Change the definition of the word. An existing definition cannot be
	/// erased using this function.
	pub fn set_definition(&mut self, definition: &str) {
		self.definition = Some(definition.to_string());
	}
}

impl Deref for Word {
	type Target = str;

	/// Dereference the word into its simple string representation.
	fn deref(&self) -> &str {
		&self.raw
	}
}
