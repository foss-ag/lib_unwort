// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

#![warn(missing_docs)]
#![allow(dead_code)]

//! The unwort library aims to deliver a less frustrating experience with spell
//! checking software, which more often than not acts rather stupidly when
//! encountering a word that obviously exists, but it does not recognise it in
//! a particular variation.
//! For example, a dictionary may know the word `dictionary` but has no idea
//! what `dictionaries` are, even though the construction rules are quite simple.
//! Unwort should favour making a mistake in suspecting a word is correct rather
//! than the other way around as long as it is reasonable to assume the error
//! is easily caught by a native speaker of that language. It should not make
//! guesses where they may overlook it, since such would make it unusable.

#[macro_use]
extern crate serde_derive;
extern crate toml;

pub mod is_dict;
mod dict_config;
pub mod definable;

pub use is_dict::*;
pub use definable::Definable;

pub mod german;
