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

/// A pointer to some object.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Ptr {
  index: usize,
  generation: u64,
}

use std::collections::HashMap;

pub type Tab = HashMap<Rc<str>, mem::Ptr>;

enum Obj {
  Nil,
  Sym(Rc<str>),
  Ann(Rc<str>),
  Bit(Bit),
  Fun(Ptr),
  Cmd(Ptr),
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

  fn is_cmd(&self) -> bool {
    match self {
      Obj::Cmd(_) => true,
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

  /// Creates a new cmdow.
  pub fn new_cmd(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Cmd(body);
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

  /// Predicates cmdows.
  pub fn is_cmd(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_cmd());
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

  /// Get the body of an cmdow.
  pub fn get_cmd_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Cmd(ref body) => {
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
          &Obj::Cmd(body) => {
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
    let mut brackets = Vec::new();
    let src = src.replace("{", "{ ");
    let src = src.replace("}", " }");
    let src = src.replace("[", "[ ");
    let src = src.replace("]", " ]");
    for word in src.split_whitespace() {
      match word {
        "{" => {
          brackets.push('{');
          stack.push(build);
          build = Vec::new();
        }
        "}" => {
          let current_bracket = brackets.pop().ok_or(Error::Syntax)?;
          if current_bracket != '{' {
            return Err(Error::Syntax);
          }
          let prev = stack.pop().ok_or(Error::Syntax)?;
          let mut xs = self.new_nil()?;
          for object in build.iter().rev() {
            xs = self.new_cat(*object, xs)?;
          }
          xs = self.new_cmd(xs)?;
          build = prev;
          build.push(xs);
        }
        "[" => {
          brackets.push('[');
          stack.push(build);
          build = Vec::new();
        }
        "]" => {
          let current_bracket = brackets.pop().ok_or(Error::Syntax)?;
          if current_bracket != '[' {
            return Err(Error::Syntax);
          }
          let prev = stack.pop().ok_or(Error::Syntax)?;
          let mut xs = self.new_nil()?;
          for object in build.iter().rev() {
            xs = self.new_cat(*object, xs)?;
          }
          xs = self.new_fun(xs)?;
          build = prev;
          build.push(xs);
        }
        "%app" => {
          let bit = Bit::App;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%box" => {
          let bit = Bit::Box;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%cat" => {
          let bit = Bit::Cat;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%cpy" => {
          let bit = Bit::Cpy;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%drp" => {
          let bit = Bit::Drp;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%swp" => {
          let bit = Bit::Swp;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%fix" => {
          let bit = Bit::Fix;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%run" => {
          let bit = Bit::Run;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        "%jmp" => {
          let bit = Bit::Jmp;
          let object = self.new_bit(bit)?;
          build.push(object);
        }
        _ => {
          if word.starts_with("%") {
            return Err(Error::Syntax);
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
            buf.push_str("%app");
          }
          Bit::Box => {
            buf.push_str("%box");
          }
          Bit::Cat => {
            buf.push_str("%cat");
          }
          Bit::Cpy => {
            buf.push_str("%cpy");
          }
          Bit::Drp => {
            buf.push_str("%drp");
          }
          Bit::Swp => {
            buf.push_str("%swp");
          }
          Bit::Fix => {
            buf.push_str("%fix");
          }
          Bit::Run => {
            buf.push_str("%run");
          }
          Bit::Jmp => {
            buf.push_str("%jmp");
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
      &Obj::Cmd(body) => {
        buf.push_str("{ ");
        self.quote(body, buf)?;
        buf.push_str(" }");
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
