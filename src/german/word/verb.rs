// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use super::{Class, Definition, Word};
use definable::Definable;

// Representation of a German grammatical person.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Person {
	First,
	Second,
	Third
}

/// Representation of a German grammatical mood.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Mood {
	Indicative,
	ConjunctiveOne,
	ConjunctiveTwo
}

/// The Genus of a verb, part of diathesis.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum VerbGender {
	Active,
	Passive
}

#[derive(Clone)]
pub struct Verb {
	/// The raw String that describes this word
	raw: String,
	/// Contains the definition, if available
	definition: Option<Definition>,
	/// The person this verb stands in, if available
	person: Option<Person>,
	/// The mood of the verb, if available
	mood: Option<Mood>
}

impl Verb {
	/// Create a new Verb from its String representation. Everything else is set
	/// to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Verb {
		Verb {
			raw: raw.as_ref().to_string(),
			definition: None,
			person: None,
			mood: None
		}
	}

	/// The person which is acting with this verb, if it is known
	pub fn person(&self) -> Option<Person> { self.person }

	/// The mood of the verb, such as hearsay mode etc.
	pub fn mood(&self) -> Option<Mood> { self.mood }
}

impl Word for Verb {
	/// The word class is always verb.
	fn class(&self) -> Class { Class::Verb }
}

impl Definable for Verb {
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
