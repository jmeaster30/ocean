use crate::util::tokentrait::TokenTrait;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
  Error,
  Comment,
  Identifier,
  Number,

  Type,
  FunctionPointer,
  VariableRef,
  IndexRef,

  Main,
  Using,
  Module,
  Function,
  Body,
  Layout,
  Array,
  This,

  Alloc,
  Push,
  Pop,
  Duplicate,
  Swap,
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  LeftShift,
  RightShift,
  BitwiseAnd,
  BitwiseOr,
  BitwiseXor,
  BitwiseNot,
  And,
  Or,
  Xor,
  Not,
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanEqual,
  GreaterThanEqual,
  Jump,
  Branch,
  Call,
  Return,
  Load,
  Store,
  Index,

  True,
  False,
}

#[derive(Clone, Debug)]
pub struct Token {
  pub lexeme: String,
  pub token_type: TokenType,
  pub offset: (usize, usize),
  pub line: (usize, usize),
  pub column: (usize, usize),
}

impl TokenTrait<TokenType> for Token {
  fn is_token_type(&self, value: TokenType) -> bool {
    self.token_type == value
  }

  fn is_lexeme(&self, value: &str) -> bool {
    self.lexeme == value
  }
}

impl Token {
  pub fn new(
    lexeme: String,
    token_type: TokenType,
    offset: (usize, usize),
    line: (usize, usize),
    column: (usize, usize),
  ) -> Self {
    Self {
      lexeme,
      token_type,
      offset,
      line,
      column,
    }
  }
}
