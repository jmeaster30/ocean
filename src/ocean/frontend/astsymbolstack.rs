use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

#[derive(Clone, Debug)]
pub enum  AstSymbol {
  StatementList(Vec<StatementNode>),
  StatementData(Vec<StatementNodeData>),
  OptStatement(Option<Statement>),

  UsingPathEntries(Vec<UsingPathEntry>),
  CompoundStatement(CompoundStatement),
  Branch(Branch),
  ElseBranch(ElseBranch),
  PackMembers(Vec<PackMember>),

  Identifier(Identifier),
  Type(Type),
  FunctionTypeArgument(FunctionTypeArgument),
  FunctionTypeArguments(Vec<FunctionTypeArgument>),

  Expression(ExpressionNode),

  Token(Token<TokenType>),
  OptToken(Option<Token<TokenType>>),
  ExpressionTokenList(Vec<Token<TokenType>>),
}

#[derive(Debug)]
pub struct AstSymbolStack {
  stack: Vec<AstSymbol>,
}

impl AstSymbolStack {
  pub fn new() -> Self {
    Self { stack: Vec::new() }
  }

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
      println!("{:?}", entry);
    }
  }
}