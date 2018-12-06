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

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

pub mod rt;
pub use self::rt::Pod;

struct Database {

}

impl Database {
  fn new() -> Self {
    Database {

    }
  }
}

use std::sync::Mutex;

lazy_static! {
  static ref DATA: Mutex<Database> = {
    let data = Database::new();
    return Mutex::new(data);
  };
}

#[wasm_bindgen]
pub fn exec(uid: i64, src: &str) -> String {
  return src.to_string();
}
