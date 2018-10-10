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
extern crate regex;

use std::rc::Rc;
use std::io::Write;
use std::collections::HashMap;

fn main() {
  let mut source_buffer = String::new();
  let mut target_buffer = String::new();
  let space_quota       = 1024;
  let time_quota        = 1024;
  let mut heap          = eq::heap::Heap::with_capacity(space_quota);
  let mut container     = eq::container::Container::with_heap(heap);
  loop {
    print!("user@eq\nλ ");
    std::io::stdout().flush().unwrap();
    source_buffer.clear();
    target_buffer.clear();
    std::io::stdin().read_line(
      &mut source_buffer).expect("stdin");
    if source_buffer.starts_with(".dump") {
      let dump = container.to_string().expect("dump");
      print!("{}", dump);
    } else {
      let target = container.eval(&source_buffer, time_quota).expect("eval");
      println!("=> {}", &target);
    }
  }
}
