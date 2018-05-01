// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

//! Module for word operations in the German language.

pub mod noun;
pub use self::noun::*;

pub mod verb;
pub use self::verb::*;

use std::ops::Deref;

use definable::Definable;

/// Representation of a German grammatical word class/type.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Class {
	Noun,
	Verb,
	Adjective
}

/// Representation of a German grammatical gender.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Gender {
	Masculine,
	Feminine,
	Neuter
}

/// Representation of a German grammatical number.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Number {
	Singular,
	Plural
}

// Representation of a German grammatical person.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Person {
	First,
	Second,
	Third
}

/// Representation of a German grammatical mood.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum Mood {
	Indicative,
	ConjunctiveOne,
	ConjunctiveTwo
}

pub type Definition = String;

pub trait Word<Def=Definition>: Definable {
	/// The word class aka. type this word is known as.
	fn class(&self) -> Class;

	/// Get the raw string representation of the word in its current form.
	fn raw(&self) -> &str;
}

impl Deref for Word<Def=Definition> {
	type Target = str;

fn deref(&self) -> &str {
		self.raw()
	}
}
