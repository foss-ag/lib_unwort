use english::word::EnglishWord;
use naive_dictionary::NaiveDictionary;

use std::io::Result as IOResult;


pub struct Dictionary {
	base: NaiveDictionary<EnglishWord>
}

impl Dictionary {
	pub fn new() -> IOResult<Dictionary> {
		// Open a new Dictionary in English mode.
		let base = NaiveDictionary::new("dict/english")?;

		Ok(Dictionary {
			base: base
		})
	}

	fn has(&self, word: EnglishWord) -> bool {
		false
	}
}
