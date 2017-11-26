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

//! Helper functions for parsing text. These are designed to be used purely
//! internally. Anything that is not strictly a function designed for one very
//! basic string parsing purpose does not belong here.

pub fn has_punctuation(s: &str) -> bool {
	for c in s.chars() {
		if c == '.' {
			return true;
		}
		if c == ',' {
			return true;
		}
	}

	false
}

pub fn has_whitespace(s: &str) -> bool {
	s.contains(" ")
}
