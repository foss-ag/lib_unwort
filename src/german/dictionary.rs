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

//! A German Dictionary, once loaded can be used to check for any word that
//! comes up if it is available in the German language. It should also be able
//! to create any word that might be able to exist in the German language and
//! check against that.

use german::word::Word;
use naive_dictionary::NaiveDictionary;

use std::io::Result as IOResult;

/// A smart dictionary that follows the rules of German
pub struct Dictionary {
	/// The dictionary this is based off of containing only the raw words
	base: NaiveDictionary<Word>
}

impl Dictionary {
	/// Create a new German Dictionary
	pub fn new() -> IOResult<Dictionary> {
		// Open a new Dictionary on the German word file.
		let base = NaiveDictionary::new("lang/de_DE/dict")?;

		Ok(Dictionary {
			base: base
		})
	}

	/// Check if the Dictionary contains this word or any of its derivates
	pub fn contains(&self, word: Word) -> bool {
		self.base.contains(word)
	}
}
