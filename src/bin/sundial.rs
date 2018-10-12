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

extern crate sundial;

use std::rc::Rc;
use std::io::Write;
use std::collections::HashMap;

fn main() {
  let mut source_buffer = String::new();
  let mut target_buffer = String::new();
  let space_quota       = 1024;
  let time_quota        = 1024;
  let pod_path          = std::env::var("SUNDIAL_POD").unwrap();
  let pod_src           = std::fs::read_to_string(&pod_path).unwrap();
  let mut pod           = sundial::pod::Pod::from_string(
    &pod_src, space_quota, time_quota).unwrap();
  loop {
    print!("user@sundial\nλ ");
    std::io::stdout().flush().unwrap();
    source_buffer.clear();
    target_buffer.clear();
    std::io::stdin().read_line(&mut source_buffer).unwrap();
    let target = pod.eval(&source_buffer, time_quota).unwrap();
    println!("=> {}", &target);
  }
}
