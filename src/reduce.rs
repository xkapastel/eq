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

#[derive(Debug, Clone)]
struct Frame {
  con: Vec<heap::Pointer>,
  env: Vec<heap::Pointer>,
  err: Vec<heap::Pointer>,
}

impl Frame {
  fn new(root: heap::Pointer) -> Self {
    Frame {
      con: vec![root],
      env: vec![],
      err: vec![],
    }
  }
}

pub struct Thread {
  frame: Frame,
  stack: Vec<Frame>,
  heap: heap::Heap,
}

impl Thread {
  pub fn with_continuation(
    root: heap::Pointer,
    heap: heap::Heap) -> Self {
    Thread {
      frame: Frame::new(root),
      stack: Vec::new(),
      heap: heap,
    }
  }

  pub fn has_continuation(&self) -> bool {
    return !self.frame.con.is_empty() || !self.stack.is_empty();
  }

  pub fn get_continuation(&mut self) -> Result<heap::Pointer> {
    let mut xs = self.heap.new_id()?;
    for object in self.frame.con.iter() {
      xs = self.heap.new_sequence(*object, xs)?;
    }
    self.frame.con.clear();
    return Ok(xs);
  }

  pub fn push_continuation(&mut self, data: heap::Pointer) {
    self.frame.con.push(data);
  }

  pub fn pop_continuation(&mut self) -> Result<heap::Pointer> {
    loop {
      if self.frame.con.is_empty() {
        if self.stack.is_empty() {
          return Err(Error::Underflow);
        }
        let mut previous = self.stack.pop().ok_or(Error::Bug)?;
        if self.frame.err.is_empty() {
          previous.env.append(&mut self.frame.env);
          self.frame = previous;
        } else {
          let arrow_body = self.get_environment()?;
          let arrow = self.heap.new_arrow(arrow_body)?;
          self.frame = previous;
          self.crash(arrow);
        }
      }
      let code = self.frame.con.pop().ok_or(Error::Bug)?;
      if self.heap.is_sequence(code)? {
        let head = self.heap.get_sequence_head(code)?;
        let tail = self.heap.get_sequence_tail(code)?;
        self.frame.con.push(tail);
        self.frame.con.push(head);
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

  pub fn get_environment(&mut self) -> Result<heap::Pointer> {
    let mut xs = self.heap.new_id()?;
    for object in self.frame.env.iter().rev() {
      xs = self.heap.new_sequence(*object, xs)?;
    }
    for object in self.frame.err.iter().rev() {
      xs = self.heap.new_sequence(*object, xs)?;
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

  pub fn crash(&mut self, root: heap::Pointer) {
    self.frame.err.append(&mut self.frame.env);
    self.frame.err.push(root);
  }

  pub fn step(&mut self) -> Result<()> {
    let code = self.pop_continuation()?;
    if self.heap.is_arrow(code)? {
      let body = self.heap.get_arrow_body(code)?;
      self.push_frame(body);
    } else if self.heap.is_block(code)? {
      self.push_environment(code);
    } else if self.heap.is_number(code)? {
      self.push_environment(code);
    } else if self.heap.is_function(code)? {
      match self.heap.get_function(code)? {
        Function::Apply => {
          if !self.is_monadic() {
            self.crash(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = self.heap.get_block_body(source)?;
          self.push_continuation(target);
        }
        Function::Bind => {
          if !self.is_monadic() {
            self.crash(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = self.heap.new_block(source)?;
          self.push_environment(target);
        }
        Function::Compose => {
          if !self.is_dyadic() {
            self.crash(code);
            return Ok(());
          }
          let rhs = self.pop_environment()?;
          let lhs = self.pop_environment()?;
          let rhs_body = self.heap.get_block_body(rhs)?;
          let lhs_body = self.heap.get_block_body(lhs)?;
          let target_body = self.heap.new_sequence(lhs_body, rhs_body)?;
          let target = self.heap.new_block(target_body)?;
          self.push_environment(target);
        }
        Function::Copy => {
          if !self.is_monadic() {
            self.crash(code);
            return Ok(());
          }
          let source = self.peek_environment()?;
          self.push_environment(source);
        }
        Function::Drop => {
          if !self.is_monadic() {
            self.crash(code);
            return Ok(());
          }
          self.pop_environment()?;
        }
        Function::Swap => {
          if !self.is_dyadic() {
            self.crash(code);
            return Ok(());
          }
          let fst = self.pop_environment()?;
          let snd = self.pop_environment()?;
          self.push_environment(fst);
          self.push_environment(snd);
        }
        Function::Fix => {
          if !self.is_monadic() {
            self.crash(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_body = self.heap.get_block_body(source)?;
          let fixed = self.heap.new_sequence(source, code)?;
          let target_body = self.heap.new_sequence(fixed, source_body)?;
          let target = self.heap.new_block(target_body)?;
          self.push_environment(target);
        }
        Function::Shift => {
          if !self.is_monadic() || self.stack.is_empty() {
            self.crash(code);
            return Ok(());
          }
          let callback = self.pop_environment()?;
          let callback_body = self.heap.get_block_body(callback)?;
          let env_body = self.get_environment()?;
          let con_body = self.get_continuation()?;
          let environment = self.heap.new_block(env_body)?;
          let continuation = self.heap.new_block(con_body)?;
          self.push_environment(environment);
          self.push_environment(continuation);
          self.push_continuation(callback_body);
        }
      }
    } else if self.heap.is_word(code)? {
      self.crash(code);
    } else if self.heap.is_id(code)? {
      //
    } else {
      return Err(Error::Bug);
    }
    return Ok(());
  }

  pub fn dump(mut self, dst: &mut String) -> Result<()> {
    let env = self.get_environment()?;
    let con = self.get_continuation()?;
    self.heap.quote(env, dst)?;
    self.heap.quote(con, dst)?;
    return Ok(());
  }
}
