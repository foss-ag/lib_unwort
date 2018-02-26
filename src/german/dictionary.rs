// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::path::Path;
use std::io;

use is_dict::IsDict;
use german::Word;

pub struct Dictionary {
	
}

impl IsDict for Dictionary {
	type W = Word;

	fn from_file(path: &Path) -> io::Result<Self> {
		unimplemented!();
	}
	
	fn to_file(&self, path: &Path) -> io::Result<()> {
		unimplemented!();
	}

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
