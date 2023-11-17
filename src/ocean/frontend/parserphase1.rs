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
        ast_stack.push(AstSymbol::UsingPathEntries(Vec::new()));
        token_index += 1;
        parser_state_stack.goto(ParseState::UsingPathIdentifier);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Let) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::LetAssignment);
        parser_state_stack.push(ParseState::IdentifierStart);
      }


      //<editor-fold desc="> Using Statement">
      (Some(ParseState::UsingPathIdentifier), Some(AstSymbol::UsingPathEntries(_)), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.push(ParseState::UsingPathOptionalDot);
      }
      (Some(ParseState::UsingPathIdentifier), Some(AstSymbol::UsingPathEntries(entries)), TokenType::Newline) |
      (Some(ParseState::UsingPathIdentifier), Some(AstSymbol::UsingPathEntries(entries)), TokenType::EndOfInput)=> {
        ast_stack.pop();
        let using_token = ast_stack.pop_panic();
        match using_token {
          AstSymbol::Token(using_token) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Using(Using::new(using_token, entries.clone())))));
            parser_state_stack.goto(ParseState::StatementFinalize);
          }
          _ => panic!("Invalid state")
        }
      }
      (Some(ParseState::UsingPathOptionalDot), Some(AstSymbol::Token(identifier)), TokenType::Dot) => {
        ast_stack.pop();
        let path_entries = ast_stack.pop_panic();
        match path_entries {
          AstSymbol::UsingPathEntries(mut entries) => {
            entries.push(UsingPathEntry::new(identifier, Some(current_token.clone())));
            ast_stack.push(AstSymbol::UsingPathEntries(entries));
            token_index += 1;
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state")
        }
      }
      (Some(ParseState::UsingPathOptionalDot), Some(AstSymbol::Token(identifier)), _) => {
        ast_stack.pop();
        let path_entries = ast_stack.pop_panic();
        match path_entries {
          AstSymbol::UsingPathEntries(mut entries) => {
            entries.push(UsingPathEntry::new(identifier, None));
            ast_stack.push(AstSymbol::UsingPathEntries(entries));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state")
        }
      }
      //</editor-fold>

      (Some(ParseState::IdentifierStart), _, TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::IdentifierOptionalColon);
      }
      (Some(ParseState::IdentifierOptionalColon), Some(AstSymbol::Token(_)), TokenType::Colon) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index += 1;
        parser_state_stack.goto(ParseState::IdentifierEnd);
        parser_state_stack.push(ParseState::Type)
      }
      (Some(ParseState::IdentifierOptionalColon), Some(AstSymbol::Token(identifier)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Identifier(Identifier::new(identifier, None, None)));
        parser_state_stack.pop();
      }
      (Some(ParseState::IdentifierEnd), Some(AstSymbol::Type(identifier_type)), _) => {
        ast_stack.pop();
        let opt_colon_sym = ast_stack.pop_panic();
        let identifier_sym = ast_stack.pop_panic();
        match (identifier_sym, opt_colon_sym) {
          (AstSymbol::Token(identifier), AstSymbol::OptToken(opt_colon)) => {
            ast_stack.push(AstSymbol::Identifier(Identifier::new(identifier, opt_colon, Some(identifier_type))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }

      (Some(ParseState::StatementFinalize), Some(AstSymbol::OptStatement(optional_statement)), _) => {
        ast_stack.pop();
        let statement_data = ast_stack.pop_panic();
        let statements = ast_stack.pop_panic();
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
      (a, b, c) => {
        ast_stack.print();
        panic!("Unexpected state {:?} {:?} {:?}", a, b, c)
      },
    }
  }

  if ast_stack.size() != 1 {
    panic!("Too many things in the ast stack");
  }

  match ast_stack.pop_panic() {
    AstSymbol::StatementList(statements) => Program { statements },
    _ => panic!("Unexpected ast stack symbol"),
  }


}

fn consume_newline(tokens: &Vec<Token<TokenType>>, current_index: usize) -> usize {
  let mut result = current_index;
  while tokens[result].token_type == TokenType::Newline {
    result += 1;
  }
  result
}
