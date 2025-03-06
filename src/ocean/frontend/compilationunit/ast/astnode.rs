use ocean_macros::{AstNode, New};
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;

pub trait AstNodeTrait {
  fn get_token_index_range(&self) -> (TokenIndex, TokenIndex);
  fn set_start_index(&mut self, start_index: TokenIndex);
  fn set_end_index(&mut self, end_index: TokenIndex);
  fn get_parent_node(&self) -> Option<AstNodeIndex>;
  fn set_parent_node(&mut self, node_index: AstNodeIndex);
  fn get_sibling_node(&self) -> Option<AstNodeIndex>;
  fn set_sibling_node(&mut self, node_index: AstNodeIndex);
}

#[derive(Copy, Clone, Debug, New)]
pub struct AstNodeMetadata {
  pub start_token_index: TokenIndex,
  pub end_token_index: TokenIndex,
  pub parent_node: Option<AstNodeIndex>,
  pub sibling_node: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug)]
pub enum AstNode {
  Statement(Statement),
  Annotation(Annotation),
  AnnotationArgument(AnnotationArgument),
  ExpressionStatement(ExpressionStatement),
  CompoundStatement(CompoundStatement),
  WhileLoop(WhileLoop),
  ForLoop(ForLoop),
  Loop(Loop),
  Branch(Branch),
  ElseBranch(ElseBranch),
  Match(Match),
  MatchCase(MatchCase),
  Assignment(Assignment),
  LetTarget(LetTarget),
  Identifier(Identifier),
  Function(Function),
  FunctionParam(FunctionParam),
  FunctionReturn(FunctionParam),
  Pack(Pack),
  PackMember(PackMember),
  Union(Union),
  UnionMember(UnionMember),
  UnionSubTypes(UnionSubTypes),
  UnionSubTypeEntry(UnionSubTypeEntry),
  Interface(Interface),
  InterfaceEntry(InterfaceEntry),
  InterfaceDeclaration(InterfaceDeclaration),
  InterfaceImplDeclaration(InterfaceImplDeclaration),
  Return(Return),
  Break(Break),
  Continue(Continue),
  Using(Using),
  UsingPathEntry(UsingPathEntry),
  
  ExpressionNode(ExpressionNode),
  Literal(Literal),
  Operator(Operator),
  ArrayLiteral(ArrayLiteral),
  InterpolatedString(InterpolatedString),
  Cast(Cast),
  Variable(Variable),
  Call(Call),
  ArrayIndex(ArrayIndex),
  Argument(Argument),
  Tuple(Tuple),
  TupleMember(TupleMember),
  SubExpression(SubExpression),
  PrefixOperator(PrefixOperator),
  PostfixOperator(PostfixOperator),
  BinaryOperator(BinaryOperator),
  
  BaseType(BaseType),
  CustomType(CustomType),
  TupleType(TupleType),
  TupleArgument(TupleArgument),
  TypeParameters(TypeParameters),
  TypeArgument(TypeArgument),
  SubType(SubType),
  AutoType(AutoType),
  LazyType(LazyType),
  RefType(RefType),
  MutType(MutType),
  ArrayType(ArrayType),
  VariableType(VariableType),
  FunctionType(FunctionType),
}
