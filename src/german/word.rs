// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::str::FromStr;

use definable::Definable;

/// Representation of a German grammatical gender.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Gender {
	None,
	Masculine,
	Feminine,
	Neuter
}

/// Representation of a German grammatical number.
pub enum Number {
	None,
	Singular,
	Plural
}

pub enum Case {
	None,
	Nominative,
	Genitive,
	Dative,
	Accusative
}

// Representation of a German grammatical person.
pub enum Person {
	None,
	First,
	Second,
	Third
}

/// Representation of a German grammatical mood.
pub enum Mood {
	None,
	Indicative,
	ConjunctiveOne,
	ConjunctiveTwo
}

/// Representation of a German grammatical genera.
pub enum Genera {
	None,
	Active,
	Passive
}

/// Describes all parts that may or may not be known about a german word.
pub struct Word {
	/// The raw string that describes this word.
	raw: String,
	/// Contains the definition, if it is available
	definition: Option<String>,
	/// The grammatical gender of the word. Some, if it is known, also if it
	/// does not have a gender (because it is not a noun)
	gender: Option<Gender>,
	/// The grammatical number of the word. Some, if unknown or not necessary
	/// for the word's type.
	number: Option<Number>,

	case: Option<Case>,
	
	person: Option<Person>,

	mood: Option<Mood>,

	genera: Option<Genera>
}

impl Word {
	/// Create a new word from its String representation. Everything else is
	/// set to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Word {
		Word {
			raw: raw.as_ref().to_string(),
			definition: None,
			gender: None,
			number: None,
			case: None,
			person: None,
			mood: None,
			genera: None
		}
	}

	/// Get the grammatical gender of a word. None, if the gender is unknown.
	pub fn gender(&self) -> Option<Gender> {
		self.gender
	}

	/// Get the grammatical number of a word. None, if the number is unknown.
	pub fn number(&self) -> Option<Number> {
		self.number
	}

	pub fn case(&self) -> Option<Case> {
		self.case
	}

	pub fn person(&self) -> Option<Person> {
		self.person
	}

	pub fn mood(&self) -> Option<Mood> {
		self.mood
	}

	pub fn genera(&self) -> Option<Genera> {
		self.genera
	}

	/// Used to set the grammatical gender of a word. Can also be used to reset it.
	pub fn set_gender(&mut self, gender: Gender) {
		self.gender = Some(gender);
	}

	/// Used to set the grammatical number of a word. Can also be used to reset it.
	pub fn set_number(&mut self, number: Number) {
		self.number = Some(number);
	}

	pub fn set_case(&mut self, case: Case) {
		self.case = Some(case);
	}

	pub fn set_person(&mut self, person: Person) {
		self.person = Some(person);
	}

	pub fn set_mood(&mut self, mood: Mood) {
		self.mood = Some(mood);
	}

	pub fn set_genera(&mut self, genera: Genera) {
		self.genera = Some(genera);
	}
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
