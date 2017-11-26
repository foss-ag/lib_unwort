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

//! An English Dictionary, once loaded can be used to check for any word that
//! comes up if it is available in the english language. It should also be able
//! to create any word that might be able to exist in the English language and
//! check against that.

use english::word::EnglishWord;
use naive_dictionary::NaiveDictionary;

use std::io::Result as IOResult;

/// A smart dictionary that follows the rules of English
pub struct Dictionary {
	/// The dictionary this is based off of containing only the raw words
	base: NaiveDictionary<EnglishWord>
}

impl Dictionary {
	/// Create a new English Dictionary
	pub fn new() -> IOResult<Dictionary> {
		// Open a new Dictionary in English mode.
		let base = NaiveDictionary::new("dict/english")?;

		Ok(Dictionary {
			base: base
		})
	}

	/// Check if the Dictionary contains this word or any of its derivates
	pub fn has(&self, word: EnglishWord) -> bool {
		false
	}
}
