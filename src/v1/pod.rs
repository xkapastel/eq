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

use std::rc::Rc;
use std::collections::HashMap;

fn iter_nodes<'a, F>(
  node: &'a comrak::nodes::AstNode<'a>,
  func: &mut F) where F: FnMut(&'a comrak::nodes::AstNode<'a>) {
  func(node);
  for child in node.children() {
    iter_nodes(child, func);
  }
}

fn extract_code_blocks(src: &str) -> String {
  let arena = comrak::Arena::new();
  let options = comrak::ComrakOptions::default();
  let root = comrak::parse_document(&arena, src, &options);
  let mut blocks: Vec<String> = Vec::new();
  iter_nodes(root, &mut |node| {
    if let &comrak::nodes::NodeValue::CodeBlock(ref node) = &node.data.borrow().value {
      if "eq" == std::str::from_utf8(&node.info).unwrap() {
        let block = std::str::from_utf8(&node.literal).unwrap().to_string();
        blocks.push(block);
      }
    }
  });
  return blocks.join("\n");
}

pub struct Pod {
  mem: mem::Mem,
  tab: mem::Tab,
}

impl Pod {
  fn with_mem(mem: mem::Mem) -> Self {
    Pod {
      mem: mem,
      tab: HashMap::new(),
    }
  }

  pub fn from_string(
    src: &str,
    space_quota: usize,
    time_quota: u64) -> Result<Self> {
    let code = extract_code_blocks(src);
    let mem = mem::Mem::with_capacity(space_quota);
    let mut pod = Pod::with_mem(mem);
    for line in code.lines() {
      pod.eval(line, time_quota)?;
    }
    return Ok(pod);
  }

  pub fn default(space_quota: usize, time_quota: u64) -> Result<Self> {
    let home = std::env::var("SUNDIAL_HOME").or(Err(Error::Home))?;
    let path: std::path::PathBuf = [&home, "pod", "default.md"].iter().collect();
    let src = std::fs::read_to_string(path).or(Err(Error::Home))?;
    return Pod::from_string(&src, space_quota, time_quota);
  }

  pub fn eval(&mut self, src: &str, time_quota: u64) -> Result<String> {
    let mut dst = String::new();
    if let Some(data) = POD_INSERT_REGEX.captures(src) {
      let key: Rc<str> = data.get(1).expect("key").as_str().into();
      let value_src = data.get(2).expect("value").as_str();
      let value = self.mem.parse(value_src)?;
      let value = run::reduce(
        value, &mut self.mem, &self.tab, time_quota)?;
      self.tab.insert(key.clone(), value);
      dst.push(':');
      dst.push_str(&key);
      dst.push(' ');
      self.mem.quote(value, &mut dst)?;
    } else if let Some(data) = POD_DELETE_REGEX.captures(src) {
      let key: Rc<str> = data.get(1).expect("key").as_str().into();
      self.tab.remove(&key);
      dst.push('~');
      dst.push_str(&key);
    } else {
      let source = self.mem.parse(src)?;
      let target = run::reduce(
        source, &mut self.mem, &self.tab, time_quota)?;
      self.mem.quote(target, &mut dst)?;
    }
    for pointer in self.tab.values() {
      self.mem.mark(*pointer)?;
    }
    self.mem.sweep()?;
    return Ok(dst);
  }

  pub fn to_string(&self) -> Result<String> {
    let mut target = String::new();
    let mut keys: Vec<Rc<str>> = self.tab.keys()
      .map(|x| x.clone()).collect();
    keys.sort();
    for key in keys.iter() {
      let value = self.tab.get(key).unwrap();
      target.push(':');
      target.push_str(&key);
      target.push(' ');
      self.mem.quote(*value, &mut target)?;
      target.push('\n');
    }
    return Ok(target);
  }
}

#[test]
fn primitives() {
  let space   = 1024;
  let time    = 1024;
  let mut pod = Pod::from_string("", space, time).unwrap();
  let mut check = |source, expected| {
    println!("{} => {}", source, expected);
    let target = pod.eval(source, time).unwrap();
    assert_eq!(expected, &target);
  };
  check("", "");
  check("[A]", "[A]");
  check("[[A]]", "[[A]]");
  check("[A] [B]", "[A] [B]");
  check("%app", "%app");
  check("%box", "%box");
  check("%cat", "%cat");
  check("%copy", "%copy");
  check("%drop", "%drop");
  check("%swap", "%swap");
  check("%fix", "%fix");
  check("[A] %app", "A");
  check("[A] %box", "[[A]]");
  check("[A] [B] %cat", "[A B]");
  check("[A] %cat", "[A] %cat");
  check("[A] %copy", "[A] [A]");
  check("[A] %drop", "");
  check("[A] [B] %swap", "[B] [A]");
  check("[A] %swap", "[A] %swap");
  check("[A] %fix", "[[A] %fix A]");
  check("[A] [B] %box %cat", "[A [B]]");
}
