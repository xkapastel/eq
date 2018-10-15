// This file is a part of Sundial.
// Copyright (C) 2018 Matthew Blount

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public
// License along with this program.  If not, see
// <https://www.gnu.org/licenses/.

extern crate regex;
extern crate comrak;
extern crate atom_syndication;

/// An error that might occur during computation.
#[derive(Debug, Copy, Clone)]
pub enum Error {
  Time,
  Space,
  Tag,
  Stub,
  Bug,
  Null,
  Assert,
  Syntax,
  Underflow,
  Home,
}

/// The result of a computation.
pub type Result<T> = std::result::Result<T, Error>;

pub type Num = f64;

/// A Sundial bitcode.
#[derive(Debug, Copy, Clone)]
pub enum Bit {
  App,
  Box,
  Cat,
  Cpy,
  Drp,
  Swp,
  Fix,
  Run,
  Jmp,
  Num,
  Set,
  All,
  Min,
  Max,
  Add,
  Neg,
  Mul,
  Inv,
  Exp,
  Log,
  Cos,
  Sin,
  Abs,
  Cel,
  Flr,
}

/// Halt the computation if the given condition is false.
pub fn assert(flag: Result<bool>) -> Result<()> {
  match flag {
    Ok(true) => {
      return Ok(());
    }
    Ok(false) => {
      return Err(Error::Assert);
    }
    Err(error) => {
      return Err(error);
    }
  }
}

pub const SYM_PATTERN: &'static str = r"[a-z0-9+\-*/<>=_]+";

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref WORD_REGEX: regex::Regex = {
    regex::Regex::new(SYM_PATTERN).unwrap()
  };
  static ref POD_INSERT_REGEX: regex::Regex = {
    let src = format!(r"^:({})\s+(.*)", SYM_PATTERN);
    regex::Regex::new(&src).unwrap()
  };
  static ref POD_DELETE_REGEX: regex::Regex = {
    let src = format!(r"^~({})\s*", SYM_PATTERN);
    regex::Regex::new(&src).unwrap()
  };
  static ref ANN_REGEX: regex::Regex = {
    let src = format!(r"^\(({})\)$", SYM_PATTERN);
    regex::Regex::new(&src).unwrap()
  };
}

pub mod mem;
pub mod run;
pub mod pod;

pub use self::pod::Pod;
