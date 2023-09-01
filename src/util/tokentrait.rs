pub trait TokenTrait<TokenType> {
  fn is_token_type(&self, _: TokenType) -> bool;
  fn is_lexeme(&self, _: &str) -> bool;
}
