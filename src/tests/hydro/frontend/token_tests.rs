use crate::hydro::frontend::token::{Token, TokenType};
use crate::util::tokentrait::TokenTrait;

#[test]
fn test_token_is_lexeme() {
  let token = Token::new(
    "test".to_string(),
    TokenType::Identifier,
    (0, 0),
    (0, 0),
    (0, 0),
  );

  assert!(token.is_lexeme("test"));
}

#[test]
fn test_token_is_token_type_id_1() {
  let token = Token::new(
    "test".to_string(),
    TokenType::Identifier,
    (0, 0),
    (0, 0),
    (0, 0),
  );

  assert!(token.is_token_type(TokenType::Identifier));
}

#[test]
fn test_token_is_token_type_id_2() {
  let token = Token::new("main".to_string(), TokenType::Main, (0, 0), (0, 0), (0, 0));

  assert!(token.is_token_type(TokenType::Main));
}

#[test]
fn test_token_is_token_type_id_3() {
  let token = Token::new("push".to_string(), TokenType::Push, (0, 0), (0, 0), (0, 0));

  assert!(token.is_token_type(TokenType::Push));
}

#[test]
fn test_token_is_token_type_id_4() {
  let token = Token::new("true".to_string(), TokenType::True, (0, 0), (0, 0), (0, 0));

  assert!(token.is_token_type(TokenType::True));
}

#[test]
fn test_token_is_token_type_id_5() {
  let token = Token::new("true".to_string(), TokenType::True, (0, 0), (0, 0), (0, 0));

  assert!(!token.is_token_type(TokenType::Identifier));
}
