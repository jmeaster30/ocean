use std::ops::{Add, AddAssign, Sub};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct TokenIndex {
  index: usize,
}

impl TokenIndex {
  pub fn at(index: usize) -> Self {
    Self { index }
  }

  pub fn to_usize(&self) -> usize {
    self.index
  }
}

impl Add for TokenIndex {
  type Output = TokenIndex;
  fn add(self, other: Self) -> Self {
    Self {
      index: self.index + other.index
    }
  }
}

impl Add<usize> for TokenIndex {
  type Output = TokenIndex;
  fn add(self, other: usize) -> Self {
    Self {
      index: self.index + other
    }
  }
}

impl AddAssign<usize> for TokenIndex {
  fn add_assign(&mut self, other: usize) {
    *self = TokenIndex::at(self.index + other);
  }
}

impl Sub for TokenIndex {
  type Output = TokenIndex;
  fn sub(self, other: Self) -> Self {
    Self {
      index: self.index - other.index
    }
  }
}

impl Sub<usize> for TokenIndex {
  type Output = TokenIndex;
  fn sub(self, other: usize) -> Self {
    Self {
      index: self.index - other
    }
  }
}
