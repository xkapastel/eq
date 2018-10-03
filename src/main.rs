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

extern crate eq;

use std::io::Write;

fn main() {
  let mut heap = eq::Heap::with_capacity(1024);
  let mut buf = String::new();
  loop {
    print!("user@eq\nλ ");
    std::io::stdout().flush().unwrap();
    buf.clear();
    std::io::stdin().read_line(&mut buf).unwrap();
    let object = heap.parse(&buf).unwrap();
    buf.clear();
    heap.quote(object, &mut buf).unwrap();
    println!("object = {}", &buf);
  }
}
