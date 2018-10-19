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
  Copy,
  Drop,
  Swap,
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

pub const SYM_PATTERN: &'static str = r"[a-z0-9-]+";

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

/// A pointer to some object.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ptr {
  index: usize,
  generation: u64,
}

use std::rc::Rc;
use std::collections::HashMap;

pub type Tab = HashMap<Rc<str>, Ptr>;

enum Obj {
  Nil,
  Sym(Rc<str>),
  Ann(Rc<str>),
  Bit(Bit),
  Fun(Ptr),
  Cat(Ptr, Ptr),
}

struct Node {
  object: Obj,
  generation: u64,
  is_visible: bool,
}

/// A garbage-collected heap.
pub struct Mem {
  nodes: Vec<Option<Node>>,
  generation: u64,
}

impl Ptr {
  fn new(index: usize, generation: u64) -> Self {
    Ptr {
      index: index,
      generation: generation,
    }
  }
}

impl Obj {
  fn is_nil(&self) -> bool {
    match self {
      Obj::Nil => true,
      _ => false,
    }
  }

  fn is_bit(&self) -> bool {
    match self {
      Obj::Bit(_) => true,
      _ => false,
    }
  }

  fn is_sym(&self) -> bool {
    match self {
      Obj::Sym(_) => true,
      _ => false,
    }
  }

  fn is_ann(&self) -> bool {
    match self {
      Obj::Ann(_) => true,
      _ => false,
    }
  }

  fn is_fun(&self) -> bool {
    match self {
      Obj::Fun(_) => true,
      _ => false,
    }
  }

  fn is_cat(&self) -> bool {
    match self {
      Obj::Cat(_, _) => true,
      _ => false,
    }
  }
}

impl Node {
  fn new(object: Obj, generation: u64) -> Self {
    Node {
      object: object,
      generation: generation,
      is_visible: false,
    }
  }
}

