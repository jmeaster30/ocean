use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

pub struct Program {
  pub statements: Vec<Statement>,
}

pub struct StatementNode {
  pub annotations: Vec<Annotation>,
  pub statement: Statement,
}

pub struct CompoundStatement {
  pub left_curly: Token<TokenType>,
  pub body: Vec<StatementNode>,
  pub right_curly: Token<TokenType>,
}

pub struct Annotation {
  pub token: Token<TokenType>,
}

pub enum Statement {
  WhileLoop(WhileLoop),
  ForLoop(ForLoop),
  Loop(Loop),
  Branch(Branch),
  Match(Match),
  Assignment(Assignment),
  Function(Function),
  Pack(Pack),
  Union(Union),
  Return(Return),
  Break(Break),
  Continue(Continue),
  Use(Use),
}

pub struct WhileLoop {
  pub while_token: Token<TokenType>,
  pub body: CompoundStatement,
}

pub struct ForLoop {
  pub for_token: Token<TokenType>,
  pub iterator: ExpressionNode,
  pub in_token: Token<TokenType>,
  pub iterable: ExpressionNode,
  pub body: CompoundStatement,
}

pub struct Loop {
  pub loop_token: Token<TokenType>,
  pub body: CompoundStatement,
}

pub struct Branch {
  pub if_token: Token<TokenType>,
  pub condition: ExpressionNode,
  pub body: CompoundStatement,
  pub else_branch: Option<ElseBranch>
}

pub struct ElseBranch {
  pub else_token: Token<TokenType>,
  pub body: Option<CompoundStatement>,
  pub branch: Option<Box<Branch>>,
}

pub struct Match {}

pub struct Assignment {}

pub struct Function {}

pub struct Pack {}

pub struct Union {}

pub struct Return {
  pub return_token: Token<TokenType>,
}

pub struct Break {
  pub break_token: Token<TokenType>,
}

pub struct Continue {
  pub continue_token: Token<TokenType>,
}

pub struct Use {
  pub use_token: Token<TokenType>,
  pub root_token: Token<TokenType>,
  pub path: Vec<(Token<TokenType>, Token<TokenType>)>, // will be (., id)
}

pub struct ExpressionNode {
  pub tokens: Vec<Token<TokenType>>,
}
