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

use super::*;

pub struct Container {
  heap: heap::Heap,
  dictionary: Dictionary,
  insert_pattern: regex::Regex,
  delete_pattern: regex::Regex,
}

const word_pattern: &'static str = r"[a-z+\-*/<>!?=]+";

impl Container {
  fn with_heap(heap: heap::Heap) -> Self {
    let src = format!(r"^:({})\s+(.*)", word_pattern);
    let insert_pattern = regex::Regex::new(&src).expect("insert");
    let src = format!(r"^~({})\s*", word_pattern);
    let delete_pattern = regex::Regex::new(&src).expect("delete");
    Container {
      heap: heap,
      dictionary: HashMap::new(),
      insert_pattern: insert_pattern,
      delete_pattern: delete_pattern,
    }
  }

  pub fn from_image(
    path: &str,
    space_quota: usize,
    time_quota: u64) -> Result<Self> {
    let contents = std::fs::read_to_string(path).or(Err(Error::Bug))?;
    let code = feed::extract_code_blocks(&contents);
    let heap = heap::Heap::with_capacity(space_quota);
    let mut container = Container::with_heap(heap);
    for line in code.lines() {
      container.eval(line, time_quota)?;
    }
    return Ok(container);
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
    for pointer in self.dictionary.values() {
      self.heap.mark(*pointer)?;
    }
    self.heap.sweep()?;
    return Ok(dst);
  }

  pub fn to_string(&self) -> Result<String> {
    let mut target = String::new();
    let mut keys: Vec<Rc<str>> = self.dictionary.keys()
      .map(|x| x.clone()).collect();
    keys.sort();
    for key in keys.iter() {
      let value = self.dictionary.get(key).unwrap();
      target.push(':');
      target.push_str(&key);
      target.push(' ');
      self.heap.quote(*value, &mut target);
      target.push('\n');
    }
    return Ok(target);
  }
}
