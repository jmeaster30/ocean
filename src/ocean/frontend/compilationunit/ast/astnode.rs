use ocean_macros::{AstNode, New};
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;

pub trait AstNodeTrait {
  fn get_token_index_range(&self) -> (TokenIndex, TokenIndex);
  fn get_sibling_node(&self) -> Option<AstNodeIndex>;
}

#[derive(Copy, Clone, Debug, New)]
pub struct AstNodeMetadata {
  pub start_token_index: TokenIndex,
  pub end_token_index: TokenIndex,
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
  StringLiteral(StringLiteral),
  ArrayLiteral(ArrayLiteral),
  Number(Number),
  Boolean(Boolean),
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
  TernaryOperator(TernaryOperator),
  
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
  FunctionTypeArgument(FunctionTypeArgument),
}

/*
impl AstNode {
  pub fn get_token_index_range(&self) -> (TokenIndex, TokenIndex) {
    match self {
      AstNode::Statement(x) => x.get_token_index_range(),
      AstNode::Annotation(x) => x.get_token_index_range(),
      AstNode::AnnotationArgument(x) => x.get_token_index_range(),
      AstNode::ExpressionStatement(x) => x.get_token_index_range(),
      AstNode::CompoundStatement(x) => x.get_token_index_range(),
      AstNode::WhileLoop(x) => x.get_token_index_range(),
      AstNode::ForLoop(x) => x.get_token_index_range(),
      AstNode::Loop(x) => x.get_token_index_range(),
      AstNode::Branch(x) => x.get_token_index_range(),
      AstNode::ElseBranch(x) => x.get_token_index_range(),
      AstNode::Match(x) => x.get_token_index_range(),
      AstNode::MatchCase(x) => x.get_token_index_range(),
      AstNode::Assignment(x) => x.get_token_index_range(),
      AstNode::LetTarget(x) => x.get_token_index_range(),
      AstNode::Identifier(x) => x.get_token_index_range(),
      AstNode::Function(x) => x.get_token_index_range(),
      AstNode::FunctionParam(x) => x.get_token_index_range(),
      AstNode::FunctionReturn(x) => x.get_token_index_range(),
      AstNode::Pack(x) => x.get_token_index_range(),
      AstNode::PackMember(x) => x.get_token_index_range(),
      AstNode::Union(x) => x.get_token_index_range(),
      AstNode::UnionMember(x) => x.get_token_index_range(),
      AstNode::UnionSubTypes(x) => x.get_token_index_range(),
      AstNode::UnionSubTypeEntry(x) => x.get_token_index_range(),
      AstNode::Interface(x) => x.get_token_index_range(),
      AstNode::InterfaceEntry(x) => x.get_token_index_range(),
      AstNode::InterfaceDeclaration(x) => x.get_token_index_range(),
      AstNode::InterfaceImplDeclaration(x) => x.get_token_index_range(),
      AstNode::Return(x) => x.get_token_index_range(),
      AstNode::Break(x) => x.get_token_index_range(),
      AstNode::Continue(x) => x.get_token_index_range(),
      AstNode::Using(x) => x.get_token_index_range(),
      AstNode::UsingPathEntry(x) => x.get_token_index_range(),
      AstNode::ExpressionNode(x) => x.get_token_index_range(),
      AstNode::StringLiteral(x) => x.get_token_index_range(),
      AstNode::ArrayLiteral(x) => x.get_token_index_range(),
      AstNode::Number(x) => x.get_token_index_range(),
      AstNode::Boolean(x) => x.get_token_index_range(),
      AstNode::InterpolatedString(x) => x.get_token_index_range(),
      AstNode::Cast(x) => x.get_token_index_range(),
      AstNode::Variable(x) => x.get_token_index_range(),
      AstNode::Call(x) => x.get_token_index_range(),
      AstNode::ArrayIndex(x) => x.get_token_index_range(),
      AstNode::Argument(x) => x.get_token_index_range(),
      AstNode::Tuple(x) => x.get_token_index_range(),
      AstNode::TupleMember(x) => x.get_token_index_range(),
      AstNode::SubExpression(x) => x.get_token_index_range(),
      AstNode::PrefixOperator(x) => x.get_token_index_range(),
      AstNode::PostfixOperator(x) => x.get_token_index_range(),
      AstNode::BinaryOperator(x) => x.get_token_index_range(),
      AstNode::TernaryOperator(x) => x.get_token_index_range(),
      AstNode::BaseType(x) => x.get_token_index_range(),
      AstNode::CustomType(x) => x.get_token_index_range(),
      AstNode::TupleType(x) => x.get_token_index_range(),
      AstNode::TupleArgument(x) => x.get_token_index_range(),
      AstNode::TypeParameters(x) => x.get_token_index_range(),
      AstNode::TypeArgument(x) => x.get_token_index_range(),
      AstNode::SubType(x) => x.get_token_index_range(),
      AstNode::AutoType(x) => x.get_token_index_range(),
      AstNode::LazyType(x) => x.get_token_index_range(),
      AstNode::RefType(x) => x.get_token_index_range(),
      AstNode::MutType(x) => x.get_token_index_range(),
      AstNode::ArrayType(x) => x.get_token_index_range(),
      AstNode::VariableType(x) => x.get_token_index_range(),
      AstNode::FunctionType(x) => x.get_token_index_range(),
      AstNode::FunctionTypeArgument(x) => x.get_token_index_range(),
    }
  }

  pub fn get_sibling_node(&self) -> Option<AstNodeIndex> {
    match self {
      AstNode::Statement(x) => x.get_sibling_node(),
      AstNode::Annotation(x) => x.get_sibling_node(),
      AstNode::AnnotationArgument(x) => x.get_sibling_node(),
      AstNode::ExpressionStatement(x) => x.get_sibling_node(),
      AstNode::CompoundStatement(x) => x.get_sibling_node(),
      AstNode::WhileLoop(x) => x.get_sibling_node(),
      AstNode::ForLoop(x) => x.get_sibling_node(),
      AstNode::Loop(x) => x.get_sibling_node(),
      AstNode::Branch(x) => x.get_sibling_node(),
      AstNode::ElseBranch(x) => x.get_sibling_node(),
      AstNode::Match(x) => x.get_sibling_node(),
      AstNode::MatchCase(x) => x.get_sibling_node(),
      AstNode::Assignment(x) => x.get_sibling_node(),
      AstNode::LetTarget(x) => x.get_sibling_node(),
      AstNode::Identifier(x) => x.get_sibling_node(),
      AstNode::Function(x) => x.get_sibling_node(),
      AstNode::FunctionParam(x) => x.get_sibling_node(),
      AstNode::FunctionReturn(x) => x.get_sibling_node(),
      AstNode::Pack(x) => x.get_sibling_node(),
      AstNode::PackMember(x) => x.get_sibling_node(),
      AstNode::Union(x) => x.get_sibling_node(),
      AstNode::UnionMember(x) => x.get_sibling_node(),
      AstNode::UnionSubTypes(x) => x.get_sibling_node(),
      AstNode::UnionSubTypeEntry(x) => x.get_sibling_node(),
      AstNode::Interface(x) => x.get_sibling_node(),
      AstNode::InterfaceEntry(x) => x.get_sibling_node(),
      AstNode::InterfaceDeclaration(x) => x.get_sibling_node(),
      AstNode::InterfaceImplDeclaration(x) => x.get_sibling_node(),
      AstNode::Return(x) => x.get_sibling_node(),
      AstNode::Break(x) => x.get_sibling_node(),
      AstNode::Continue(x) => x.get_sibling_node(),
      AstNode::Using(x) => x.get_sibling_node(),
      AstNode::UsingPathEntry(x) => x.get_sibling_node(),
      AstNode::ExpressionNode(x) => x.get_sibling_node(),
      AstNode::StringLiteral(x) => x.get_sibling_node(),
      AstNode::ArrayLiteral(x) => x.get_sibling_node(),
      AstNode::Number(x) => x.get_sibling_node(),
      AstNode::Boolean(x) => x.get_sibling_node(),
      AstNode::InterpolatedString(x) => x.get_sibling_node(),
      AstNode::Cast(x) => x.get_sibling_node(),
      AstNode::Variable(x) => x.get_sibling_node(),
      AstNode::Call(x) => x.get_sibling_node(),
      AstNode::ArrayIndex(x) => x.get_sibling_node(),
      AstNode::Argument(x) => x.get_sibling_node(),
      AstNode::Tuple(x) => x.get_sibling_node(),
      AstNode::TupleMember(x) => x.get_sibling_node(),
      AstNode::SubExpression(x) => x.get_sibling_node(),
      AstNode::PrefixOperator(x) => x.get_sibling_node(),
      AstNode::PostfixOperator(x) => x.get_sibling_node(),
      AstNode::BinaryOperator(x) => x.get_sibling_node(),
      AstNode::TernaryOperator(x) => x.get_sibling_node(),
      AstNode::BaseType(x) => x.get_sibling_node(),
      AstNode::CustomType(x) => x.get_sibling_node(),
      AstNode::TupleType(x) => x.get_sibling_node(),
      AstNode::TupleArgument(x) => x.get_sibling_node(),
      AstNode::TypeParameters(x) => x.get_sibling_node(),
      AstNode::TypeArgument(x) => x.get_sibling_node(),
      AstNode::SubType(x) => x.get_sibling_node(),
      AstNode::AutoType(x) => x.get_sibling_node(),
      AstNode::LazyType(x) => x.get_sibling_node(),
      AstNode::RefType(x) => x.get_sibling_node(),
      AstNode::MutType(x) => x.get_sibling_node(),
      AstNode::ArrayType(x) => x.get_sibling_node(),
      AstNode::VariableType(x) => x.get_sibling_node(),
      AstNode::FunctionType(x) => x.get_sibling_node(),
      AstNode::FunctionTypeArgument(x) => x.get_sibling_node(),
    }
  }
}
*/

