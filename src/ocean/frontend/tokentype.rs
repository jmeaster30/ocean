#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
  EndOfInput,
  Error,
  Newline,
  Comment,
  Annotation,
  String,
  InterpolatedString,
  Number,
  Identifier,
  True,
  False,
  Type,

  Pack,
  Union,
  Function,
  While,
  Loop,
  For,
  In,
  Match,
  If,
  Else,
  Continue,
  Break,
  Return,
  Using,
  As,
  Let,

  LeftParen,
  RightParen,
  LeftSquare,
  RightSquare,
  LeftCurly,
  RightCurly,
  Dot,
  Comma,
  Colon,
  Arrow,
  Symbol,
}