impl Mem {
  /// Creates a heap with the given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    let mut nodes = Vec::with_capacity(capacity);
    for _ in 0..capacity {
      nodes.push(None);
    }
    Mem {
      nodes: nodes,
      generation: 0,
    }
  }
  
  /// Returns the nil object
  pub fn new_nil(&mut self) -> Result<Ptr> {
    let object = Obj::Nil;
    return self.put(object);
  }

  pub fn new_bit(&mut self, bit: Bit) -> Result<Ptr> {
    let object = Obj::Bit(bit);
    return self.put(object);
  }

  /// Creates a new symbol.
  pub fn new_sym(&mut self, value: Rc<str>) -> Result<Ptr> {
    let object = Obj::Sym(value);
    return self.put(object);
  }

  /// Creates a new annotation.
  pub fn new_ann(&mut self, value: Rc<str>) -> Result<Ptr> {
    let object = Obj::Ann(value);
    return self.put(object);
  }

  /// Creates a new funtraction.
  pub fn new_fun(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Fun(body);
    return self.put(object);
  }

  /// Creates a new catenation.
  pub fn new_cat(&mut self, fst: Ptr, snd: Ptr) -> Result<Ptr> {
    if self.is_nil(fst)? {
      return Ok(snd);
    }
    if self.is_cat(fst)? {
      let fst_fst = self.get_cat_fst(fst)?;
      let fst_snd = self.get_cat_snd(fst)?;
      let rhs = self.new_cat(fst_snd, snd)?;
      return self.new_cat(fst_fst, rhs);
    }
    let object = Obj::Cat(fst, snd);
    return self.put(object);
  }

  /// Predicates the nil object.
  pub fn is_nil(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_nil());
  }

  /// Predicates bitcodes.
  pub fn is_bit(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_bit());
  }

  /// Predicates symbols.
  pub fn is_sym(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_sym());
  }

  /// Predicates annotations.
  pub fn is_ann(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_ann());
  }

  /// Predicates funtractions.
  pub fn is_fun(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_fun());
  }

  /// Predicates catenations.
  pub fn is_cat(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_cat());
  }

  pub fn get_bit(&self, pointer: Ptr) -> Result<Bit> {
    match self.get_ref(pointer)? {
      &Obj::Bit(ref value) => {
        return Ok(*value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the value of a symbol.
  pub fn get_sym(&self, pointer: Ptr) -> Result<Rc<str>> {
    match self.get_ref(pointer)? {
      &Obj::Sym(ref value) => {
        return Ok(value.clone());
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the value of an annotation.
  pub fn get_ann(&self, pointer: Ptr) -> Result<Rc<str>> {
    match self.get_ref(pointer)? {
      &Obj::Ann(ref value) => {
        return Ok(value.clone());
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the body of an funtraction.
  pub fn get_fun_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Fun(ref body) => {
        return Ok(*body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the first element of a catenation.
  pub fn get_cat_fst(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Cat(ref fst, _) => {
        return Ok(*fst);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the second element of a cat.
  pub fn get_cat_snd(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Cat(_, ref snd) => {
        return Ok(*snd);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn mark(&mut self, root: Ptr) -> Result<()> {
    match &mut self.nodes[root.index] {
      &mut Some(ref mut node) => {
        if node.generation != root.generation {
          return Err(Error::Null);
        }
        node.is_visible = true;
        match &node.object {
          &Obj::Fun(body) => {
            return self.mark(body);
          }
          &Obj::Cat(fst, snd) => {
            self.mark(fst)?;
            return self.mark(snd);
          }
          _ => {
            return Ok(());
          }
        }
      }
      _ => {
        return Err(Error::Null);
      }
    }
  }

  pub fn sweep(&mut self) -> Result<()> {
    let mut nodes_deleted = 0;
    for maybe_node in self.nodes.iter_mut() {
      let should_delete_node;
      if let Some(ref mut node) = maybe_node {
        if node.is_visible {
          node.is_visible = false;
          should_delete_node = false;
        } else {
          should_delete_node = true;
        }
      } else {
        should_delete_node = false;
      }
      if should_delete_node {
        *maybe_node = None;
        nodes_deleted += 1;
      }
    }
    self.generation += 1;
    println!(
      "[gc] deleted: {} generation: {}", nodes_deleted, self.generation);
    return Ok(());
  }

  pub fn parse(&mut self, src: &str) -> Result<Ptr> {
    let mut build = Vec::new();
    let mut stack = Vec::new();
    let src = src.replace("[", "[ ");
    let src = src.replace("]", " ]");
    for word in src.split_whitespace() {
      match word {
        "[" => {
          stack.push(build);
          build = Vec::new();
        }
        "]" => {
          let prev = stack.pop().ok_or(Error::Syntax)?;
          let mut xs = self.new_nil()?;
          for object in build.iter().rev() {
            xs = self.new_cat(*object, xs)?;
          }
          xs = self.new_fun(xs)?;
          build = prev;
          build.push(xs);
        }
        "a" => {
          let bit = Bit::App;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "b" => {
          let bit = Bit::Box;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "c" => {
          let bit = Bit::Cat;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "d" => {
          let bit = Bit::Copy;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "e" => {
          let bit = Bit::Drop;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "f" => {
          let bit = Bit::Swap;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        _ => {
          if word.len() == 1 {
            if word.chars().all(|x| x.is_lowercase()) {
              return Err(Error::Syntax);
            }
          }
          if let Some(data) = ANN_REGEX.captures(&word) {
            let name = data.get(1).ok_or(Error::Bug)?.as_str();
            let object = self.new_ann(name.into())?;
            build.push(object);
          } else {
            let object = self.new_sym(word.into())?;
            build.push(object);
          }
        }
      }
    }
    if !stack.is_empty() {
      return Err(Error::Syntax);
    }
    let mut xs = self.new_nil()?;
    for object in build.iter().rev() {
      xs = self.new_cat(*object, xs)?;
    }
    return Ok(xs);
  }

  pub fn quote(&self, root: Ptr, buf: &mut String) -> Result<()> {
    match self.get_ref(root)? {
      &Obj::Nil => {
        //
      }
      &Obj::Bit(ref value) => {
        match value {
          Bit::App => {
            buf.push('a');
          }
          Bit::Box => {
            buf.push('b');
          }
          Bit::Cat => {
            buf.push('c');
          }
          Bit::Copy => {
            buf.push('d');
          }
          Bit::Drop => {
            buf.push('e');
          }
          Bit::Swap => {
            buf.push('f');
          }
        }
      }
      &Obj::Sym(ref value) => {
        buf.push_str(&value);
      }
      &Obj::Ann(ref value) => {
        buf.push('(');
        buf.push_str(&value);
        buf.push(')');
      }
      &Obj::Fun(body) => {
        buf.push('[');
        self.quote(body, buf)?;
        buf.push(']');
      }
      &Obj::Cat(fst, snd) => {
        self.quote(fst, buf)?;
        if !self.is_nil(snd)? {
          buf.push(' ');
          self.quote(snd, buf)?;
        }
      }
    }
    return Ok(());
  }

  fn put(&mut self, object: Obj) -> Result<Ptr> {
    for (index, maybe_node) in self.nodes.iter_mut().enumerate() {
      if maybe_node.is_some() {
        continue;
      }
      let node = Node::new(object, self.generation);
      let pointer = Ptr::new(index, self.generation);
      *maybe_node = Some(node);
      return Ok(pointer);
    }
    return Err(Error::Space);
  }

  fn get_ref(&self, pointer: Ptr) -> Result<&Obj> {
    match &self.nodes[pointer.index] {
      &Some(ref node) => {
        if node.generation == pointer.generation {
          return Ok(&node.object);
        }
        return Err(Error::Null);
      }
      None => {
        return Err(Error::Null);
      }
    }
  }
}

pub fn reduce(
  continuation: Ptr,
  mem: &mut Mem,
  tab: &Tab,
  mut time_quota: u64) -> Result<Ptr> {
  let mut thread = Thread::with_continuation(continuation);
  while time_quota > 0 && thread.has_continuation() {
    time_quota -= 1;
    thread.step(mem, tab)?;
  }
  if thread.has_continuation() {
    let snd = thread.get_continuation(mem)?;
    let fst = thread.get_environment(mem)?;
    return mem.new_cat(fst, snd);
  }
  return thread.get_environment(mem);
}

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Frame {
  con: VecDeque<Ptr>,
  env: Vec<Ptr>,
  err: Vec<Ptr>,
}

impl Frame {
  fn new(root: Ptr) -> Self {
    let mut con = VecDeque::new();
    con.push_back(root);
    Frame {
      con: con,
      env: vec![],
      err: vec![],
    }
  }
}

pub struct Thread {
  frame: Frame,
}

impl Thread {
  pub fn with_continuation(continuation: Ptr) -> Self {
    Thread {
      frame: Frame::new(continuation),
    }
  }

  pub fn has_continuation(&self) -> bool {
    return !self.frame.con.is_empty();
  }

  pub fn get_continuation(
    &mut self, mem: &mut Mem) -> Result<Ptr> {
    let mut xs = mem.new_nil()?;
    for object in self.frame.con.iter() {
      xs = mem.new_cat(*object, xs)?;
    }
    self.frame.con.clear();
    return Ok(xs);
  }

  pub fn push_continuation_front(&mut self, data: Ptr) {
    self.frame.con.push_front(data);
  }

  pub fn push_continuation_back(&mut self, data: Ptr) {
    self.frame.con.push_back(data);
  }

  pub fn pop_continuation(
    &mut self, mem: &mut Mem) -> Result<Ptr> {
    loop {
      let code = self.frame.con.pop_front().ok_or(Error::Bug)?;
      if mem.is_cat(code)? {
        let fst = mem.get_cat_fst(code)?;
        let snd = mem.get_cat_snd(code)?;
        self.frame.con.push_front(snd);
        self.frame.con.push_front(fst);
      } else {
        return Ok(code);
      }
    }
  }

  pub fn is_monadic(&self) -> bool {
    return self.frame.env.len() >= 1;
  }

  pub fn is_dyadic(&self) -> bool {
    return self.frame.env.len() >= 2;
  }

  pub fn get_environment(
    &mut self, mem: &mut Mem) -> Result<Ptr> {
    let mut xs = mem.new_nil()?;
    for object in self.frame.env.iter().rev() {
      xs = mem.new_cat(*object, xs)?;
    }
    for object in self.frame.err.iter().rev() {
      xs = mem.new_cat(*object, xs)?;
    }
    self.frame.env.clear();
    self.frame.err.clear();
    return Ok(xs);
  }

  pub fn push_environment(&mut self, data: Ptr) {
    self.frame.env.push(data);
  }

  pub fn pop_environment(&mut self) -> Result<Ptr> {
    return self.frame.env.pop().ok_or(Error::Underflow);
  }

  pub fn peek_environment(&mut self) -> Result<Ptr> {
    return self.frame.env.last().map(|x| *x).ok_or(Error::Underflow);
  }

  pub fn thunk(&mut self, root: Ptr) {
    self.frame.err.append(&mut self.frame.env);
    self.frame.err.push(root);
  }

  pub fn step(
    &mut self,
    mem: &mut Mem,
    tab: &HashMap<Rc<str>, Ptr>) -> Result<()> {
    let code = self.pop_continuation(mem)?;
    if mem.is_fun(code)? {
      self.push_environment(code);
    } else if mem.is_bit(code)? {
      match mem.get_bit(code)? {
        Bit::App => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = mem.get_fun_body(source)?;
          self.push_continuation_front(target);
        }
        Bit::Box => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = mem.new_fun(source)?;
          self.push_environment(target);
        }
        Bit::Cat => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let rhs = self.pop_environment()?;
          let lhs = self.pop_environment()?;
          let rhs_body = mem.get_fun_body(rhs)?;
          let lhs_body = mem.get_fun_body(lhs)?;
          let target_body = mem.new_cat(lhs_body, rhs_body)?;
          let target = mem.new_fun(target_body)?;
          self.push_environment(target);
        }
        Bit::Copy => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.peek_environment()?;
          self.push_environment(source);
        }
        Bit::Drop => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          self.pop_environment()?;
        }
        Bit::Swap => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let fst = self.pop_environment()?;
          let snd = self.pop_environment()?;
          self.push_environment(fst);
          self.push_environment(snd);
        }
      }
    } else if mem.is_sym(code)? {
      let code_value = mem.get_sym(code)?;
      match tab.get(&code_value) {
        Some(binding) => {
          self.push_continuation_front(*binding);
        }
        None => {
          self.thunk(code);
        }
      }
      return Ok(());
    } else if mem.is_nil(code)? || mem.is_ann(code)? {
      return Ok(());
    } else {
      return Err(Error::Bug);
    }
    return Ok(());
  }
}

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
  mem: Mem,
  tab: Tab,
}

impl Pod {
  fn with_mem(mem: Mem) -> Self {
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
    let mem = Mem::with_capacity(space_quota);
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
      let value = reduce(
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
      let target = reduce(
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
  check("a", "a");
  check("b", "b");
  check("c", "c");
  check("d", "d");
  check("e", "e");
  check("f", "f");
  check("[A] a", "A");
  check("[A] b", "[[A]]");
  check("[A] [B] c", "[A B]");
  check("[A] c", "[A] c");
  check("[A] d", "[A] [A]");
  check("[A] e", "");
  check("[A] [B] f", "[B] [A]");
  check("[A] f", "[A] f");
  check("[A] [B] b c", "[A [B]]");
}
