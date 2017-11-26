// unwort - A rule set-based language correction library.
// Copyright (C) 2017 FOSS-AG TU Dortmund
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
// 
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
// 
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301
// USA

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
