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

//! The unwort library aims to be an a little cleverer dictionary, that tries
//! to take into account the simplerer particularities of certain languages and
//! avoid being a nuisance rather than a bane that you have to fight every step.
//! Realising that it will most certainly never be possible to write the
//! perfect spellchecker, it is a benevolent spellchecker, which should prefer
//! not to mark an error that is one rather than marking an error that is not.

#![warn(missing_docs)]

extern crate rust_stemmers;

pub mod german;
