// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::io;

use german::compound;
use is_dict::IsDict;
use dict_config::*;
use german::Word;

/// German dictionary adhering to the rules and particularities of German.
pub struct Dictionary {
	cfg: Option<DictConfig>,
	words: Vec<Word>
}

impl Dictionary {
	pub fn new() -> Dictionary {
		Dictionary {
			cfg: None,
			words: Vec::new()
		}
	}
}

impl IsDict for Dictionary {
	type W = Word;

	fn contains(&self, word: &str) -> bool {
		// The word is already contained as full so we can wrap this up easily.
		if self.get(&word).is_some() {
			return true;
		}

		// TODO: This currently ignores different endings.
		compound::is_possible(word, &self.words)
	}

	fn get(&self, word: &str) -> Option<&Word> {
		// TODO: Make this into a function to get the word position, so it does
		// not have to be copied.
		let pos = match self.words.binary_search_by(|w| { w.cmp(&word) }) {
			Ok(pos) => pos,
			Err(_) => return None
		};

		self.words.get(pos)
	}

	fn get_mut(&mut self, word: &str) -> Option<&mut Word> {
		let pos = match self.words.binary_search_by(|w| { w.cmp(&word) }) {
			Ok(pos) => pos,
			Err(_) => return None
		};

		self.words.get_mut(pos)
	}

	fn add(&mut self, word: &Word) -> bool {
		match self.words.binary_search_by(|w| { w.cmp(&word) }) {
			Ok(_) => false,
			Err(pos) => {
				self.words.insert(pos, word.clone());
				true
			}
		}
	}
}

impl ConfiguredDict for Dictionary {
	type Error = io::Error;

	fn from_config(cfg: DictConfig) -> io::Result<Dictionary> {
		// Open the file containing the raw words. This will be done with a
		// parser later, but for now this must suffice.
		unimplemented!()
	}

	fn save(&self) -> io::Result<()> {
		unimplemented!();
	}
}
