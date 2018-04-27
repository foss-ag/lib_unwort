// lib_unwort parser by Alexander Korn

// Licensed under the German Free Software License

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use german::word::Word;

pub struct Parser {
    file: String,
    dict: Vec<Word>
}

impl Parser {
    fn parse(&self) -> bool {
        // Load the file to be parsed into a String.
        let file = File::open(self.file)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;

        // Split string at dots
        let mut first = true;
        let mut correct_header = false;
        let contents_split = contents.split(".");

        for part in contents_split {
            // If first part, check if part is correct dictionary file header
            if first {
                correct_header = match part.as_ref() {
                    "unwortdict" => true,
                    _            => false,
                }
                first = false;
            }
            if correct_header {
                let part_split = part.split(",");

                if part_split.len() == 2 {
                    // TODO: take first split part as word

                    let mut word = Word(&part_split[0]);
                    let chars = part_split[1].chars();
                    let mut pos: u8 = 0;

                    loop {
                        match chars.next() {
                            Some(c) => {
                                validate_option(&mut word, &c, &pos);
                                pos += 1;
                            },
                            None    => break,
                        }
                    }
                } else {
                    // Break program execution because of invalid dictionary file (or just skip this entry)
                    unimplemented!();
                }
            } else {
                // Break program execution because of invalid dictionary file
                unimplemented!();
            }
        }
    }

    fn validate_option(word: &mut Word, c: &char, pos: &u8) {
        match pos {
            0 => {
                match c {
                    "n" => word.set_word_type(WordType::Noun),
                    "v" => word.set_word_type(WordType::Verb),
                    "a" => word.set_word_type(WordType::Adjective),
                }
            },
            1 => {
                match c {
                    "n" => word.set_case(Case::Nominative),
                    "g" => word.set_case(Case::Genitive),
                    "d" => word.set_case(Case::Dative),
                    "a" => word.set_case(Case::Accusative),
                    "f" => word.set_person(Person::First),
                    "s" => word.set_person(Person::Second),
                    "t" => word.set_person(Person::Third),
                }
            },
        }
    }
}