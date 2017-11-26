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
