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

enum Object {
  Id,
  Number(Number),
  Word(Rc<str>),
  Function(Function),
  Block(Pointer),
  Arrow(Pointer),
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

  fn is_compose(&self) -> bool {
    match self {
      Function::Compose => true,
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

  fn is_swap(&self) -> bool {
    match self {
      Function::Swap => true,
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

  fn is_arrow(&self) -> bool {
    match self {
      Object::Arrow(_) => true,
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

  /// Creates a new arrow.
  pub fn new_arrow(&mut self, body: Pointer) -> Result<Pointer> {
    let object = Object::Arrow(body);
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
    let object = self.get_function(pointer)?;
    return Ok(object.is_apply());
  }

  pub fn is_bind(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_bind());
  }

  pub fn is_compose(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_compose());
  }

  pub fn is_copy(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_copy());
  }

  pub fn is_drop(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_drop());
  }

  pub fn is_swap(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_swap());
  }

  pub fn is_fix(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_fix());
  }

  pub fn is_shift(&self, pointer: Pointer) -> Result<bool> {
    if !self.is_function(pointer)? {
      return Ok(false);
    }
    let object = self.get_function(pointer)?;
    return Ok(object.is_shift());
  }

  /// Predicates blocks.
  pub fn is_block(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_block());
  }

  /// Predicates arrows.
  pub fn is_arrow(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_arrow());
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

  /// Get the body of an arrow.
  pub fn get_arrow_body(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_ref(pointer)? {
      &Object::Arrow(ref body) => {
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
          let mut xs = self.new_id()?;
          for object in build.iter().rev() {
            xs = self.new_sequence(*object, xs)?;
          }
          xs = self.new_arrow(xs)?;
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
          let mut xs = self.new_id()?;
          for object in build.iter().rev() {
            xs = self.new_sequence(*object, xs)?;
          }
          xs = self.new_block(xs)?;
          build = prev;
          build.push(xs);
        }
        "apply" => {
          let func = Function::Apply;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "bind" => {
          let func = Function::Bind;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "compose" => {
          let func = Function::Compose;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "copy" => {
          let func = Function::Copy;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "drop" => {
          let func = Function::Drop;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "swap" => {
          let func = Function::Swap;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "fix" => {
          let func = Function::Fix;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "shift" => {
          let func = Function::Shift;
          let object = self.new_function(func)?;
          build.push(object);
        }
        _ => {
          let object = self.new_word(word.into())?;
          build.push(object);
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
            buf.push_str("apply");
          }
          Function::Bind => {
            buf.push_str("bind");
          }
          Function::Compose => {
            buf.push_str("compose");
          }
          Function::Copy => {
            buf.push_str("copy");
          }
          Function::Drop => {
            buf.push_str("drop");
          }
          Function::Swap => {
            buf.push_str("swap");
          }
          Function::Fix => {
            buf.push_str("fix");
          }
          Function::Shift => {
            buf.push_str("shift");
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
      &Object::Arrow(body) => {
        buf.push_str("{ ");
        self.quote(body, buf)?;
        buf.push_str(" }");
      }
      &Object::Sequence(head, tail) => {
        self.quote(head, buf)?;
        if !self.is_id(tail)? {
          buf.push(' ');
          self.quote(tail, buf)?;
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

  pub fn get_function(&self, pointer: Pointer) -> Result<Function> {
    match self.get_ref(pointer)? {
      &Object::Function(ref value) => {
        return Ok(*value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }
}
