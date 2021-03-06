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
    pub fn parse(&self) -> bool {
        // Load the file to be parsed into a String.
        let file = File::open(self.file)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;

        // Split string at dots
        let mut first = true;
        let contents_split = contents.split(".");

        for part in contents_split {
            // If first part, check if part is correct dictionary file header
            if first {
                match part.as_ref() {
                    "unwortdict" => (),
                    // If header is wrong, break parser execution and return false.
                    _            => return false,
                }
                first = false;
            }

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

                self.dict.push(word);
            } else {
                // Break parser execution because of invalid dictionary file (or just skip this entry)
                unimplemented!();
            }

            return true;
        }
    }

    pub fn dict(&self) -> Vec<Word> {
        self.dict
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
            2 => {
                match c {
                    "s" => word.set_number(Number::Singular),
                    "p" => word.set_number(Number::Plural),
                }
            },
            3 => {
                match c {
                    "m" => word.set_gender(Gender::Masculine),
                    "f" => word.set_gender(Gender::Feminine),
                    "n" => word.set_gender(Gender::Neuter),
                    "i" => word.set_mood(Mood::Indicative),
                    "c" => word.set_mood(Mood::ConjunctiveOne),
                    "j" => word.set_mood(Mood::ConjunctiveTwo),
                }
            },
            4 => {
                match c {
                    "a" => word.set_genera(Genera:Active),
                    "p" => word.set_genera(Genera::Passive),
                }
            },
        }
    }
}