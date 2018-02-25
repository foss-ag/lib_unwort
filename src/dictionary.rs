// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::io;
use std::path::Path;

use word::Word;

pub trait Dictionary {
	/// Load a dictionary file from the path specified.
	fn from_file(path: &Path) -> io::Result<Self> where Self: Sized;
	/// Save the dictionary into a dictionary file. If the file already exists
	/// it will be overwritten.
	fn to_file(&self, path: &Path) -> io::Result<()>;

	/// Check if the dictionary contains the word. If the word is not directly
	/// contained, the language specific algorithm tries to find one or
	/// possibly more words, depending on the language to see if this word is
	/// likely to exist.
	fn contains(&self, word: &str) -> bool;

	/// Get the word information for the word with that string representation.
	/// Does not work, if the word is not *explicitly* contained. In that case,
	/// `contains` needs to be used.
	fn get(&self, word: &str) -> Option<&Word>;

	/// Like `get()`, but the word returned may be edited.
	fn get_mut(&mut self, word: &str) -> Option<&mut Word>;

	/// Add the word to the dictionary. Works if the word is not yet explicitly
	/// contained.
	/// Returns true, if the word was entered, false otherwise.
	fn add(&self, word: &Word) -> bool;
}
