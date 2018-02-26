// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

use std::path::{Path, PathBuf};
use std::io::{self, Read};
use std::fs::File;
use toml;

/// Configuration file for a dictionary. This aims to be the same for every
/// dictionary, which means some of the functions might seem useless for
/// certain languages, but are needed for others. For instance, in standard
/// English, genders are irrelevant, but interesting for German.
/// In case certain functionalities become uninteresting, they may be removed.
#[derive(Serialize, Deserialize)]
pub struct DictConfig {
	name: String,
	definitions: PathBuf,
	words: PathBuf
}

impl DictConfig {
	/// Open a dictionary file definition at p.
	pub fn open(p: &Path) -> io::Result<DictConfig> {
		let mut file = File::open(p)?;
		let mut content = String::new();
		file.read_to_string(&mut content)?;

		let mut dc: DictConfig = match toml::from_str(&content) {
			Ok(dc) => dc,
			Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("TOML could not be read: {}", e)))
		};

		// File paths hinted at in the main file should be relative to it,
		// not the executing program.
		dc.definitions = match p.parent() {
			Some(parent) => parent.join(&dc.definitions),
			None => dc.definitions
		};
		dc.words = match p.parent() {
			Some(parent) => parent.join(&dc.definitions),
			None => dc.words
		};

		Ok(dc)
	}

	/// The name of the Dictionary by which it can be identified.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// The path of the file containing the definitions of words, relative to
	/// the configuration file.
	pub fn definitions(&self) -> &Path {
		&self.definitions
	}

	/// The path of the file containing all known words, from which others may
	/// be derived.
	pub fn words(&self) -> &Path {
		&self.words
	}
}
