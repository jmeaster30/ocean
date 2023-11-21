use itertools::Either;
use ocean_helpers::New;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

#[derive(Clone, Debug, New)]
pub struct Program {
  pub statements: Vec<StatementNode>,
}

#[derive(Clone, Debug, New)]
pub struct StatementNode {
  pub data: Vec<StatementNodeData>,
  pub statement: Option<Statement>,
}

#[derive(Clone, Debug)]
pub enum StatementNodeData {
  Comment(Comment),
  Annotation(Annotation)
}

#[derive(Clone, Debug, New)]
pub struct Comment {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Annotation {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug)]
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
  Using(Using),
  Expression(ExpressionNode),
}

#[derive(Clone, Debug, New)]
pub struct CompoundStatement {
  pub left_curly: Token<TokenType>,
  pub body: Vec<StatementNode>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct WhileLoop {
  pub while_token: Token<TokenType>,
  pub body: CompoundStatement,
}

#[derive(Clone, Debug, New)]
pub struct ForLoop {
  pub for_token: Token<TokenType>,
  pub iterator: ExpressionNode,
  pub in_token: Token<TokenType>,
  pub iterable: ExpressionNode,
  pub body: CompoundStatement,
}

#[derive(Clone, Debug, New)]
pub struct Loop {
  pub loop_token: Token<TokenType>,
  pub body: CompoundStatement,
}

#[derive(Clone, Debug, New)]
pub struct Branch {
  pub if_token: Token<TokenType>,
  pub condition: ExpressionNode,
  pub body: CompoundStatement,
  pub else_branch: Option<ElseBranch>
}

#[derive(Clone, Debug, New)]
pub struct ElseBranch {
  pub else_token: Token<TokenType>,
  pub body: Either<CompoundStatement, Box<Branch>>,
}

#[derive(Clone, Debug, New)]
pub struct Match {
  pub match_token: Token<TokenType>,
  pub expression: ExpressionNode,
  pub left_curly: Token<TokenType>,
  pub cases: Vec<MatchCase>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct MatchCase {
  pub pattern: ExpressionNode, // This should be a "Pattern" concept which is similar to an expression but with different bits
  pub arrow_token: Token<TokenType>,
  pub body: Either<ExpressionNode, CompoundStatement>,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Assignment {
  pub left_expression: Either<LetTarget, ExpressionNode>, // This expression node must result in 1 left expression
  pub equal_token: Token<TokenType>,
  pub right_expression: ExpressionNode,
}

#[derive(Clone, Debug, New)]
pub struct LetTarget {
  pub let_token: Token<TokenType>,
  pub identifier: Identifier,
}

#[derive(Clone, Debug, New)]
pub struct Identifier {
  pub identifier: Token<TokenType>,
  pub colon: Option<Token<TokenType>>,
  pub optional_type: Option<Type>,
}

#[derive(Clone, Debug)]
pub enum Type {
  Base(BaseType),
  Auto(AutoType),
  Lazy(LazyType),
  Ref(RefType),
  Mutable(MutType),
  Function(FunctionType),
  Sub(SubType),
  Array(ArrayType),
}

#[derive(Clone, Debug, New)]
pub struct BaseType {
  pub base_type: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct SubType {
  pub left_paren_token: Token<TokenType>,
  pub sub_type: Box<Type>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct AutoType {
  pub auto_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct LazyType {
  pub lazy_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct RefType {
  pub ref_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct MutType {
  pub mut_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct ArrayType {
  pub base_type: Box<Type>,
  pub left_square: Token<TokenType>,
  pub index_type: Option<Box<Type>>,
  pub right_square: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionType {
  pub function_token: Token<TokenType>,
  pub param_left_paren: Token<TokenType>,
  pub param_types: Vec<Type>,
  pub param_right_paren: Token<TokenType>,
  pub arrow_token: Token<TokenType>,
  pub result_left_paren: Option<Token<TokenType>>,
  pub result_types: Vec<Type>,
  pub result_right_paren: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Function {
  pub function_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
  pub param_left_paren: Token<TokenType>,
  pub params: Vec<Identifier>,
  pub param_right_paren: Token<TokenType>,
  pub arrow_token: Token<TokenType>,
  pub result_left_paren: Token<TokenType>,
  pub results: Vec<FunctionReturn>,
  pub result_right_paren: Token<TokenType>,
  pub compound_statement: Option<CompoundStatement>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionReturn {
  pub identifier: Identifier,
  pub equal_token: Option<Token<TokenType>>,
  pub expression: Option<ExpressionNode>,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Pack {
  pub pack_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
  pub left_curly: Token<TokenType>,
  pub members: Vec<PackMember>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct PackMember {
  pub identifier: Identifier,
  pub comma_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Union {
  pub union_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
  pub left_curly: Token<TokenType>,
  pub members: Vec<UnionMember>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct UnionMember {
  pub identifier: Token<TokenType>,
  pub sub_type: Option<UnionSubTypes>,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct UnionSubTypes {
  pub left_paren: Token<TokenType>,
  pub types: Vec<UnionSubTypeEntry>,
  pub right_paren: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct UnionSubTypeEntry {
  pub types: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Return {
  pub return_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Break {
  pub break_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Continue {
  pub continue_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Using {
  pub using_token: Token<TokenType>,
  pub path: Vec<UsingPathEntry>,
}

#[derive(Clone, Debug, New)]
pub struct UsingPathEntry {
  pub identifier: Token<TokenType>,
  pub dot_token: Option<Token<TokenType>>,
}

// This may result in multiple sub expressions
#[derive(Clone, Debug, New)]
pub struct ExpressionNode {
  pub tokens: Vec<Token<TokenType>>,
}
