// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::ops::Deref;
use german::Word;
const FUGEN: &[&str] = &["", "e", "s", "es", "n", "en", "er", "ens"];
const TILGFUGEN: &[&str] = &["e"];

/// Removes the words from the front of the word string where it's possible,
/// returning the word how it looks without the word in whatever form may fit.
fn remove_starting_words(word: &str, dict: &Vec<Word>) -> Vec<String> {
	let mut candidates: Vec<String> = Vec::new();
	for d in dict {
		if word.starts_with(d.deref()) {
			for f in FUGEN {
				let mut d = d.to_string();
				d.push_str(f);
				if word.starts_with(&d) {
					candidates.push(word.to_string());
				}
			}
		}

		for tf in TILGFUGEN {
			if d.ends_with(tf) {
				let mut d = d.to_string();
				let d_len = d.len();
				d.truncate(d_len - tf.len());

				if word.starts_with(&d) {
					candidates.push(word.to_string());
				}
			}
		}
	}

	candidates
}

/// Check if it is possible to create the word from the available dictionary.
pub fn is_possible(word: &str, dictionary: &Vec<Word>) -> bool {
	let mut candidates = remove_starting_words(word, dictionary);
	let mut i = 0;

	while i < candidates.len() {
		let mut new = remove_starting_words(&candidates[i], dictionary);
		candidates.append(&mut new);

		if candidates[i].is_empty() {
			return true;
		}
		
		i += 1;
	}

	false
}

#[cfg(test)]
mod test {
	use super::*;
	use german::Dictionary;

	#[test]
	fn find_starting_word() {
		// Create a test dictionary
		let mut dict = Dictionary::from_config("test_data/compound_dict.toml");

		
	}
}
