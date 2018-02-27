// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::path::Path;
use std::io;
use std::collections::HashMap;

use is_dict::IsDict;
use dict_config::*;
use german::Word;

/// German dictionary adhering to the rules and particularities of German.
pub struct Dictionary {
	cfg: DictConfig,
	words: HashMap<String, Word>
}

impl IsDict for Dictionary {
	type W = Word;

	fn contains(&self, word: &str) -> bool {
		unimplemented!();
	}

	fn get(&self, word: &str) -> Option<&Word> {
		unimplemented!();
	}

	fn get_mut(&mut self, word: &str) -> Option<&mut Word> {
		unimplemented!();
	}

	fn add(&self, word: &Word) -> bool {
		unimplemented!();
	}
}

impl ConfiguredDict for Dictionary {
	type Error = io::Error;

	fn from_config(cfg: DictConfig) -> io::Result<Dictionary> {
		unimplemented!();
	}

	fn save(&self) -> io::Result<()> {
		unimplemented!();
	}
}
