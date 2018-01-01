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

use rust_stemmers::{Algorithm, Stemmer};

use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, BufWriter};

use std::collections::HashSet;
use std::io::Result as IOResult;

/// A smart dictionary that follows the rules of German
pub struct Dictionary {
	/// The base dictionary file this is created from
	file: PathBuf,
	/// The dictionary this is based off of containing only the stems
	known: HashSet<String>,
	/// The stemmer that is used to get at the raw stem of the word
	stemmer: Stemmer
}

impl Dictionary {
	/// Create a new German Dictionary
	pub fn new<P: AsRef<Path>>(dict_file: P) -> IOResult<Dictionary> {
		// Open the base dictionary file and read it's contents into memory
		let file = OpenOptions::new().read(true).create(true).open(&dict_file)?;
		let reader = BufReader::new(file);

		let mut known = HashSet::new();
		for l in reader.lines() {
			let l = l?;
			known.insert(l.trim().to_string());
		}

		Ok(Dictionary {
			file: dict_file.as_ref().to_path_buf(),
			known: known,
			stemmer: Stemmer::create(Algorithm::German)
		})
	}

	/// Add the word to the dictionary. Returns true if it is a new word, false
	/// otherwise.
	pub fn add(&mut self, word: &str) -> bool {
		// Reduce the word to it's stem.
		let stem = self.stemmer.stem(&word.trim().to_string()).into_owned();
		self.known.insert(stem)
	}

	/// Check if the Dictionary contains this word or any of its derivates
	pub fn contains(&self, word: &str) -> bool {
		self.known.contains(&self.stemmer.stem(word).into_owned().to_string())
	}

	/// Save the dictionary file back to were it was loaded from.
	// TODO: This is a bit stupid. I don't like multiple instances of
	// the same dictionary being loaded and then saving into the same file again
	pub fn save(&self) -> IOResult<()> {
		let file = File::open(&self.file)?;
		let mut writer = BufWriter::new(file);

		for ref k in &self.known {
			writer.write(&k.as_bytes())?;
		}

		// Check if the write was successful
		writer.flush()?;

		Ok(())
	}
}

impl Drop for Dictionary {
	fn drop(&mut self) {
		if self.save().is_err() {
			println!("Failed to save dictionary file");
		}
	}
}
