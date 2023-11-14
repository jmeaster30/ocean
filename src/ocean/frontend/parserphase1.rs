use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::astsymbolstack::*;
use crate::ocean::frontend::parsestatestack::*;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

pub fn parse_phase_one(tokens: &Vec<Token<TokenType>>) -> Program {

  let mut parser_state_stack = ParseStateStack::new();
  let mut ast_stack = AstSymbolStack::new();
  let mut token_index = 0;

  parser_state_stack.push(ParseState::StatementList);
  ast_stack.push(AstSymbol::StatementList(Vec::new()));

  loop {
    if token_index >= tokens.len() {
      break;
    }

    let current_token = &tokens[token_index];
    let current_state = parser_state_stack.current_state();
    let current_ast_symbol = ast_stack.peek();

    match (current_state, current_ast_symbol, &current_token.token_type) {
      (Some(ParseState::StatementList), Some(AstSymbol::StatementList(_)), TokenType::EndOfInput) => {
        break;
      }
      (Some(ParseState::StatementList), _, _) => {
        parser_state_stack.push(ParseState::PreStatement);
        ast_stack.push(AstSymbol::StatementData(Vec::new()));
      }

      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(mut statement_data)), TokenType::Annotation) => {
        ast_stack.pop();
        statement_data.push(StatementNodeData::Annotation(Annotation::new(current_token.clone())));
        ast_stack.push(AstSymbol::StatementData(statement_data));
        token_index += 1;
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(mut statement_data)), TokenType::Comment) => {
        ast_stack.pop();
        statement_data.push(StatementNodeData::Comment(Comment::new(current_token.clone())));
        ast_stack.push(AstSymbol::StatementData(statement_data));
        token_index += 1;
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(_)), TokenType::Newline) => {
        token_index += 1;
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(_)), _) => {
        parser_state_stack.goto(ParseState::Statement);
      }
      (Some(ParseState::PreStatement), _, _) => {
        panic!("Invalid parse state")
      }

      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::EndOfInput) => {
        ast_stack.push(AstSymbol::OptStatement(None));
        parser_state_stack.goto(ParseState::StatementFinalize);
      }
      (Some(ParseState::Statement), Some(AstSymbol::OptStatement(_)), _) => {
        parser_state_stack.goto(ParseState::StatementFinalize);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Using) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::UsingPathIdentifier);
      }

      (Some(ParseState::StatementFinalize), Some(AstSymbol::OptStatement(optional_statement)), _) => {
        ast_stack.pop();
        let statement_data = ast_stack.pop_panic().unwrap();
        let statements = ast_stack.pop_panic().unwrap();
        match (statement_data, statements) {
          (AstSymbol::StatementData(data), AstSymbol::StatementList(mut statements)) => {
            statements.push(StatementNode::new(data, optional_statement));
            ast_stack.push(AstSymbol::StatementList(statements));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::StatementFinalize), _, _) => panic!("Invalid state :("),
      (a, b, c) => panic!("Unexpected state {:?} {:?} {:?}", a, b, c),
    }
  }

  if ast_stack.size() != 1 {
    panic!("Too many things in the ast stack");
  }

  match ast_stack.pop_panic() {
    Some(AstSymbol::StatementList(statements)) => Program { statements },
    _ => panic!("Unexpected ast stack symbol"),
  }


}
