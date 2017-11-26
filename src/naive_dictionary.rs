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

use word::Word;
use std::path::{Path, PathBuf};

use std::io::Result as IOResult;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// Dictionary that does not do any inflections or other fancy stuff, but
/// rather used to compare simply to a database of known base words.
pub struct NaiveDictionary<W: Word> {
	file: PathBuf,
	cache: Vec<W>
}

impl<W: Word> NaiveDictionary<W> {
	/// Create a new NaiveDictionary from the dictionary file provided.
	/// A NaiveDictionary file is a file that contains words, possibly the
	/// most basic version of them seperated by newlines.
	/// The words inside must be sorted alphabetically.
	pub fn new<P: AsRef<Path>>(dict_file: P) -> IOResult<NaiveDictionary<W>> {
		// Open the file and read the contents to the cache
		// TODO: The file should probably not be saved to mem in its entirety
		let file = File::open(&dict_file)?;
		let reader = BufReader::new(file);

		let mut cache = Vec::new();
		for l in reader.lines() {
			let l = l?;
			let word = match W::from_str(&l) {
				Ok(w) => w,
				Err(_) => panic!("Corrupted dictionary file. One word per line only")
			};

			cache.push(word);
		}

		// Make sure the cache is sorted, eventhough it "should" be. Algorithm
		// is O(n) anyway.
		// The function provided is necessary to take advantage of the
		// Deref<String> functionality.
		cache.sort_unstable_by(|a, b| { a.cmp(b) });

		Ok(NaiveDictionary {
			file: dict_file.as_ref().to_path_buf(),
			cache: cache
		})
	}

	/// The dictionary file this dictionary is associated with.
	pub fn file(&self) -> PathBuf {
		self.file.clone()
	}

	/// Checks if the word is inside of this dictionary. Does not check for
	/// derivates, since this Dictionary is naive.
	pub fn has(&self, word: W) -> bool {
		self.cache.binary_search_by(|p| { p.cmp(&word) }).is_ok()
	}
}
