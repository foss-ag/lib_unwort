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

//! Represents the abstract idea of a word, granting some features that at
//! least most languages will surely benefit from. Does not aim to be complete
//! and languages are encouraged to implement their own version of words.

use std::str::FromStr;
use std::ops::Deref;

/// A words type can be represented using this, granted it is known
#[allow(missing_docs)]
pub enum WordType {
	Noun,
	Adjective,
	Adverb,
	Verb
}

/// The Word trait. Languages that use words (so all of them) will probably
/// want to use this to access and be accessed by other parts.
pub trait Word: FromStr<Err=()> + Deref<Target=String> {
	/// The type of word we are dealing with. May return None, since a words
	/// type can't always be known.
	fn typ(&self) -> Option<WordType>;
}
