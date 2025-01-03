use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct AstNodeIndex {
  index: usize,
}

impl AstNodeIndex {
  pub fn at(index: usize) -> Self {
    Self { index }
  }
}

impl From<usize> for AstNodeIndex {
  fn from(index: usize) -> Self {
    AstNodeIndex::at(index)
  }
}

impl Into<usize> for AstNodeIndex {
  fn into(self) -> usize {
    self.index
  }
}

impl Add for AstNodeIndex {
  type Output = AstNodeIndex;
  fn add(self, other: Self) -> Self {
    Self {
      index: self.index + other.index
    }
  }
}

impl Add<usize> for AstNodeIndex {
  type Output = AstNodeIndex;
  fn add(self, other: usize) -> Self {
    Self {
      index: self.index + other
    }
  }
}

impl AddAssign<usize> for AstNodeIndex {
  fn add_assign(&mut self, other: usize) {
    *self = AstNodeIndex::at(self.index + other);
  }
}

impl Sub for AstNodeIndex {
  type Output = AstNodeIndex;
  fn sub(self, other: Self) -> Self {
    Self {
      index: self.index - other.index
    }
  }
}

impl Sub<usize> for AstNodeIndex {
  type Output = AstNodeIndex;
  fn sub(self, other: usize) -> Self {
    Self {
      index: self.index - other
    }
  }
}

