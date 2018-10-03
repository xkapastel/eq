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

pub struct Machine {
  data: Vec<Pointer>,
  code: Vec<Pointer>,
  heap: Heap,
}

impl Machine {
  fn with_capacity(capacity: usize) -> Self {
    Machine {
      data: Vec::new(),
      code: Vec::new(),
      heap: Heap::with_capacity(capacity),
    }
  }

  fn push_data(&mut self, data: Pointer) -> Result<()> {
    self.data.push(data);
    return Ok(());
  }

  fn pop_data(&mut self) -> Result<Pointer> {
    return self.data.pop().ok_or(Error::Bug);
  }

  fn push_code(&mut self, code: Pointer) -> Result<()> {
    self.code.push(code);
    return Ok(());
  }

  fn pop_code(&mut self) -> Result<Pointer> {
    loop {
      let object = self.code.pop().ok_or(Error::Bug)?;
      if self.heap.is_pair(object)? {
      let fst = self.heap.get_pair_fst(object)?;
        let snd = self.heap.get_pair_snd(object)?;
        self.code.push(snd);
        self.code.push(fst);
      } else {
        return Ok(object);
      }
    }
  }

  fn jump_code(&mut self) -> Result<Pointer> {
    let mut buf = Vec::new();
    loop {
      let object = self.pop_code()?;
      if self.heap.is_reset(object)? {
        self.push_code(object);
        let mut xs = self.heap.nil()?;
        for object in buf.iter().rev() {
          xs = self.heap.new_pair(*object, xs)?;
        }
        return Ok(xs);
      } else {
        buf.push(object);
      }
    }
  }

  fn step(&mut self) -> Result<()> {
    let object = self.pop_code()?;
    if self.heap.is_number(object)? {
      self.push_data(object)?;
    } else if self.heap.is_wrap(object)? {
      self.push_data(object)?;
    } else if self.heap.is_app(object)? {
      let func = self.pop_data()?;
      let hide = self.pop_data()?;
      assert(self.heap.is_wrap(func))?;
      let func_body = self.heap.get_wrap_body(func)?;
      let hide_wrap = self.heap.new_wrap(hide)?;
      self.push_code(hide_wrap)?;
      self.push_code(func_body)?;
    } else if self.heap.is_bind(object)? {
      let func = self.pop_data()?;
      let show = self.pop_data()?;
      assert(self.heap.is_wrap(func))?;
      let func_body = self.heap.get_wrap_body(func)?;
      let pair = self.heap.new_pair(show, func_body)?;
      let wrap = self.heap.new_wrap(pair)?;
      self.push_data(wrap)?;
    } else if self.heap.is_copy(object)? {
      let copy = self.pop_data()?;
      self.push_data(copy)?;
      self.push_data(copy)?;
    } else if self.heap.is_drop(object)? {
      self.pop_data()?;
    } else if self.heap.is_shift(object)? {
      let handler = self.pop_data()?;
      let continuation = self.jump_code()?;
      self.push_data(continuation)?;
      self.push_code(handler)?;
    } else if self.heap.is_reset(object)? {
      //
    } else if self.heap.is_nil(object)? {
      //
    } else {
      return Err(Error::Stub);
    }
    return Ok(());
  }
}
