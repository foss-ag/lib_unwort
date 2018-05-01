// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use super::{Definition, Gender, Number};
use definable::Definable;

/// The case of a German noun
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Case {
	Nominative,
	Genitive,
	Dative,
	Accusative
}

/// A specific German noun. Some words can be derived from this if they are
/// sufficiently similar to it, especially if they share a stem.
pub struct Noun {
	/// The raw String that describes this word
	raw: String,
	/// Contains the definition, if it is available
	definition: Option<Definition>,
	/// The gender of this Noun, if it is known
	gender: Option<Gender>,
	/// If known, contains if this Noun is in Plural or Singular form
	number: Option<Number>,
	/// The grammatical case this Noun is currently in
	case: Option<Case>
}

impl Noun {
	/// Create a new Noun from its String representation. Everything else is set
	/// to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Noun {
		Noun {
			raw: raw.as_ref().to_string(),
			definition: None,
			gender: None,
			number: None,
			case: None
		}
	}

	/// The grammatical Gender of this Noun if known.
	pub fn gender(&self) -> Option<Gender> { self.gender }

	/// Get if this noun is plural or singular. None if it is unknown.
	pub fn number(&self) -> Option<Number> { self.number }

	/// The case this Noun is in if it is known.
	pub fn case(&self) -> Option<Case> { self.case }
}

impl Definable for Noun {
	fn definition(&self) -> Option<&Definition> {
		match &self.definition {
			&Some(ref def) => Some(&def),
			&None => None
		}
	}

	fn set_definition(&mut self, definition: &Definition) {
		self.definition = Some(definition.clone());
	}
}

