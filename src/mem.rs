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

pub type Tab = HashMap<Rc<str>, mem::Ptr>;

enum Pro {
  Set(Ptr),
  Num(Ptr),
  All(Ptr, Ptr),
}

enum Obj {
  Nil,
  Num(Num),
  Sym(Rc<str>),
  Fun(Fun),
  Abs(Ptr),
  Arr(Ptr),
  Cat(Ptr, Ptr),
  Pro(Pro),
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
  fn id() -> Self {
    Ptr {
      index: 0,
      generation: 0,
    }
  }

  fn new(index: usize, generation: u64) -> Self {
    Ptr {
      index: index,
      generation: generation,
    }
  }
}

impl Pro {
  fn is_num(&self) -> bool {
    match self {
      Pro::Num(_) => true,
      _ => false,
    }
  }

  fn is_set(&self) -> bool {
    match self {
      Pro::Set(_) => true,
      _ => false,
    }
  }

  fn is_all(&self) -> bool {
    match self {
      Pro::All(_, _) => true,
      _ => false,
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

  fn is_fun(&self) -> bool {
    match self {
      Obj::Fun(_) => true,
      _ => false,
    }
  }

  fn is_pro(&self) -> bool {
    match self {
      Obj::Pro(_) => true,
      _ => false,
    }
  }

  fn is_num(&self) -> bool {
    match self {
      Obj::Num(_) => true,
      _ => false,
    }
  }

  fn is_sym(&self) -> bool {
    match self {
      Obj::Sym(_) => true,
      _ => false,
    }
  }

  fn is_abs(&self) -> bool {
    match self {
      Obj::Abs(_) => true,
      _ => false,
    }
  }

  fn is_arr(&self) -> bool {
    match self {
      Obj::Arr(_) => true,
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

  pub fn new_fun(&mut self, func: Fun) -> Result<Ptr> {
    let object = Obj::Fun(func);
    return self.put(object);
  }

  pub fn new_nump(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Pro(Pro::Num(body));
    return self.put(object);
  }

  pub fn new_setp(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Pro(Pro::Set(body));
    return self.put(object);
  }

  pub fn new_allp(&mut self, fst: Ptr, snd: Ptr) -> Result<Ptr> {
    let object = Obj::Pro(Pro::All(fst, snd));
    return self.put(object);
  }

  /// Creates a new number.
  pub fn new_num(&mut self, value: Num) -> Result<Ptr> {
    let object = Obj::Num(value);
    return self.put(object);
  }

  /// Creates a new symbol.
  pub fn new_sym(&mut self, value: Rc<str>) -> Result<Ptr> {
    let object = Obj::Sym(value);
    return self.put(object);
  }

  /// Creates a new abstraction.
  pub fn new_abs(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Abs(body);
    return self.put(object);
  }

  /// Creates a new arrow.
  pub fn new_arr(&mut self, body: Ptr) -> Result<Ptr> {
    let object = Obj::Arr(body);
    return self.put(object);
  }

  /// Creates a new catenation.
  pub fn new_cat(&mut self, fst: Ptr, snd: Ptr) -> Result<Ptr> {
    let object = Obj::Cat(fst, snd);
    return self.put(object);
  }

  /// Predicates the nil object.
  pub fn is_nil(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_nil());
  }

  /// Predicates functions.
  pub fn is_fun(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_fun());
  }

  /// Predicates propositions.
  pub fn is_pro(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_pro());
  }

  /// Predicates propositions.
  pub fn is_nump(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_pro_ref(pointer)?;
    return Ok(object.is_num());
  }

  pub fn is_setp(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_pro_ref(pointer)?;
    return Ok(object.is_set());
  }

  pub fn is_allp(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_pro_ref(pointer)?;
    return Ok(object.is_all());
  }

  /// Predicates numbers.
  pub fn is_num(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_num());
  }

