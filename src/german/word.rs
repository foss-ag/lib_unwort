// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use definable::Definable;

/// Describes all parts that may or may not be known about a german word.
pub struct Word {
	/// The raw string that describes this word.
	raw: String,
	/// Contains the definition, if it is available
	definition: Option<String>
}

impl Definable for Word {
	type Def = String;

	fn definition(&self) -> Option<&String> {
		match &self.definition {
			&Some(ref def) => Some(&def),
			&None => None
		}
	}

	fn set_definition(&mut self, definition: &String) {
		self.definition = Some(definition.clone());
	}
}
