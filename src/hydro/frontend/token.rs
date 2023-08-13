#[derive(Clone, Debug)]
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

impl Token {
  pub fn new(lexeme: String, token_type: TokenType, offset: (usize, usize), line: (usize, usize), column: (usize, usize)) -> Self {
    Self {
      lexeme,
      token_type,
      offset,
      line,
      column,
    }
  }
}