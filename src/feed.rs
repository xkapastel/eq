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

fn iter_nodes<'a, F>(
  node: &'a comrak::nodes::AstNode<'a>,
  func: &mut F) where F: FnMut(&'a comrak::nodes::AstNode<'a>) {
  func(node);
  for child in node.children() {
    iter_nodes(child, func);
  }
}

pub fn extract_code_blocks(src: &str) -> String {
  let arena = comrak::Arena::new();
  let options = comrak::ComrakOptions::default();
  let root = comrak::parse_document(&arena, src, &options);
  let mut blocks: Vec<String> = Vec::new();
  iter_nodes(root, &mut |node| {
    if let &comrak::nodes::NodeValue::CodeBlock(ref node) = &node.data.borrow().value {
      if "eq" == std::str::from_utf8(&node.info).unwrap() {
        let block = std::str::from_utf8(&node.literal).unwrap().to_string();
        blocks.push(block);
      }
    }
  });
  return blocks.join("\n");
}
