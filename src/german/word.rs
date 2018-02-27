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

/// Describes all parts that may or may not be known about a german word.
pub struct Word {
	/// The raw string that describes this word.
	raw: String,
	/// Contains the definition, if it is available
	definition: Option<String>,
	/// The grammatical gender of the word. Some if it is known, also if it
	/// does not have a gender (because it is not a noun)
	gender: Option<Gender>
}

impl Word {
	/// Create a new word from its String representation. Everything else is
	/// set to unknown.
	pub fn new<R: AsRef<str>>(raw: R) -> Word {
		Word {
			raw: raw.as_ref().to_string(),
			definition: None,
			gender: None
		}
	}

	/// Get the gender of this word. None if the gender is unknown.
	pub fn gender(&self) -> Option<Gender> {
		self.gender
	}

	/// Once the gender is known, it can be known or changed. It cannot be
	/// reset using this function.
	pub fn set_gender(&mut self, gender: Gender) {
		self.gender = Some(gender);
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

impl ToString for Word {
	fn to_string(&self) -> String {
		let mut r = self.raw.clone();

		if let Some(gender) = self.gender {
			let g_str = match gender {
				Gender::Masculine => "m",
				Gender::Feminine => "f",
				Gender::Neuter => "n",
				Gender::None => "-"
			};

			r.push_str(format!(", {}", g_str));
		}

		r
	}
}

#[derive(Debug)]
pub enum FormatError {
	UnexpectedToken(String),
	InvalidArgument(String, String),
}

impl FromStr for Word {
	type Err = FormatError;

	fn from_str(s: &str) -> Result<Word, FormatError> {
		
	}
}
