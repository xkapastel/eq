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

pub fn reduce(
  continuation: heap::Pointer,
  heap: &mut heap::Heap,
  dictionary: &Dictionary,
  mut time_quota: u64) -> Result<heap::Pointer> {
  let mut thread = Thread::with_continuation(continuation);
  while time_quota > 0 && thread.has_continuation() {
    time_quota -= 1;
    thread.step(heap, dictionary)?;
  }
  if thread.has_continuation() {
    let tail = thread.get_continuation(heap)?;
    let head = thread.get_environment(heap)?;
    return heap.new_sequence(head, tail);
  }
  return thread.get_environment(heap);
}

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Frame {
  con: VecDeque<heap::Pointer>,
  env: Vec<heap::Pointer>,
  err: Vec<heap::Pointer>,
}

impl Frame {
  fn new(root: heap::Pointer) -> Self {
    let mut con = VecDeque::new();
    con.push_back(root);
    Frame {
      con: con,
      env: vec![],
      err: vec![],
    }
  }

  fn is_thunked(&self) -> bool {
    return !self.err.is_empty();
  }
}

use std::rc::Rc;
use std::collections::HashMap;

pub struct Thread {
  frame: Frame,
  stack: Vec<Frame>,
}

impl Thread {
  pub fn with_continuation(continuation: heap::Pointer) -> Self {
    Thread {
      frame: Frame::new(continuation),
      stack: Vec::new(),
    }
  }

  pub fn has_continuation(&self) -> bool {
    return !self.frame.con.is_empty() || !self.stack.is_empty();
  }

  pub fn get_continuation(
    &mut self, heap: &mut heap::Heap) -> Result<heap::Pointer> {
    let mut xs = heap.new_id()?;
    for object in self.frame.con.iter() {
      xs = heap.new_sequence(*object, xs)?;
    }
    self.frame.con.clear();
    return Ok(xs);
  }

  pub fn push_continuation_front(&mut self, data: heap::Pointer) {
    self.frame.con.push_front(data);
  }

  pub fn push_continuation_back(&mut self, data: heap::Pointer) {
    self.frame.con.push_back(data);
  }

