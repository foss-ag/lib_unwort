use word::*;
use std::str::FromStr;
use std::ops::Deref;
use parsing;

pub struct EnglishWord {
	raw: String
}

impl Word for EnglishWord {
	fn typ(&self) -> Option<WordType> { None }
}

impl Deref for EnglishWord {
	type Target = String;

	fn deref(&self) -> &String {
		&self.raw
	}
}

impl ToString for EnglishWord {
	fn to_string(&self) -> String {
		self.raw.clone()
	}
}

impl FromStr for EnglishWord {
	type Err = ();

	fn from_str(s: &str) -> Result<EnglishWord, ()> {
		let mut s = s.trim().to_string();

		// There ought to be no punctuation inside a word, that would make it
		// into two or any whitespace for that matter.
		if parsing::has_punctuation(&s) { return Err(()); }
		if parsing::has_whitespace(&s) { return Err(()); }

		Ok(EnglishWord {
			raw: s
		})
	}
}
