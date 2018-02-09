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

use dictionary::Dictionary as Super;

use rust_stemmers::{Algorithm, Stemmer};

use std::path::{Path, PathBuf};
<<<<<<< HEAD
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, BufWriter};
=======
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, BufWriter};
>>>>>>> b2ec8e7e42c0c8cfc9ef96d42a43025b27a01a27

use std::collections::HashSet;

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
	pub fn new<P: AsRef<Path>>(dict_file: P) -> io::Result<Dictionary> {
		// Look if the dictionary file already exists. If that is the case,
		// read its contents into memory. This does not bother with creating
		// a new file if none exists, since that is covered in `save()`.
		let known = match File::open(&dict_file) {
			Ok(f) => {
				let reader = BufReader::new(f);

				let mut known = HashSet::new();
				for l in reader.lines() {
					let l = l?;
					known.insert(l.trim().to_string());
				}

				known
			},
			Err(e) => match e.kind() {
				io::ErrorKind::NotFound => HashSet::new(),
				_ => return Err(e)
			}
		};

		Ok(Dictionary {
			file: dict_file.as_ref().to_path_buf(),
			known: known,
			stemmer: Stemmer::create(Algorithm::German)
		})
	}

	/// Save the dictionary file back to were it was loaded from.
	// TODO: This is a bit stupid. I don't like multiple instances of
	// the same dictionary being loaded and then saving into the same file again
	pub fn save(&self) -> io::Result<()> {
		let file = File::create(&self.file)?;
		let mut writer = BufWriter::new(file);

		for ref k in &self.known {
			writer.write(&k.as_bytes())?;
			writer.write(b"\n")?;
		}

		// Check if the write was successful
		writer.flush()?;

		Ok(())
	}
}

impl Super for Dictionary {
	fn add(&mut self, word: &str) -> bool {
		// Reduce the word to it's stem.
		let stem = self.stemmer.stem(word.trim()).into_owned();
		self.known.insert(stem)
	}

	fn contains(&self, word: &str) -> bool {
		self.known.contains(&self.stemmer.stem(word).into_owned().to_string())
	}
}

impl Drop for Dictionary {
	fn drop(&mut self) {
		if self.save().is_err() {
			println!("Failed to save dictionary file");
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::fs;

	#[test]
	// Testing the dictionary in case there is no file existing yet and the
	// dictionary is created from scratch.
	fn new_file_case() {
		let fname = "test_new_file_case_ger";

		{
			let mut dict = Dictionary::new(fname).expect("Dict should not care that there is no file");
			dict.add("Guten");
			dict.add("Morgen");

			dict.save().expect("Could not write Dictionary file");
		}

		fs::remove_file(fname).expect("Could not clean up. Please manually remove test files");
	}

	#[test]
	// If there is already a dictionary file, the contents must be read and
	// new words added correctly without corrupting the old file.
	fn existing_file_case() {
		let fname = "test_existing_file_case_ger";

		{
			// Create an existing file with some contents.
			let mut dict = Dictionary::new(fname).expect("Dict should not care that there is no file");
			dict.add("Guten");
			dict.add("Morgen");

			dict.save().expect("Could not write Dictionary file");
		}

		{
			// Now use the existing file to create a new dictionary and check if
			// the contents are appreciated by it.
			let mut dict = Dictionary::new(fname).expect("Could not create from existing file");

			assert!(dict.contains("Guten"));
			assert!(dict.contains("Morgen"));
		}

		fs::remove_file(fname).expect("Could not clean up. Please manually remove test files");
	}
}
