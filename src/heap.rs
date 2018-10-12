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
pub struct Pointer {
  index: usize,
  generation: u64,
}

enum Prop {
  Type(Pointer),
  Real(Pointer),
  Forall(Pointer, Pointer),
}

enum Object {
  Id,
  Number(Number),
  Word(Rc<str>),
  Function(Function),
  Block(Pointer),
  Arrow(Pointer),
  Sequence(Pointer, Pointer),
  Prop(Prop),
}

struct Node {
  object: Object,
  generation: u64,
  is_visible: bool,
}

/// A garbage-collected heap.
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

impl Prop {
  fn is_real(&self) -> bool {
    match self {
      Prop::Real(_) => true,
      _ => false,
    }
  }

  fn is_type(&self) -> bool {
    match self {
      Prop::Type(_) => true,
      _ => false,
    }
  }

  fn is_forall(&self) -> bool {
    match self {
      Prop::Forall(_, _) => true,
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

  fn is_prop(&self) -> bool {
    match self {
      Object::Prop(_) => true,
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

  pub fn new_function(&mut self, func: Function) -> Result<Pointer> {
    let object = Object::Function(func);
    return self.put(object);
  }

  pub fn new_real(&mut self, body: Pointer) -> Result<Pointer> {
    let object = Object::Prop(Prop::Real(body));
    return self.put(object);
  }

  pub fn new_type(&mut self, body: Pointer) -> Result<Pointer> {
    let object = Object::Prop(Prop::Type(body));
    return self.put(object);
  }

  pub fn new_forall(&mut self, fst: Pointer, snd: Pointer) -> Result<Pointer> {
    let object = Object::Prop(Prop::Forall(fst, snd));
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

  /// Predicates propositions.
  pub fn is_prop(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_prop());
  }

  /// Predicates propositions.
  pub fn is_real(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_prop_ref(pointer)?;
    return Ok(object.is_real());
  }

  pub fn is_type(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_prop_ref(pointer)?;
    return Ok(object.is_type());
  }

  pub fn is_forall(&self, pointer: Pointer) -> Result<bool> {
    let object = self.get_prop_ref(pointer)?;
    return Ok(object.is_forall());
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

  pub fn get_type_body(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_prop_ref(pointer)? {
      &Prop::Type(body) => {
        return Ok(body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_real_body(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_prop_ref(pointer)? {
      &Prop::Real(body) => {
        return Ok(body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_forall_fst(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_prop_ref(pointer)? {
      &Prop::Forall(fst, _) => {
        return Ok(fst);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_forall_snd(&self, pointer: Pointer) -> Result<Pointer> {
    match self.get_prop_ref(pointer)? {
      &Prop::Forall(_, snd) => {
        return Ok(snd);
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

  pub fn mark(&mut self, root: Pointer) -> Result<()> {
    match &mut self.nodes[root.index] {
      &mut Some(ref mut node) => {
        if node.generation != root.generation {
          return Err(Error::Null);
        }
        node.is_visible = true;
        match &node.object {
          &Object::Block(body) => {
            return self.mark(body);
          }
          &Object::Arrow(body) => {
            return self.mark(body);
          }
          &Object::Prop(ref value) => {
            match value {
              &Prop::Type(body) => {
                return self.mark(body);
              }
              &Prop::Real(body) => {
                return self.mark(body);
              }
              &Prop::Forall(fst, snd) => {
                self.mark(fst)?;
                return self.mark(snd);
              }
            }
          }
          &Object::Sequence(head, tail) => {
            self.mark(head)?;
            return self.mark(tail);
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
        "app" => {
          let func = Function::App;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "box" => {
          let func = Function::Box;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "cat" => {
          let func = Function::Cat;
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
        "run" => {
          let func = Function::Run;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "shift" => {
          let func = Function::Shift;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "real" => {
          let func = Function::Real;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "type" => {
          let func = Function::Type;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "forall" => {
          let func = Function::Forall;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "min" => {
          let func = Function::Min;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "max" => {
          let func = Function::Max;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "+" => {
          let func = Function::Add;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "-" => {
          let func = Function::Negate;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "*" => {
          let func = Function::Multiply;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "/" => {
          let func = Function::Invert;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "exp" => {
          let func = Function::Exp;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "log" => {
          let func = Function::Log;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "cos" => {
          let func = Function::Cos;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "sin" => {
          let func = Function::Sin;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "abs" => {
          let func = Function::Abs;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "ceil" => {
          let func = Function::Ceil;
          let object = self.new_function(func)?;
          build.push(object);
        }
        "floor" => {
          let func = Function::Floor;
          let object = self.new_function(func)?;
          build.push(object);
        }
        _ => {
          match word.parse::<Number>() {
            Ok(value) => {
              let object = self.new_number(value)?;
              build.push(object);
            }
            Err(error) => {
              let object = self.new_word(word.into())?;
              build.push(object);
            }
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
          Function::App => {
            buf.push_str("app");
          }
          Function::Box => {
            buf.push_str("box");
          }
          Function::Cat => {
            buf.push_str("cat");
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
          Function::Run => {
            buf.push_str("run");
          }
          Function::Shift => {
            buf.push_str("shift");
          }
          Function::Real => {
            buf.push_str("real");
          }
          Function::Type => {
            buf.push_str("type");
          }
          Function::Forall => {
            buf.push_str("forall");
          }
          Function::Min => {
            buf.push_str("min");
          }
          Function::Max => {
            buf.push_str("max");
          }
          Function::Add => {
            buf.push('+');
          }
          Function::Negate => {
            buf.push('-');
          }
          Function::Multiply => {
            buf.push('*');
          }
          Function::Invert => {
            buf.push('/');
          }
          Function::Exp => {
            buf.push_str("exp");
          }
          Function::Log => {
            buf.push_str("log");
          }
          Function::Cos => {
            buf.push_str("cos");
          }
          Function::Sin => {
            buf.push_str("sin");
          }
          Function::Abs => {
            buf.push_str("abs");
          }
          Function::Ceil => {
            buf.push_str("ceil");
          }
          Function::Floor => {
            buf.push_str("floor");
          }
        }
      }
      &Object::Prop(ref value) => {
        match value {
          &Prop::Real(body) => {
            self.quote(body, buf);
            buf.push_str(" real");
          }
          &Prop::Type(body) => {
            self.quote(body, buf)?;
            buf.push_str(" type");
          }
          &Prop::Forall(fst, snd) => {
            self.quote(fst, buf)?;
            buf.push(' ');
            self.quote(snd, buf)?;
            buf.push_str(" forall");
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

  fn get_prop_ref(&self, pointer: Pointer) -> Result<&Prop> {
    match self.get_ref(pointer)? {
      &Object::Prop(ref value) => {
        return Ok(value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }
}
