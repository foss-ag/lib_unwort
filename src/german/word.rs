// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::ops::Deref;

use definable::Definable;

/// Representation of a German grammatical word type.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum WordType {
	Noun,
	Verb,
	Adjective
}

/// Representation of a German grammatical gender.
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

/// Representation of a German grammatical case.
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
#[derive(Clone)]
pub struct Word {
	/// The raw string that describes this word.
	raw: String,
	/// Contains the definition, if it is available
	definition: Option<String>,
	/// The type of the word. Some, if it is known
	word_type: Option<WordType>,
	/// The grammatical gender of the word.
	gender: Option<Gender>,
	/// The grammatical number of the word.
	number: Option<Number>,
	/// The grammatical case of the word. 
	case: Option<Case>,
	/// The grammatical person of the word.
	person: Option<Person>,
	/// the grammatical mood of the word.
	mood: Option<Mood>,
	/// The grammatical word of the genera.
	genera: Option<Genera>
}

impl Word {
	/// Create a new word from its String representation. Everything else is
	/// set to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Word {
		Word {
			raw: raw.as_ref().to_string(),
			definition: None,
			word_type: None,
			gender: None,
			number: None,
			case: None,
			person: None,
			mood: None,
			genera: None
		}
	}

	/// Get the grammatical type of a word. None, if the type is unknown.
	pub fn word_type(&self) -> Option<WordType> {
		Some(self.word_type)
	}

	/// Get the grammatical gender of a word. None, if the gender is unknown.
	pub fn gender(&self) -> Option<Gender> {
		Some(self.gender)
	}

	/// Get the grammatical number of a word. None, if the number is unknown.
	pub fn number(&self) -> Option<Number> {
		Some(self.number)
	}

	/// Get the grammatical case of a word. None, if the case is unknown.
	pub fn case(&self) -> Option<Case> {
		Some(self.case)
	}

	/// Get the grammatical person of a word. None, if the case is unknown.
	pub fn person(&self) -> Option<Person> {
		Some(self.person)
	}

	//7 Get the grammatical mood of a word. None, if the case is unknown.
	pub fn mood(&self) -> Option<Mood> {
		Some(self.mood)
	}

	/// Get the grammatical genera of a word. None, if the case is unknown.
	pub fn genera(&self) -> Option<Genera> {
		Some(self.genera)
	}

	/// Used to set the grammatical type of a word. Can also be used to reset it.
	pub fn set_word_type(&mut self, word_type: WordType) {
		self.word_type = Some(word_type);
	}

	/// Used to set the grammatical gender of a word. Can also be used to reset it.
	pub fn set_gender(&mut self, gender: Gender) {
		self.gender = Some(gender);
	}

	/// Used to set the grammatical number of a word. Can also be used to reset it.
	pub fn set_number(&mut self, number: Number) {
		self.number = Some(number);
	}

	/// Used to set the grammatical case of a word. Can also be used to reset it.
	pub fn set_case(&mut self, case: Case) {
		self.case = Some(case);
	}

	/// Used to set the grammatical person of a word. Can also be used to reset it.
	pub fn set_person(&mut self, person: Person) {
		self.person = Some(person);
	}

	/// Used to set the grammatical mood of a word. Can also be used to reset it.
	pub fn set_mood(&mut self, mood: Mood) {
		self.mood = Some(mood);
	}

	/// Used to set the grammatical genera of a word. Can also be used to reset it.
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

impl Deref for Word {
	type Target = str;

	fn deref(&self) -> &str {
		&self.raw
	}
}
