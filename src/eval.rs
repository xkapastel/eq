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

fn fetch(
  code: &mut Vec<Pointer>,
  heap: &mut Heap) -> Result<Pointer> {
  loop {
    let object = code.pop().ok_or(Error::Bug)?;
    if heap.is_sequence(object)? {
      let head = heap.get_sequence_head(object)?;
      let tail = heap.get_sequence_tail(object)?;
      code.push(tail);
      code.push(head);
    } else {
      return Ok(object);
    }
  }
}

fn jump(
  code: &mut Vec<Pointer>,
  heap: &mut Heap) -> Result<Pointer> {
  let mut buf = Vec::new();
  loop {
    let object = fetch(code, heap)?;
    if heap.is_reset(object)? {
      code.push(object);
      let mut xs = heap.new_id()?;
      for object in buf.iter().rev() {
        xs = heap.new_sequence(*object, xs)?;
      }
      xs = heap.new_block(xs)?;
      return Ok(xs);
    } else {
      buf.push(object);
    }
  }
}

fn freeze(
  code: Pointer,
  data: &mut Vec<Pointer>,
  kill: &mut Vec<Pointer>) {
  kill.append(data);
  kill.push(code);
}

/// Evaluate the given string of Eq code.
pub fn eval(
  source: &str,
  target: &mut String,
  space: usize,
  mut time: usize) -> Result<()> {
  let mut heap = Heap::with_capacity(space);
  let root = heap.parse(source)?;
  let mut code = vec![root];
  let mut data = vec![];
  let mut kill = vec![];
  // Technically the "kill" trick doesn't work anymore:
  // if dead code later expands to something containing a
  // shift, then the meaning of the code changes. For now
  // I'll just ignore this but I'll need to think about
  // how to address this later.
  while time > 0 && !code.is_empty() {
    time -= 1;
    let object = fetch(&mut code, &mut heap)?;
    if heap.is_number(object)? {
      data.push(object);
    } else if heap.is_block(object)? {
      data.push(object);
    } else if heap.is_app(object)? {
      if data.len() < 2 {
        freeze(object, &mut data, &mut kill);
        continue;
      }
      let func = data.pop().ok_or(Error::Underflow)?;
      let hide = data.pop().ok_or(Error::Underflow)?;
      assert(heap.is_block(func))?;
      let func_body = heap.get_block_body(func)?;
      code.push(hide);
      code.push(func_body);
    } else if heap.is_bind(object)? {
      if data.len() < 2 {
        freeze(object, &mut data, &mut kill);
        continue;
      }
      let func = data.pop().ok_or(Error::Underflow)?;
      let show = data.pop().ok_or(Error::Underflow)?;
      assert(heap.is_block(func))?;
      let func_body = heap.get_block_body(func)?;
      let sequence = heap.new_sequence(show, func_body)?;
      let block = heap.new_block(sequence)?;
      data.push(block);
    } else if heap.is_copy(object)? {
      if data.is_empty() {
        freeze(object, &mut data, &mut kill);
        continue;
      }
      let copy = data.last().ok_or(Error::Underflow)?;
      data.push(*copy);
    } else if heap.is_drop(object)? {
      if data.is_empty() {
        freeze(object, &mut data, &mut kill);
        continue;
      }
      data.pop().ok_or(Error::Underflow)?;
    } else if heap.is_shift(object)? {
      // We should crash on underflow here due to effects (?)
      let callback = data.pop().ok_or(Error::Underflow)?;
      let callback_body = heap.get_block_body(callback)?;
      let continuation = jump(&mut code, &mut heap)?;
      code.push(callback_body);
      data.push(continuation);
    } else if heap.is_reset(object)? {
      // If there's dead code, we can't delete stuff.
      if !kill.is_empty() {
        freeze(object, &mut data, &mut kill);
      }
    } else if heap.is_id(object)? {
      //
    } else {
      freeze(object, &mut data, &mut kill);
    }
  }
  let mut xs = heap.new_id()?;
  for object in code.iter() {
    xs = heap.new_sequence(*object, xs)?;
  }
  for object in data.iter().rev() {
    xs = heap.new_sequence(*object, xs)?;
  }
  for object in kill.iter().rev() {
    xs = heap.new_sequence(*object, xs)?;
  }
  heap.quote(xs, target)?;
  return Ok(());
}
