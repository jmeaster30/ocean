use std::fmt;
use std::fmt::{Debug, Display};
use crate::util::span::Spanned;

pub trait TokenTrait<TokenType: PartialEq + Debug> {
  fn is_token_type(&self, _: TokenType) -> bool;
  fn is_lexeme(&self, _: &str) -> bool;
}

#[derive(Clone)]
pub struct Token<TokenType> {
  pub lexeme: String,
  pub token_type: TokenType,
  pub offset: (usize, usize),
  pub line: (usize, usize),
  pub column: (usize, usize),
}

impl<TokenType: PartialEq + Debug> TokenTrait<TokenType> for Token<TokenType> {
  fn is_token_type(&self, value: TokenType) -> bool {
    self.token_type == value
  }

  fn is_lexeme(&self, value: &str) -> bool {
    self.lexeme == value
  }
}

impl<TokenType> Spanned for Token<TokenType> {
  fn get_span(&self) -> (usize, usize) {
    self.offset
  }
}

impl<TokenType> Token<TokenType> {
  pub fn new(lexeme: String, token_type: TokenType, offset: (usize, usize), line: (usize, usize), column: (usize, usize)) -> Self {
    Self { lexeme, token_type, offset, line, column }
  }
}

impl<TokenType: Debug> Display for Token<TokenType> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "<[{:?}] '{}' {} {}>",
      self.token_type, self.lexeme.escape_default(), self.offset.0, self.offset.1
    )
  }
}

impl<TokenType: Debug> Debug for Token<TokenType> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

