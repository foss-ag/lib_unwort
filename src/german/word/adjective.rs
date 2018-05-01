// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use super::{Definition, Gender, Number, Word};
use definable::Definable;

#[derive(Clone)]
pub struct Adjective {
	/// The raw String that describes this word
	raw: String,
	/// Contains the definition, if available
	definition: Option<Definition>,
	/// The Genus this adjective is in or None in case it is in the base form
	gender: Option<Gender>,
	/// If available or applicable, this describes if the Noun this describes is
	/// Singular or Plural
	number: Option<Number>
}

impl Adjective {
	/// Create a new Adjective from its String representation. Everything else
	/// is set to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Verb {
		Verb {
			raw: raw.as_ref().to_string(),
			definition: None,
			gender: None,
			number: None
		}
	}

	/// The gender of the thing this Adjective describes if it is known and
	/// applicable.
	pub fn gender(&self) -> Option<Gender> { self.gender }

	/// If applicable, is this verb in singular or plural
	pub fn number(&self) -> Option<Number> { self.number }
}

impl Word for Verb {
	/// The word class is always Adjective.
	fn class(&self) -> Class { Class::Adjective }

	fn raw(&self) -> &str { &self.raw }
}

impl Definable for Adjective {
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
