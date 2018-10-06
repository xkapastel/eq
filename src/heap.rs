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

enum Function {
  Apply,
  Bind,
  Copy,
  Drop,
  Fix,
  Shift,
  Reset,
}

enum Object {
  Id,
  Number(Number),
  Word(Rc<str>),
  Function(Function),
  Block(Pointer),
  Sequence(Pointer, Pointer),
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
  fn id() -> Self {
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

impl Function {
  fn is_apply(&self) -> bool {
    match self {
      Function::Apply => true,
      _ => false,
    }
  }

  fn is_bind(&self) -> bool {
    match self {
      Function::Bind => true,
      _ => false,
    }
  }

  fn is_copy(&self) -> bool {
    match self {
      Function::Copy => true,
      _ => false,
    }
  }

  fn is_drop(&self) -> bool {
    match self {
      Function::Drop => true,
      _ => false,
    }
  }

  fn is_fix(&self) -> bool {
    match self {
      Function::Fix => true,
      _ => false,
    }
  }

  fn is_shift(&self) -> bool {
    match self {
      Function::Shift => true,
      _ => false,
    }
  }

  fn is_reset(&self) -> bool {
    match self {
      Function::Reset => true,
      _ => false,
    }
  }
}

impl Object {
  fn is_id(&self) -> bool {
    match self {
      Object::Id => true,
      _ => false,
    }
  }

  fn is_function(&self) -> bool {
    match self {
      Object::Function(_) => true,
      _ => false,
    }
  }

  fn is_number(&self) -> bool {
    match self {
      Object::Number(_) => true,
      _ => false,
    }
  }

  fn is_word(&self) -> bool {
    match self {
      Object::Word(_) => true,
      _ => false,
    }
  }

  fn is_block(&self) -> bool {
    match self {
      Object::Block(_) => true,
      _ => false,
    }
  }

  fn is_sequence(&self) -> bool {
    match self {
      Object::Sequence(_, _) => true,
      _ => false,
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
    for _ in 0..capacity {
      nodes.push(None);
    }
    Heap {
      nodes: nodes,
      generation: 0,
    }
  }
  
  /// Returns the id object
  pub fn new_id(&mut self) -> Result<Pointer> {
    let object = Object::Id;
    return self.put(object);
  }

  fn new_function(&mut self, func: Function) -> Result<Pointer> {
    let object = Object::Function(func);
    return self.put(object);
  }

  /// Creates a new number.
  pub fn new_number(&mut self, value: Number) -> Result<Pointer> {
    let object = Object::Number(value);
    return self.put(object);
  }

  /// Creates a new word.
  pub fn new_word(&mut self, value: Rc<str>) -> Result<Pointer> {
    let object = Object::Word(value);
    return self.put(object);
  }

  /// Creates a new block.
  pub fn new_block(&mut self, body: Pointer) -> Result<Pointer> {
    let object = Object::Block(body);
    return self.put(object);
  }

  /// Creates a new sequence.
  pub fn new_sequence(
    &mut self,
    head: Pointer,
    tail: Pointer) -> Result<Pointer> {
    let object = Object::Sequence(head, tail);
    return self.put(object);
  }

  /// Predicates the id object.
  pub fn is_id(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_id());
  }

  /// Predicates functions.
  pub fn is_function(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_function());
  }

  /// Predicates numbers.
  pub fn is_number(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_number());
  }

