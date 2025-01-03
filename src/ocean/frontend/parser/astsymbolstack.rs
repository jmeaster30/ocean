use itertools::Either;
use ocean_macros::New;
use crate::ocean::frontend::compilationunit::ast::astnode::AstNode;
use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;
use crate::util::token::Token;

#[derive(Clone, Debug)]
pub enum AstSymbol {
  StatementData(AstNode),
  OptStatement(Option<AstNode>),

  Annotation(AstNode),
  AnnotationArguments(Vec<AstNode>),

  UsingPathEntries(AstNode),
  CompoundStatement(Vec<AstNode>),
  Branch(AstNode),
  ElseBranch(AstNode),
  MatchCases(Vec<AstNode>),
  PackMembers(AstNodeIndex, Option<AstNodeIndex>),
  UnionMembers(AstNodeIndex, Option<AstNodeIndex>),
  UnionSubTypes(AstNodeIndex),
  UnionSubTypeEntries(AstNodeIndex, Option<AstNodeIndex>),
  InterfaceMembers(AstNodeIndex, Option<AstNodeIndex>),

  FunctionParams(AstNodeIndex, Option<AstNodeIndex>),
  FunctionReturns(AstNodeIndex, Option<AstNodeIndex>),

  InterfaceDeclaration(AstNodeIndex, Option<AstNodeIndex>),
  InterfaceImpls(AstNodeIndex, Option<AstNodeIndex>),

  Identifier(AstNodeIndex),
  Type(AstNodeIndex),
  TypeTupleArguments(AstNodeIndex, Option<AstNodeIndex>),
  TypeArguments(AstNodeIndex, Option<AstNodeIndex>),
  TypeParameters(AstNodeIndex),
  FunctionTypeArguments(AstNodeIndex, Option<AstNodeIndex>),

  Expression(AstNodeIndex),

  Token(TokenIndex),
  OptToken(Option<TokenIndex>),
  //ExpressionTokenList(Vec<Either<Token<TokenType>, AstNodeExpression>>),
}

#[derive(Debug, New)]
pub struct AstSymbolStack {
  #[default(Vec::new())]
  stack: Vec<AstSymbol>,
}

impl AstSymbolStack {
  pub fn peek(&self) -> Option<AstSymbol> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn push(&mut self, symbol: AstSymbol) {
    self.stack.push(symbol);
  }

  pub fn pop(&mut self) -> Option<AstSymbol> {
    if !self.stack.is_empty() {
      self.stack.pop()
    } else {
      None
    }
  }

  pub fn pop_panic(&mut self) -> AstSymbol {
    if self.stack.is_empty() {
      panic!("Ah crap I tried to pop an empty stack :(");
    }
    self.stack.pop().unwrap()
  }

  pub fn size(&self) -> usize {
    self.stack.len()
  }

  pub fn print(&self) {
    println!("AST STACK:   ");
    for entry in &self.stack {
      println!("{:#?}", entry);
    }
  }
}
