use itertools::Either;
use ocean_macros::New;
use crate::ocean::frontend::ast::node::*;
use crate::ocean::frontend::ast::typenode::*;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

#[derive(Clone, Debug)]
pub enum AstSymbol {
  StatementList(Vec<StatementNode>),
  StatementData(Vec<StatementNodeData>),
  OptStatement(Option<Statement>),

  Annotation(Annotation),
  AnnotationArguments(Vec<AnnotationArgument>),

  UsingPathEntries(Vec<UsingPathEntry>),
  CompoundStatement(CompoundStatement),
  Branch(Branch),
  ElseBranch(ElseBranch),
  MatchCases(Vec<MatchCase>),
  PackMembers(Vec<PackMember>),
  UnionMembers(Vec<UnionMember>),
  UnionSubTypes(UnionSubTypes),
  UnionSubTypeEntries(Vec<UnionSubTypeEntry>),
  InterfaceMembers(Vec<InterfaceEntry>),

  FunctionParams(Vec<FunctionParam>),
  FunctionReturns(Vec<FunctionReturn>),

  InterfaceDeclaration(Option<InterfaceDeclaration>),
  InterfaceImpls(Vec<InterfaceImplDeclaration>),

  Identifier(Identifier),
  Type(Type),
  TypeTupleArguments(Vec<TupleArgument>),
  TypeArguments(Vec<TypeArgument>),
  TypeParameters(TypeParameters),
  FunctionTypeArguments(Vec<FunctionTypeArgument>),

  Expression(ExpressionNode),

  Token(Token<TokenType>),
  OptToken(Option<Token<TokenType>>),
  ExpressionTokenList(Vec<Either<Token<TokenType>, AstNodeExpression>>),
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
