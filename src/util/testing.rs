#![cfg(test)]

use std::fmt::Debug;
use crate::util::token::Token;

pub fn assert_token_eq<T: PartialEq + Debug>(actual_token: &Token<T>, expected_lexeme: &str, expected_token_type: T, expected_offset: (usize, usize), expected_line: (usize, usize), expected_column: (usize, usize)) {
  assert_eq!(actual_token.lexeme, expected_lexeme, "Actual lexeme '{}' is different from expected lexeme '{}'", actual_token.lexeme, expected_lexeme);
  assert_eq!(actual_token.token_type, expected_token_type, "Actual token type '{:?}' is different from expected token type '{:?}'", actual_token.token_type, expected_token_type);
  assert_eq!(actual_token.offset, expected_offset, "Actual offset '{:?}' is different from expected offset '{:?}'", actual_token.offset, expected_offset);
  // TODO assert_eq!(actual_token.line, expected_line, "Actual line '{:?}' is different from expected line '{:?}'", actual_token.line, expected_line);
  // TODO assert_eq!(actual_token.column, expected_column, "Actual column '{:?}' is different from expected column '{:?}'", actual_token.column, expected_column);
}