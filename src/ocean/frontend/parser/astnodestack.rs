use itertools::Either;
use ocean_macros::New;
use crate::ocean::frontend::compilationunit::ast::astnode::AstNode;
use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::ast::astnodes::AstNodes;
use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;
use crate::util::token::Token;

#[derive(Debug, New)]
pub struct AstNodeStack {
  #[default(Vec::new())]
  stack: Vec<AstNodeIndex>,
}

impl AstNodeStack {
  pub fn peek(&self) -> Option<AstNodeIndex> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  pub fn push(&mut self, index: AstNodeIndex) {
    self.stack.push(index);
  }

  pub fn pop(&mut self) -> Option<AstNodeIndex> {
    if !self.stack.is_empty() {
      self.stack.pop()
    } else {
      None
    }
  }

  pub fn pop_panic(&mut self) -> AstNodeIndex {
    if self.stack.is_empty() {
      panic!("Ah crap I tried to pop an empty stack :(");
    }
    self.stack.pop().unwrap()
  }

  pub fn size(&self) -> usize {
    self.stack.len()
  }

  pub fn is_empty(&self) -> bool {
    self.stack.len() == 0
  }

  pub fn print(&self, nodes: &AstNodes) {
    println!("AST STACK:   ");
    for entry in &self.stack {
      println!("{:#?} > {:#?}", entry, nodes[*entry]);
    }
  }
}