  pub fn pop_continuation(
    &mut self, heap: &mut heap::Heap) -> Result<heap::Pointer> {
    loop {
      if self.frame.con.is_empty() {
        if self.stack.is_empty() {
          return Err(Error::Underflow);
        }
        let mut previous = self.stack.pop().ok_or(Error::Bug)?;
        if self.frame.is_thunked() {
          let arrow_body = self.get_environment(heap)?;
          let arrow = heap.new_arrow(arrow_body)?;
          self.frame = previous;
          self.thunk(arrow);
        } else {
          previous.env.append(&mut self.frame.env);
          self.frame = previous;
        }
      }
      let code = self.frame.con.pop_front().ok_or(Error::Bug)?;
      if heap.is_sequence(code)? {
        let head = heap.get_sequence_head(code)?;
        let tail = heap.get_sequence_tail(code)?;
        self.frame.con.push_front(tail);
        self.frame.con.push_front(head);
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
    &mut self, heap: &mut heap::Heap) -> Result<heap::Pointer> {
    let mut xs = heap.new_id()?;
    for object in self.frame.env.iter().rev() {
      xs = heap.new_sequence(*object, xs)?;
    }
    for object in self.frame.err.iter().rev() {
      xs = heap.new_sequence(*object, xs)?;
    }
    self.frame.env.clear();
    self.frame.err.clear();
    return Ok(xs);
  }

  pub fn push_environment(&mut self, data: heap::Pointer) {
    self.frame.env.push(data);
  }

  pub fn pop_environment(&mut self) -> Result<heap::Pointer> {
    return self.frame.env.pop().ok_or(Error::Underflow);
  }

  pub fn peek_environment(&mut self) -> Result<heap::Pointer> {
    return self.frame.env.last().map(|x| *x).ok_or(Error::Underflow);
  }

  pub fn push_frame(&mut self, root: heap::Pointer) {
    self.stack.push(self.frame.clone());
    self.frame = Frame::new(root);
  }

  pub fn thunk(&mut self, root: heap::Pointer) {
    self.frame.err.append(&mut self.frame.env);
    self.frame.err.push(root);
  }

  pub fn step(
    &mut self,
    heap: &mut heap::Heap,
    dictionary: &HashMap<Rc<str>, heap::Pointer>) -> Result<()> {
    let code = self.pop_continuation(heap)?;
    if heap.is_arrow(code)? {
      let body = heap.get_arrow_body(code)?;
      self.push_frame(body);
    } else if heap.is_block(code)? {
      self.push_environment(code);
    } else if heap.is_number(code)? {
      self.push_environment(code);
    } else if heap.is_function(code)? {
      match heap.get_function(code)? {
        Function::App => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = heap.get_block_body(source)?;
          self.push_continuation_front(target);
        }
        Function::Box => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = heap.new_block(source)?;
          self.push_environment(target);
        }
        Function::Cat => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let rhs = self.pop_environment()?;
          let lhs = self.pop_environment()?;
          let rhs_body = heap.get_block_body(rhs)?;
          let lhs_body = heap.get_block_body(lhs)?;
          let target_body = heap.new_sequence(lhs_body, rhs_body)?;
          let target = heap.new_block(target_body)?;
          self.push_environment(target);
        }
        Function::Copy => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.peek_environment()?;
          self.push_environment(source);
        }
        Function::Drop => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          self.pop_environment()?;
        }
        Function::Swap => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let fst = self.pop_environment()?;
          let snd = self.pop_environment()?;
          self.push_environment(fst);
          self.push_environment(snd);
        }
        Function::Fix => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_body = heap.get_block_body(source)?;
          let fixed = heap.new_sequence(source, code)?;
          let target_body = heap.new_sequence(fixed, source_body)?;
          let target = heap.new_block(target_body)?;
          self.push_environment(target);
        }
        Function::Run => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_body = heap.get_block_body(source)?;
          let target = heap.new_arrow(source_body)?;
          self.push_continuation_front(target);
        }
        Function::Shift => {
          if !self.is_monadic() || self.stack.is_empty() {
            self.thunk(code);
            return Ok(());
          }
          let callback = self.pop_environment()?;
          let callback_body = heap.get_block_body(callback)?;
          let env_body = self.get_environment(heap)?;
          let con_body = self.get_continuation(heap)?;
          let environment = heap.new_block(env_body)?;
          let continuation = heap.new_block(con_body)?;
          self.push_environment(environment);
          self.push_environment(continuation);
          self.push_continuation_front(callback_body);
        }
        Function::Min => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let snd_value = heap.get_number(snd)?;
          let fst_value = heap.get_number(fst)?;
          let target_value = snd_value.min(fst_value);
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Max => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let snd_value = heap.get_number(snd)?;
          let fst_value = heap.get_number(fst)?;
          let target_value = snd_value.max(fst_value);
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Add => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let snd_value = heap.get_number(snd)?;
          let fst_value = heap.get_number(fst)?;
          let target_value = snd_value + fst_value;
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Negate => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = 0.0 - source_value;
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Multiply => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let snd_value = heap.get_number(snd)?;
          let fst_value = heap.get_number(fst)?;
          let target_value = snd_value * fst_value;
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Invert => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          if source_value == 0.0 {
            self.push_environment(source);
            self.thunk(code);
            return Ok(());
          }
          let target_value = 1.0 / source_value;
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Exp => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.exp();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Log => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.ln();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Cos => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.cos();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Sin => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.sin();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Abs => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.abs();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Ceil => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.ceil();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
        Function::Floor => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_value = heap.get_number(source)?;
          let target_value = source_value.floor();
          let target = heap.new_number(target_value)?;
          self.push_environment(target);
        }
      }
    } else if heap.is_word(code)? {
      let code_value = heap.get_word(code)?;
      match dictionary.get(&code_value) {
        Some(binding) => {
          self.push_continuation_front(*binding);
        }
        None => {
          self.thunk(code);
        }
      }
      return Ok(());
    } else if heap.is_id(code)? {
      return Ok(());
    } else {
      return Err(Error::Bug);
    }
    return Ok(());
  }
}
