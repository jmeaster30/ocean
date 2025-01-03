use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::ocean::frontend::parser::astsymbolstack::*;
use crate::ocean::frontend::parser::parsestatestack::*;
use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::{tokens::Tokens, tokenindex::TokenIndex};
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;
use itertools::Either;
use crate::ocean::frontend::compilationunit::ast::astnode::{AstNode, AstNodeTrait};
use crate::ocean::frontend::compilationunit::ast::AstNodes;

pub fn parse(tokens: &Tokens) -> (AstNodes, Vec<Error>) {
  let mut ast_nodes = AstNodes::new();
  let mut errors = Vec::new();
  
  let mut current_token_index = TokenIndex::at(0);
  
  while current_token_index < tokens.len() {
    current_token_index += 1;
  }
  (ast_nodes, errors)
}

fn get_node_file_offsets(tokens: &Tokens, ast_node: &AstNode) -> (usize, usize) {
  let (start_token_index, end_token_index) = ast_node.get_token_index_range();
  let (start_token_offset, _) = tokens[start_token_index].offset;
  let (_, end_token_offset) = tokens[end_token_index].offset;
  (start_token_offset, end_token_offset)
}