  /// Predicates symbols.
  pub fn is_sym(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_sym());
  }

  /// Predicates abstractions.
  pub fn is_abs(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_abs());
  }

  /// Predicates arrows.
  pub fn is_arr(&self, pointer: Ptr) -> Result<bool> {
    let object = self.get_ref(pointer)?;
    return Ok(object.is_arr());
  }

  /// Predicates catenations.
  pub fn is_cat(&self, pointer: Ptr) -> Result<bool> {
    match self.get_ref(pointer)? {
      &Obj::Cat(_, _) => {
        return Ok(true);
      }
      _ => {
        return Ok(false);
      }
    }
  }

  /// Get the value of a number.
  pub fn get_num(&self, pointer: Ptr) -> Result<Num> {
    match self.get_ref(pointer)? {
      &Obj::Num(value) => {
        return Ok(value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_fun(&self, pointer: Ptr) -> Result<Fun> {
    match self.get_ref(pointer)? {
      &Obj::Fun(ref value) => {
        return Ok(*value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_setp_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_pro_ref(pointer)? {
      &Pro::Set(body) => {
        return Ok(body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_nump_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_pro_ref(pointer)? {
      &Pro::Num(body) => {
        return Ok(body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_allp_fst(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_pro_ref(pointer)? {
      &Pro::All(fst, _) => {
        return Ok(fst);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  pub fn get_allp_snd(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_pro_ref(pointer)? {
      &Pro::All(_, snd) => {
        return Ok(snd);
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

  /// Get the body of an abstraction.
  pub fn get_abs_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Abs(ref body) => {
        return Ok(*body);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }

  /// Get the body of an arrow.
  pub fn get_arr_body(&self, pointer: Ptr) -> Result<Ptr> {
    match self.get_ref(pointer)? {
      &Obj::Arr(ref body) => {
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
          &Obj::Abs(body) => {
            return self.mark(body);
          }
          &Obj::Arr(body) => {
            return self.mark(body);
          }
          &Obj::Pro(ref value) => {
            match value {
              &Pro::Set(body) => {
                return self.mark(body);
              }
              &Pro::Num(body) => {
                return self.mark(body);
              }
              &Pro::All(fst, snd) => {
                self.mark(fst)?;
                return self.mark(snd);
              }
            }
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
          xs = self.new_arr(xs)?;
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
          xs = self.new_abs(xs)?;
          build = prev;
          build.push(xs);
        }
        "%app" => {
          let fun = Fun::App;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%box" => {
          let fun = Fun::Box;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%cat" => {
          let fun = Fun::Cat;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%cpy" => {
          let fun = Fun::Cpy;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%drp" => {
          let fun = Fun::Drp;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%swp" => {
          let fun = Fun::Swp;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%fix" => {
          let fun = Fun::Fix;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%run" => {
          let fun = Fun::Run;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%jmp" => {
          let fun = Fun::Jmp;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%num" => {
          let fun = Fun::Num;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%set" => {
          let fun = Fun::Set;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%all" => {
          let fun = Fun::All;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%min" => {
          let fun = Fun::Min;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%max" => {
          let fun = Fun::Max;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%add" => {
          let fun = Fun::Add;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%neg" => {
          let fun = Fun::Neg;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%mul" => {
          let fun = Fun::Mul;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%inv" => {
          let fun = Fun::Inv;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%exp" => {
          let fun = Fun::Exp;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%log" => {
          let fun = Fun::Log;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%cos" => {
          let fun = Fun::Cos;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%sin" => {
          let fun = Fun::Sin;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%abs" => {
          let fun = Fun::Abs;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%cel" => {
          let fun = Fun::Cel;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        "%flr" => {
          let fun = Fun::Flr;
          let object = self.new_fun(fun)?;
          build.push(object);
        }
        _ => {
          match word.parse::<Num>() {
            Ok(value) => {
              let object = self.new_num(value)?;
              build.push(object);
            }
            Err(error) => {
              let object = self.new_sym(word.into())?;
              build.push(object);
            }
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
      &Obj::Fun(ref value) => {
        match value {
          Fun::App => {
            buf.push_str("%app");
          }
          Fun::Box => {
            buf.push_str("%box");
          }
          Fun::Cat => {
            buf.push_str("%cat");
          }
          Fun::Cpy => {
            buf.push_str("%cpy");
          }
          Fun::Drp => {
            buf.push_str("%drp");
          }
          Fun::Swp => {
            buf.push_str("%swp");
          }
          Fun::Fix => {
            buf.push_str("%fix");
          }
          Fun::Run => {
            buf.push_str("%run");
          }
          Fun::Jmp => {
            buf.push_str("%jmp");
          }
          Fun::Num => {
            buf.push_str("%num");
          }
          Fun::Set => {
            buf.push_str("%set");
          }
          Fun::All => {
            buf.push_str("%all");
          }
          Fun::Min => {
            buf.push_str("%min");
          }
          Fun::Max => {
            buf.push_str("%max");
          }
          Fun::Add => {
            buf.push_str("%add");
          }
          Fun::Neg => {
            buf.push_str("%neg");
          }
          Fun::Mul => {
            buf.push_str("%mul");
          }
          Fun::Inv => {
            buf.push_str("%inv");
          }
          Fun::Exp => {
            buf.push_str("%exp");
          }
          Fun::Log => {
            buf.push_str("%log");
          }
          Fun::Cos => {
            buf.push_str("%cos");
          }
          Fun::Sin => {
            buf.push_str("%sin");
          }
          Fun::Abs => {
            buf.push_str("%abs");
          }
          Fun::Cel => {
            buf.push_str("%cel");
          }
          Fun::Flr => {
            buf.push_str("%flr");
          }
        }
      }
      &Obj::Pro(ref value) => {
        match value {
          &Pro::Num(body) => {
            self.quote(body, buf)?;
            buf.push_str(" %num");
          }
          &Pro::Set(body) => {
            self.quote(body, buf)?;
            buf.push_str(" %set");
          }
          &Pro::All(fst, snd) => {
            self.quote(fst, buf)?;
            buf.push(' ');
            self.quote(snd, buf)?;
            buf.push_str(" %all");
          }
        }
      }
      &Obj::Num(value) => {
        let string = value.to_string();
        buf.push_str(&string);
      }
      &Obj::Sym(ref value) => {
        buf.push_str(&value);
      }
      &Obj::Abs(body) => {
        buf.push('[');
        self.quote(body, buf)?;
        buf.push(']');
      }
      &Obj::Arr(body) => {
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

  fn get_pro_ref(&self, pointer: Ptr) -> Result<&Pro> {
    match self.get_ref(pointer)? {
      &Obj::Pro(ref value) => {
        return Ok(value);
      }
      _ => {
        return Err(Error::Tag);
      }
    }
  }
}