  /// Predicates words.
  pub fn is_word(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_word());
  }

  pub fn is_apply(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_apply());
  }

  pub fn is_bind(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_bind());
  }

  pub fn is_copy(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_copy());
  }

  pub fn is_drop(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_drop());
  }

  pub fn is_fix(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_fix());
  }

  pub fn is_shift(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_shift());
  }

  pub fn is_reset(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function_ref(pointer)?;
    return Ok(object.is_reset());
  }

  /// Predicates blocks.
  pub fn is_block(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_block());
  }

  /// Predicates sequences.
  pub fn is_sequence(&self, pointer: Pointer) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Object::Sequence(_, _) => {
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

  /// Get the value of a word.
  pub fn get_word(&self, pointer: Pointer) -> Result<Rc<str>> {
    match self.get_ref(pointer)? {
      &Object::Word(ref value) => {
        return Ok(value.clone());
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the body of a block.
  pub fn get_block_body(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Block(ref body) => {
        return Ok(*body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the first element of a sequence.
  pub fn get_sequence_head(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Sequence(ref head, _) => {
        return Ok(*head);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the second element of a sequence.
  pub fn get_sequence_tail(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Sequence(_, ref tail) => {
        return Ok(*tail);
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

  pub fn parse(&mut self, src: &str) -> Result<Pointer> {
    let mut build = Vec::new();
    let mut stack = Vec::new();
    for rune in src.chars() {
      match rune {
        '[' => {
          stack.push(build);
          build = Vec::new();
        }
        ']' => {
          let prev = stack.pop().ok_or(Error::Syntax)?;
          let mut xs = self.new_id()?;
          for object in build.iter().rev() {
            xs = self.new_sequence(*object, xs)?;
          }
          xs = self.new_block(xs)?;
          build = prev;
          build.push(xs);
        }
        'a' => {
          let func = Function::Apply;
          let object = self.new_function(func)?;
          build.push(object);
        }
        'b' => {
          let func = Function::Bind;
          let object = self.new_function(func)?;
          build.push(object);
        }
        'c' => {
          let func = Function::Copy;
          let object = self.new_function(func)?;
          build.push(object);
        }
        'd' => {
          let func = Function::Drop;
          let object = self.new_function(func)?;
          build.push(object);
        }
        'f' => {
          let func = Function::Fix;
          let object = self.new_function(func)?;
          build.push(object);
        }
        's' => {
          let func = Function::Shift;
          let object = self.new_function(func)?;
          build.push(object);
        }
        'r' => {
          let func = Function::Reset;
          let object = self.new_function(func)?;
          build.push(object);
        }
        ' ' | '\t' | '\r' | '\n' => {
          //
        }
        _ => {
          if rune.is_uppercase() {
            let object = self.new_word(rune.to_string().into())?;
            build.push(object);
          } else {
            return Err(Error::Syntax);
          }
        }
      }
    }
    if !stack.is_empty() {
      return Err(Error::Syntax);
    }
    let mut xs = self.new_id()?;
    for object in build.iter().rev() {
      xs = self.new_sequence(*object, xs)?;
    }
    return Ok(xs);
  }

  pub fn quote(&self, root: Pointer, buf: &mut String) -> Result<()> {
    match self.get_ref(root)? {
      &Object::Id => {
        //
      }
      &Object::Function(ref value) => {
        match value {
          Function::Apply => {
            buf.push('a');
          }
          Function::Bind => {
            buf.push('b');
          }
          Function::Copy => {
            buf.push('c');
          }
          Function::Drop => {
            buf.push('d');
          }
          Function::Fix => {
            buf.push('f');
          }
          Function::Shift => {
            buf.push('s');
          }
          Function::Reset => {
            buf.push('r');
          }
        }
      }
      &Object::Number(value) => {
        let string = value.to_string();
        buf.push_str(&string);
      }
      &Object::Word(ref value) => {
        buf.push_str(&value);
      }
      &Object::Block(body) => {
        buf.push('[');
        self.quote(body, buf)?;
        buf.push(']');
      }
      &Object::Sequence(head, tail) => {
        self.quote(head, buf)?;
        self.quote(tail, buf)?;
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
          &Object::Block(body) => {
            return self.visit(body);
          }
          &Object::Sequence(head, tail) => {
            self.visit(head)?;
            return self.visit(tail);
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

  fn get_function_ref(&self, pointer: Pointer) -> Result<&Function> {
    match self.get_ref(pointer)? {
      &Object::Function(ref value) => {
        return Ok(value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }
}
