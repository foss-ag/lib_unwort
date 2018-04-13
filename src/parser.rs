// lib_unwort parser by Alexander Korn

// Licensed under the German Free Software License

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Parser {
	file: String
}

impl Parser {


	fn parse(&self) -> bool {
		// Load the file to be parsed into a String.
		let file = File::open(self.file)?;
		let mut buf_reader = BufReader::new(file);
		let mut contents = String::new();

		buf_reader.read_to_string(&mut contents)?;

		
	}
}