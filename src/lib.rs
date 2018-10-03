// This file is a part of Eq.
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
}

pub type Result<T> = std::result::Result<T, Error>;

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

mod heap;
pub use self::heap::Pointer;
pub use self::heap::Heap;

mod eval;
