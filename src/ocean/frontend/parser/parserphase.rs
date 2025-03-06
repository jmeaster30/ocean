use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::parser::astnodestack::*;
use crate::ocean::frontend::parser::parsestatestack::*;
use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::{tokens::Tokens, tokenindex::TokenIndex};
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;
use itertools::Either;
use ocean_macros::New;
use crate::ocean::frontend::compilationunit::ast::astnode::{AstNode, AstNodeTrait};
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::ast::astnodes::AstNodes;
use crate::util::token::Token;

#[derive(Debug, New)]
struct ParserState<'a> {
  tokens: &'a Tokens,
  #[default(TokenIndex::at(0))]
  current_token_index: TokenIndex,
  #[default(AstNodes::new())]
  ast_nodes: AstNodes,
  #[default(AstNodeStack::new())]
  ast_node_stack: AstNodeStack,
  #[default(Vec::new())]
  errors: Vec<Error>
}

impl<'a> ParserState<'a> {
  pub fn current_state(&'a self) -> (Option<&'a Token<TokenType>>, Option<&'a AstNode>) {
    let current_token = if self.current_token_index >= self.tokens.len() {
      None
    } else {
      Some(&self.tokens[self.current_token_index])
    };

    let current_node = if self.ast_node_stack.is_empty() {
      None
    } else {
      Some(&self.ast_nodes[self.ast_node_stack.peek().unwrap()])
    };

    (current_token, current_node)
  }

  pub fn consume_token(&mut self) {
    self.current_token_index += 1;
  }

  pub fn done(&self) -> bool {
    self.current_token_index >= self.tokens.len()
  }

  pub fn result(&self) -> Option<(AstNodes, Vec<Error>)> {
    if self.done() {
      Some((self.ast_nodes.clone(), self.errors.clone()))
    } else {
      None
    }
  }
}

pub fn parse(tokens: &Tokens) -> (AstNodes, Vec<Error>) {
  let mut parser_state = ParserState::new(tokens);

  while !parser_state.done() {
    parser_state.consume_token();
  }

  parser_state.result().unwrap()
}

fn get_node_file_offsets(tokens: &Tokens, ast_node: &AstNode) -> (usize, usize) {
  let (start_token_index, end_token_index) = ast_node.get_token_index_range();
  let (start_token_offset, _) = tokens[start_token_index].offset;
  let (_, end_token_offset) = tokens[end_token_index].offset;
  (start_token_offset, end_token_offset)
}
