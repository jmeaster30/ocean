use itertools::Either;
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

    //println!("{} > {} - {:?}", token_index, current_token, current_ast_symbol);

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
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::LetAssignment);
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

      //<editor-fold desc="> Identifier">
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
      //</editor-fold>

      //<editor-fold desc="> Type">
      (Some(ParseState::Type), Some(_), TokenType::Type) => {
        parser_state_stack.pop();
        ast_stack.push(AstSymbol::Type(Type::Base(BaseType::new(current_token.clone()))));
        token_index += 1;
      }
      (Some(ParseState::Type), Some(_), TokenType::RightSquare) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Type), Some(_), TokenType::TypePrefix) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeArray);
        match current_token.lexeme.as_str() {
          "auto" => {
            parser_state_stack.push(ParseState::TypeAuto);
            parser_state_stack.push(ParseState::TypeIdentifier);
          },
          "lazy" => {
            parser_state_stack.push(ParseState::TypeLazy);
            parser_state_stack.push(ParseState::Type);
          },
          "ref" => {
            parser_state_stack.push(ParseState::TypeRef);
            parser_state_stack.push(ParseState::Type);
          },
          "mut" => {
            parser_state_stack.push(ParseState::TypeMut);
            parser_state_stack.push(ParseState::Type);
          },
          _ => panic!("Unexpected type prefix"),
        }
      }
      (Some(ParseState::TypeIdentifier), Some(_), TokenType::Identifier) => {
        token_index += 1;
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeAuto), Some(AstSymbol::Token(identifier_token)), _) => {
        ast_stack.pop();
        let auto_token_sym = ast_stack.pop_panic();
        match auto_token_sym {
          AstSymbol::Token(auto_token) => {
            ast_stack.push(AstSymbol::Type(Type::Auto(AutoType::new(auto_token, identifier_token))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeLazy), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let lazy_token_sym = ast_stack.pop_panic();
        match lazy_token_sym {
          AstSymbol::Token(lazy_token) => {
            ast_stack.push(AstSymbol::Type(Type::Lazy(LazyType::new(lazy_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeRef), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let ref_token_sym = ast_stack.pop_panic();
        match ref_token_sym {
          AstSymbol::Token(ref_token) => {
            ast_stack.push(AstSymbol::Type(Type::Ref(RefType::new(ref_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeMut), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let mut_token_sym = ast_stack.pop_panic();
        match mut_token_sym {
          AstSymbol::Token(mut_token) => {
            ast_stack.push(AstSymbol::Type(Type::Mutable(MutType::new(mut_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArray), Some(_), TokenType::LeftSquare) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeArrayEnd);
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeArray), Some(_), _) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Type(subtype)), TokenType::RightSquare) => {
        ast_stack.pop();
        let left_square_sym = ast_stack.pop_panic();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index += 1;
        match (main_type_sym, left_square_sym) {
          (AstSymbol::Type(main_type), AstSymbol::Token(left_square)) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, Some(Box::new(subtype)), current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :(")
        }
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Token(left_square)), TokenType::RightSquare) => {
        ast_stack.pop();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index += 1;
        match main_type_sym {
          AstSymbol::Type(main_type) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, None, current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :(")
        }
      }
      //</editor-fold>

      (Some(ParseState::LetAssignment), Some(AstSymbol::Identifier(_)), TokenType::Symbol) => {
        // TODO may need to determine if we have an assignment token here
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::LetAssignment), Some(AstSymbol::Expression(expression_node)), _) => {
        ast_stack.pop();
        let assignment_token_sym = ast_stack.pop_panic();
        let identifier_sym = ast_stack.pop_panic();
        let let_token_sym = ast_stack.pop_panic();
        match (&let_token_sym, &identifier_sym, &assignment_token_sym) {
          (AstSymbol::Token(let_token), AstSymbol::Identifier(identifier), AstSymbol::Token(assignment_token)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Assignment(Assignment::new(Either::Left(LetTarget::new(let_token.clone(), identifier.clone())), assignment_token.clone(), expression_node)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state :( {:?} {:?} {:?}", let_token_sym, identifier_sym, assignment_token_sym)
        }
      }

      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::LeftParen) => {
        parser_state_stack.push(ParseState::SubExpression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::RightParen) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Comment) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::String) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::InterpolatedString) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Number) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Identifier) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::True) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::False) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::As) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::LeftSquare) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::RightSquare) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Dot) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Comma) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Colon) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Symbol) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Newline) => {
        ast_stack.pop();
        token_list.push(current_token.clone());
        ast_stack.push(AstSymbol::ExpressionTokenList(token_list.clone()));
        token_index += 1;
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(token_list)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Expression(ExpressionNode::new(token_list)));
        parser_state_stack.pop();
      }
      (Some(ParseState::Expression), Some(_), _) => {
        ast_stack.push(AstSymbol::ExpressionTokenList(Vec::new()));
      }

      (Some(ParseState::SubExpression), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::ExpressionTokenList(vec![current_token.clone()]));
        parser_state_stack.push(ParseState::Expression);
        token_index += 1;
      }
      (Some(ParseState::SubExpression), Some(AstSymbol::ExpressionTokenList(mut sub_token_list)), TokenType::RightParen) => {
        ast_stack.pop();
        let token_list_sym = ast_stack.pop_panic();
        match token_list_sym {
          AstSymbol::ExpressionTokenList(mut token_list) => {
            sub_token_list.push(current_token.clone());
            token_list.append(&mut sub_token_list);
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
            token_index += 1;
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }

      (Some(ParseState::StatementFinalize), Some(AstSymbol::OptStatement(optional_statement)), _) => {
        ast_stack.pop();
        let statement_data = ast_stack.pop_panic();
        let statements = ast_stack.pop_panic();
        match (statement_data.clone(), statements.clone()) { // TODO remove these clones
          (AstSymbol::StatementData(data), AstSymbol::StatementList(mut statements)) => {
            statements.push(StatementNode::new(data, optional_statement));
            ast_stack.push(AstSymbol::StatementList(statements));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :( {:?} {:?}", statement_data, statements),
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
