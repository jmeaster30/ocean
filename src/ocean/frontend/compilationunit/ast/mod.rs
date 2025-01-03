pub mod astnodeindex;
pub mod traits;
pub mod nodestructs;
pub mod astnode;

use std::ops::{Index, IndexMut};
use std::slice;
use astnode::AstNode;
use astnodeindex::AstNodeIndex;

#[derive(Clone, Debug)]
pub struct AstNodes {
  nodes: Vec<AstNode>,
}

impl AstNodes {
  pub fn new() -> Self {
    Self { nodes: Vec::new() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { nodes: Vec::with_capacity(capacity) }
  }

  pub fn push(&mut self, node: AstNode) {
    self.nodes.push(node)
  }

  pub fn len(&self) -> AstNodeIndex {
    AstNodeIndex::at(self.nodes.len())
  }

  pub fn is_empty(&self) -> bool {
    self.nodes.is_empty()
  }

  pub fn is_not_empty(&self) -> bool {
    !self.nodes.is_empty()
  }
}

impl IntoIterator for AstNodes {
  type Item = AstNode;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.nodes.into_iter()
  }
}

impl<'a> IntoIterator for &'a AstNodes {
  type Item = &'a AstNode;
  type IntoIter = slice::Iter<'a, AstNode>;

  fn into_iter(self) -> Self::IntoIter {
    self.nodes.iter()
  }
}

impl Index<AstNodeIndex> for AstNodes {
  type Output = AstNode;
  fn index(&self, index: AstNodeIndex) -> &Self::Output {
    let idx: usize = index.into();
    &self.nodes[idx]
  }
}

impl IndexMut<AstNodeIndex> for AstNodes {
  fn index_mut(&mut self, index: AstNodeIndex) -> &mut AstNode {
    let idx: usize = index.into();
    &mut self.nodes[idx]
  }
}
