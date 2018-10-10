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

use super::*;

pub struct Container {
  heap: heap::Heap,
  dictionary: Dictionary,
  insert_pattern: regex::Regex,
  delete_pattern: regex::Regex,
}

impl Container {
  pub fn with_heap(heap: heap::Heap) -> Self {
    let insert_pattern = regex::Regex::new(
      r"^:([a-z+\-*/<>!?][a-z0-9+\-*/<>!?]*)\s+(.*)")
      .expect("insert_pattern");
    let delete_pattern = regex::Regex::new(
      r"^~([a-z+\-*/<>!?][a-z0-9+\-*/<>!?]*)\s*")
      .expect("delete_pattern");
    Container {
      heap: heap,
      dictionary: HashMap::new(),
      insert_pattern: insert_pattern,
      delete_pattern: delete_pattern,
    }
  }
  
  pub fn eval(&mut self, src: &str, time_quota: u64) -> Result<String> {
    let mut dst = String::new();
    if let Some(data) = self.insert_pattern.captures(src) {
      let key: Rc<str> = data.get(1).expect("key").as_str().into();
      let value_src = data.get(2).expect("value").as_str();
      let value = self.heap.parse(value_src)?;
      let value = reduce::reduce(
        value, &mut self.heap, &self.dictionary, time_quota)?;
      self.dictionary.insert(key.clone(), value);
      dst.push(':');
      dst.push_str(&key);
      dst.push(' ');
      self.heap.quote(value, &mut dst)?;
    } else if let Some(data) = self.delete_pattern.captures(src) {
      let key: Rc<str> = data.get(1).expect("key").as_str().into();
      self.dictionary.remove(&key);
      dst.push('~');
      dst.push_str(&key);
    } else {
      let source = self.heap.parse(src)?;
      let target = reduce::reduce(
        source, &mut self.heap, &self.dictionary, time_quota)?;
      self.heap.quote(target, &mut dst)?;
    }
    return Ok(dst);
  }
}
