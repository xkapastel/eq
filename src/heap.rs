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

use std::rc::Rc;

/// A pointer to some Eq object.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pointer {
  index: usize,
  generation: u64,
}

pub type Number = f64;

enum Object {
  Nil,
  Number(Number),
  Symbol(Rc<str>),
  Wrap(Pointer),
  Pair(Pointer, Pointer),
}

struct Node {
  object: Object,
  generation: u64,
  is_visible: bool,
}

/// A garbage-collected heap of Eq objects.
pub struct Heap {
  nodes: Vec<Option<Node>>,
  generation: u64,
}

impl Pointer {
  fn nil() -> Self {
    Pointer {
      index: 0,
      generation: 0,
    }
  }

  fn new(index: usize, generation: u64) -> Self {
    Pointer {
      index: index,
      generation: generation,
    }
  }
}

impl Node {
  fn new(object: Object, generation: u64) -> Self {
    Node {
      object: object,
      generation: generation,
      is_visible: false,
    }
  }
}

impl Heap {
  /// Creates a heap with the given capacity.
  pub fn with_capacity(capacity: usize) -> Self {
    let mut nodes = Vec::with_capacity(capacity);
    for i in 0..capacity {
      nodes.push(None);
    }
    Heap {
      nodes: nodes,
      generation: 0,
    }
  }
  
  /// Returns the nil object
  pub fn nil(&mut self) -> Result<Pointer> {
    let object = Object::Nil;
    return self.put(object);
  }

  /// Creates a new number.
  pub fn new_number(&mut self, value: Number) -> Result<Pointer> {
    let object = Object::Number(value);
    return self.put(object);
  }

  /// Creates a new symbol.
  pub fn new_symbol(&mut self, value: Rc<str>) -> Result<Pointer> {
    let object = Object::Symbol(value);
    return self.put(object);
  }

  /// Wraps the given object.
  pub fn new_wrap(&mut self, body: Pointer) -> Result<Pointer> {
    let object = Object::Wrap(body);
    return self.put(object);
  }

  /// Creates a new pair.
  pub fn new_pair(
    &mut self,
    fst: Pointer,
    snd: Pointer) -> Result<Pointer> {
    let object = Object::Pair(fst, snd);
    return self.put(object);
  }

  /// Predicates the nil object.
  pub fn is_nil(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Nil => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Predicates numbers.
  pub fn is_number(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Number(_) => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Predicates symbols.
  pub fn is_symbol(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Symbol(_) => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  pub fn is_app(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "app");
  }

  pub fn is_bind(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "bind");
  }

  pub fn is_copy(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "copy");
  }

  pub fn is_drop(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "drop");
  }

  pub fn is_shift(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "shift");
  }

  pub fn is_reset(&self, pointer: Pointer) -> Result<bool> {
    return self.symbol_eq(pointer, "reset");
  }

  fn symbol_eq(&self, pointer: Pointer, rhs: &str) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Symbol(ref lhs) => {
        return Ok(lhs.as_ref() == rhs);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Predicates wraps.
  pub fn is_wrap(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Wrap(_) => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Predicates pairs.
  pub fn is_pair(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Pair(_, _) => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Get the value of a number.
  pub fn get_number(&self, pointer: Pointer) -> Result<Number> {
    match self.get_ref(pointer)? {
      &Object::Number(value) => {
        return Ok(value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the value of a symbol.
  pub fn get_symbol(&self, pointer: Pointer) -> Result<Rc<str>> {
    match self.get_ref(pointer)? {
      &Object::Symbol(ref value) => {
        return Ok(value.clone());
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the body of a wrap.
  pub fn get_wrap_body(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Wrap(ref body) => {
        return Ok(*body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the first element of a pair.
  pub fn get_pair_fst(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Pair(ref fst, _) => {
        return Ok(*fst);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the second element of a pair.
  pub fn get_pair_snd(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Pair(_, ref snd) => {
        return Ok(*snd);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Deletes all objects unreachable from the given root.
  pub fn flush(&mut self, root: Pointer) -> Result<()> {
    self.visit(root)?;
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
      }
    }
    self.generation += 1;
    return Ok(());
  }

  pub fn parse(&mut self, raw: &str) -> Result<Pointer> {
    let mut build = Vec::new();
    let mut stack = Vec::new();
    let src = raw
      .replace("[", "[ ")
      .replace("]", " ]");
    for word in src.split_whitespace() {
      match word {
        "[" => {
          stack.push(build);
          build = Vec::new();
        }
        "]" => {
          let prev = stack.pop().ok_or(Error::Syntax)?;
          let mut xs = self.nil()?;
          for object in build.iter().rev() {
            xs = self.new_pair(*object, xs)?;
          }
          xs = self.new_wrap(xs)?;
          build = prev;
          build.push(xs);
        }
        _ => {
          let object = self.new_symbol(word.into())?;
          build.push(object);
        }
      }
    }
    if stack.len() > 0 {
      return Err(Error::Syntax);
    }
    let mut xs = self.nil()?;
    for object in build.iter().rev() {
      xs = self.new_pair(*object, xs)?;
    }
    return Ok(xs);
  }

  pub fn quote(&self, root: Pointer, buf: &mut String) -> Result<()> {
    match self.get_ref(root)? {
      &Object::Nil => {
        //
      }
      &Object::Number(value) => {
        let string = value.to_string();
        buf.push_str(&string);
      }
      &Object::Symbol(ref value) => {
        buf.push_str(&value);
      }
      &Object::Wrap(body) => {
        buf.push('[');
        self.quote(body, buf)?;
        buf.push(']');
      }
      &Object::Pair(fst, snd) => {
        self.quote(fst, buf);
        if !self.is_nil(snd)? {
          buf.push(' ');
          self.quote(snd, buf);
        }
      }
    }
    return Ok(());
  }

  fn visit(&mut self, root: Pointer) -> Result<()> {
    match &mut self.nodes[root.index] {
      &mut Some(ref mut node) => {
        if node.generation != root.generation {
          return Err(Error::Null);
        }
        node.is_visible = true;
        match &node.object {
          &Object::Wrap(body) => {
            return self.visit(body);
          }
          &Object::Pair(fst, snd) => {
            self.visit(fst)?;
            return self.visit(snd);
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

  fn put(&mut self, object: Object) -> Result<Pointer> {
    for (index, maybe_node) in self.nodes.iter_mut().enumerate() {
      if maybe_node.is_some() {
        continue;
      }
      let node = Node::new(object, self.generation);
      let pointer = Pointer::new(index, self.generation);
      *maybe_node = Some(node);
      return Ok(pointer);
    }
    return Err(Error::Space);
  }

  fn get_ref(&self, pointer: Pointer) -> Result<&Object> {
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
