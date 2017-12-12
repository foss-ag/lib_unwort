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

//! Base for a Smart dictionary of any language. Language specific
//! functionalities are not covered.

/// The base trait for all dictionaries.
pub trait Dictionary {
	/// Add the word to the dictionary. Returns true if it is a new word, false
	/// otherwise.
	fn add<W: AsRef<str>>(&mut self, word: W) -> bool;
	
	/// Check if the Dictionary contains the word.
	fn contains<W: AsRef<str>>(&self, word: W) -> bool;
}
