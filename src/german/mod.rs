// Copyright (C) 2018 Arne Dußin.
//
// This Program may be used by anyone in accordance with the terms of the
// German Free Software License
//
// The License may be obtained under http://www.d-fsl.org.

//! German language module for advanced German text checking.

pub mod compound;
pub mod dictionary;
pub mod word;

pub use self::compound::*;
pub use self::dictionary::*;
pub use self::word::*;

