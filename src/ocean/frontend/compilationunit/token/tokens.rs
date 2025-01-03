use std::ops::{Index, IndexMut};
use std::slice;

use crate::util::token::Token;
use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;

#[derive(Clone, Debug)]
pub struct Tokens {
  tokens: Vec<Token<TokenType>>,
}

impl Tokens {
  pub fn new() -> Self {
    Self { tokens: Vec::new() }
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self { tokens: Vec::with_capacity(capacity) }
  }

  pub fn push(&mut self, token: Token<TokenType>) {
    self.tokens.push(token)
  }

  pub fn len(&self) -> TokenIndex {
    TokenIndex::at(self.tokens.len())
  }

  pub fn is_empty(&self) -> bool {
    self.tokens.is_empty()
  }

  pub fn is_not_empty(&self) -> bool {
    !self.tokens.is_empty()
  }
}

impl IntoIterator for Tokens {
  type Item = Token<TokenType>;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
    self.tokens.into_iter()
  }
}

impl<'a> IntoIterator for &'a Tokens {
  type Item = &'a Token<TokenType>;
  type IntoIter = slice::Iter<'a, Token<TokenType>>;

  fn into_iter(self) -> Self::IntoIter {
    self.tokens.iter()
  }
}

impl Index<TokenIndex> for Tokens {
  type Output = Token<TokenType>;
  fn index(&self, index: TokenIndex) -> &Self::Output {
    &self.tokens[index.to_usize()]
  }
}

impl IndexMut<TokenIndex> for Tokens {
  fn index_mut(&mut self, index: TokenIndex) -> &mut Token<TokenType> {
    &mut self.tokens[index.to_usize()]
  }
}
