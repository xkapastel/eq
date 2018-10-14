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
  continuation: mem::Ptr,
  mem: &mut mem::Mem,
  tab: &mem::Tab,
  mut time_quota: u64) -> Result<mem::Ptr> {
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

fn invert(x: Num) -> Num {
  if x == 0.0 {
    return 0.0;
  }
  return 1.0 / x;
}

fn num_unop(
  func: &Fn(Num) -> Num,
  source: mem::Ptr,
  mem: &mut mem::Mem) -> Result<mem::Ptr> {
  if mem.is_num(source)? {
    let source_value = mem.get_num(source)?;
    let target_value = func(source_value);
    return mem.new_num(target_value);
  } else if mem.is_fun(source)? {
    let source_body = mem.get_fun_body(source)?;
    let target_body = num_unop(func, source_body, mem)?;
    return mem.new_fun(target_body);
  } else if mem.is_cat(source)? {
    let source_fst = mem.get_cat_fst(source)?;
    let source_snd = mem.get_cat_snd(source)?;
    let target_fst = num_unop(func, source_fst, mem)?;
    let target_snd = num_unop(func, source_snd, mem)?;
    return mem.new_cat(target_fst, target_snd);
  } else if mem.is_nil(source)? {
    return Ok(source);
  } else {
    return Err(Error::Tag);
  }
}

fn num_binop(
  func: &Fn(Num, Num) -> Num,
  lhs: mem::Ptr,
  rhs: mem::Ptr,
  mem: &mut mem::Mem) -> Result<mem::Ptr> {
  if mem.is_num(lhs)? {
    assert(mem.is_num(rhs))?;
    let lhs_value = mem.get_num(lhs)?;
    let rhs_value = mem.get_num(rhs)?;
    let target_value = func(lhs_value, rhs_value);
    return mem.new_num(target_value);
  } else if mem.is_fun(lhs)? {
    assert(mem.is_fun(rhs))?;
    let lhs_body = mem.get_fun_body(lhs)?;
    let rhs_body = mem.get_fun_body(rhs)?;
    let target_body = num_binop(func, lhs_body, rhs_body, mem)?;
    return mem.new_fun(target_body);
  } else if mem.is_cat(lhs)? {
    assert(mem.is_cat(rhs))?;
    let lhs_fst = mem.get_cat_fst(lhs)?;
    let lhs_snd = mem.get_cat_snd(lhs)?;
    let rhs_fst = mem.get_cat_fst(rhs)?;
    let rhs_snd = mem.get_cat_snd(rhs)?;
    let target_fst = num_binop(func, lhs_fst, rhs_fst, mem)?;
    let target_snd = num_binop(func, lhs_snd, rhs_snd, mem)?;
    return mem.new_cat(target_fst, target_snd);
  } else if mem.is_nil(lhs)? {
    assert(mem.is_nil(rhs))?;
    return Ok(lhs);
  } else {
    return Err(Error::Tag);
  }
}

use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Frame {
  con: VecDeque<mem::Ptr>,
  env: Vec<mem::Ptr>,
  err: Vec<mem::Ptr>,
}

impl Frame {
  fn new(root: mem::Ptr) -> Self {
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
  pub fn with_continuation(continuation: mem::Ptr) -> Self {
    Thread {
      frame: Frame::new(continuation),
      stack: Vec::new(),
    }
  }

  pub fn has_continuation(&self) -> bool {
    return !self.frame.con.is_empty() || !self.stack.is_empty();
  }

  pub fn get_continuation(
    &mut self, mem: &mut mem::Mem) -> Result<mem::Ptr> {
    let mut xs = mem.new_nil()?;
    for object in self.frame.con.iter() {
      xs = mem.new_cat(*object, xs)?;
    }
    self.frame.con.clear();
    return Ok(xs);
  }

  pub fn push_continuation_front(&mut self, data: mem::Ptr) {
    self.frame.con.push_front(data);
  }

  pub fn push_continuation_back(&mut self, data: mem::Ptr) {
    self.frame.con.push_back(data);
  }

  pub fn pop_continuation(
    &mut self, mem: &mut mem::Mem) -> Result<mem::Ptr> {
    loop {
      if self.frame.con.is_empty() {
        if self.stack.is_empty() {
          return Err(Error::Underflow);
        }
        let mut previous = self.stack.pop().ok_or(Error::Bug)?;
        if self.frame.is_thunked() {
          let env = self.get_environment(mem)?;
          self.frame = previous;
          self.thunk(env);
        } else {
          previous.env.append(&mut self.frame.env);
          self.frame = previous;
        }
      }
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
    &mut self, mem: &mut mem::Mem) -> Result<mem::Ptr> {
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

  pub fn push_environment(&mut self, data: mem::Ptr) {
    self.frame.env.push(data);
  }

  pub fn pop_environment(&mut self) -> Result<mem::Ptr> {
    return self.frame.env.pop().ok_or(Error::Underflow);
  }

  pub fn peek_environment(&mut self) -> Result<mem::Ptr> {
    return self.frame.env.last().map(|x| *x).ok_or(Error::Underflow);
  }

  pub fn push_frame(&mut self, root: mem::Ptr) {
    self.stack.push(self.frame.clone());
    self.frame = Frame::new(root);
  }

  pub fn thunk(&mut self, root: mem::Ptr) {
    self.frame.err.append(&mut self.frame.env);
    self.frame.err.push(root);
  }

  pub fn step(
    &mut self,
    mem: &mut mem::Mem,
    tab: &HashMap<Rc<str>, mem::Ptr>) -> Result<()> {
    let code = self.pop_continuation(mem)?;
    if mem.is_num(code)? {
      self.push_environment(code);
    } else if mem.is_pro(code)? {
      self.push_environment(code);
    } else if mem.is_fun(code)? {
      self.push_environment(code);
    } else if mem.is_cmd(code)? {
      let body = mem.get_cmd_body(code)?;
      self.push_frame(body);
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
          let nil = mem.new_nil()?;
          let source = self.pop_environment()?;
          let target = mem.new_fun(source)?;
          let target = mem.new_cat(target, nil)?;
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
        Bit::Cpy => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.peek_environment()?;
          self.push_environment(source);
        }
        Bit::Drp => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          self.pop_environment()?;
        }
        Bit::Swp => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let fst = self.pop_environment()?;
          let snd = self.pop_environment()?;
          self.push_environment(fst);
          self.push_environment(snd);
        }
        Bit::Fix => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_body = mem.get_fun_body(source)?;
          let fixed = mem.new_cat(source, code)?;
          let target_body = mem.new_cat(fixed, source_body)?;
          let target = mem.new_fun(target_body)?;
          self.push_environment(target);
        }
        Bit::Run => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let source_body = mem.get_fun_body(source)?;
          let target = mem.new_cmd(source_body)?;
          self.push_continuation_front(target);
        }
        Bit::Jmp => {
          if !self.is_monadic() || self.stack.is_empty() {
            self.thunk(code);
            return Ok(());
          }
          let callback = self.pop_environment()?;
          let callback_body = mem.get_fun_body(callback)?;
          let env_body = self.get_environment(mem)?;
          let con_body = self.get_continuation(mem)?;
          let env_body = mem.new_cmd(env_body)?;
          let con_body = mem.new_cmd(con_body)?;
          let environment = mem.new_fun(env_body)?;
          let continuation = mem.new_fun(con_body)?;
          self.push_environment(environment);
          self.push_environment(continuation);
          self.push_continuation_front(callback_body);
        }
        Bit::Num => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = mem.new_nump(source)?;
          self.push_environment(target);
        }
        Bit::Set => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = mem.new_setp(source)?;
          self.push_environment(target);
        }
        Bit::All => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let target = mem.new_allp(fst, snd)?;
          self.push_environment(target);
        }
        Bit::Min => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let target = num_binop(&|x, y| x.min(y), fst, snd, mem)?;
          self.push_environment(target);
        }
        Bit::Max => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let target = num_binop(&|x, y| x.max(y), fst, snd, mem)?;
          self.push_environment(target);
        }
        Bit::Add => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let target = num_binop(&|x, y| x + y, fst, snd, mem)?;
          self.push_environment(target);
        }
        Bit::Neg => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| 0.0 - x, source, mem)?;
          self.push_environment(target);
        }
        Bit::Mul => {
          if !self.is_dyadic() {
            self.thunk(code);
            return Ok(());
          }
          let snd = self.pop_environment()?;
          let fst = self.pop_environment()?;
          let target = num_binop(&|x, y| x * y, fst, snd, mem)?;
          self.push_environment(target);
        }
        Bit::Inv => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&invert, source, mem)?;
          self.push_environment(target);
        }
        Bit::Exp => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.exp(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Log => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.ln(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Cos => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.cos(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Sin => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.sin(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Abs => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.abs(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Cel => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.ceil(), source, mem)?;
          self.push_environment(target);
        }
        Bit::Flr => {
          if !self.is_monadic() {
            self.thunk(code);
            return Ok(());
          }
          let source = self.pop_environment()?;
          let target = num_unop(&|x| x.floor(), source, mem)?;
          self.push_environment(target);
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
