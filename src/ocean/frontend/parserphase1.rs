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
      (Some(ParseState::StatementList), Some(AstSymbol::StatementList(_)), TokenType::RightCurly) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::StatementList), _, TokenType::Newline) => {
        token_index += 1;
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
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::BranchEndStatement);
        parser_state_stack.push(ParseState::BranchStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::While) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::WhileStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Loop) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::LoopStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::For) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::ForStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
        parser_state_stack.push(ParseState::ForStatementIn);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Break) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Break(Break::new(current_token.clone())))));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Continue) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Continue(Continue::new(current_token.clone())))));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Return) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Return(Return::new(current_token.clone())))));
        token_index += 1;
        parser_state_stack.goto(ParseState::StatementFinalize);
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Pack) => {
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::PackBodyStart);
        parser_state_stack.push(ParseState::PackIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
      }
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::LeftParen) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::RightParen) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::String) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::InterpolatedString) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Number) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Identifier) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::True) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::False) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::As) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::LeftSquare) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::RightSquare) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Dot) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Comma) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Colon) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Symbol) |
      (Some(ParseState::Statement), Some(AstSymbol::StatementData(_)), TokenType::Newline) => {
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::ExpressionStatement);
        parser_state_stack.push(ParseState::Expression);
      }

      //<editor-fold desc="> Pack">
      (Some(ParseState::PackIdentifier), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.pop();
      }
      (Some(ParseState::PackBodyStart), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::PackMembers(Vec::new()));
        token_index += 1;
        parser_state_stack.goto(ParseState::PackBody);
      }
      (Some(ParseState::PackBodyEnd), Some(AstSymbol::PackMembers(pack_members)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let identifier_sym = ast_stack.pop_panic();
        let pack_token_sym = ast_stack.pop_panic();
        match (pack_token_sym, identifier_sym, left_curly_sym) {
          (AstSymbol::Token(pack_token), AstSymbol::Token(identifier), AstSymbol::Token(left_curly)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Pack(Pack::new(pack_token, identifier, left_curly, pack_members, current_token.clone())))));
            token_index += 1;
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state")
        }
      }
      (Some(ParseState::PackBody), Some(AstSymbol::PackMembers(_)), TokenType::Identifier) => {
        parser_state_stack.push(ParseState::IdentifierStart);
      }
      (Some(ParseState::PackBody), Some(AstSymbol::PackMembers(_)), TokenType::RightCurly) => {
        parser_state_stack.goto(ParseState::PackBodyEnd);
      }
      (Some(ParseState::PackBody), Some(AstSymbol::Identifier(identifier)), TokenType::Comma) => {
        ast_stack.pop();
        let pack_members_sym = ast_stack.pop_panic();
        match pack_members_sym {
          AstSymbol::PackMembers(mut pack_members) => {
            pack_members.push(PackMember::new(identifier, Some(current_token.clone())));
            ast_stack.push(AstSymbol::PackMembers(pack_members));
            token_index += 1;
          }
          _ => panic!("Invalid state")
        }
      }
      (Some(ParseState::PackBody), Some(AstSymbol::Identifier(identifier)), _) => {
        ast_stack.pop();
        let pack_members_sym = ast_stack.pop_panic();
        match pack_members_sym {
          AstSymbol::PackMembers(mut pack_members) => {
            pack_members.push(PackMember::new(identifier, None));
            ast_stack.push(AstSymbol::PackMembers(pack_members));
          }
          _ => panic!("Invalid state")
        }
      }
      (Some(ParseState::PackBody), _, TokenType::Comment) |
      (Some(ParseState::PackBody), _, TokenType::Newline) => {
        token_index += 1;
      }
      //</editor-fold>

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
      (Some(ParseState::Type), Some(_), TokenType::Identifier) => {
        parser_state_stack.goto(ParseState::TypeArray);
        ast_stack.push(AstSymbol::Type(Type::Custom(CustomType::new(current_token.clone()))));
        token_index += 1;
      }
      (Some(ParseState::Type), Some(_), TokenType::Type) => {
        parser_state_stack.goto(ParseState::TypeArray);
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
      (Some(ParseState::Type), Some(_), TokenType::Function) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionParams);
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) |
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) |
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Newline) |
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Comment) => {
        token_index += 1;
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index += 1;
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :(")
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        ast_stack.push(AstSymbol::FunctionTypeArguments(Vec::new()));
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::Type(param_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, None));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
            ast_stack.push(AstSymbol::Token(current_token.clone()));
            token_index += 1;
            parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
          }
          _ => panic!("invalid state :(")
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::RightParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
      }
      (Some(ParseState::TypeFunctionOptArrow), Some(_), TokenType::Arrow) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionReturns);
      }
      (Some(ParseState::TypeFunctionOptArrow), Some(AstSymbol::Token(right_paren)), _) => {
        ast_stack.pop();
        let param_types_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let function_token_sym = ast_stack.pop_panic();
        match (function_token_sym, left_paren_sym, param_types_sym) {
          (AstSymbol::Token(function_token), AstSymbol::Token(left_paren), AstSymbol::FunctionTypeArguments(param_types)) => {
            ast_stack.push(AstSymbol::Type(Type::Function(FunctionType::new(function_token, left_paren, param_types, right_paren, None, None, Vec::new(), None))));
            parser_state_stack.goto(ParseState::TypeArray);
          }
          _ => panic!("invalid state :(")
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) |
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) |
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Newline) |
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Comment) => {
        token_index += 1;
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index += 1;
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :(")
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        ast_stack.push(AstSymbol::FunctionTypeArguments(Vec::new()));
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, None));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
            // TODO
          }
          _ => panic!("invalid state :(")
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(return_args)), TokenType::RightParen) => {
        ast_stack.pop();
        let return_left_paren_sym = ast_stack.pop_panic();
        let arrow_sym = ast_stack.pop_panic();
        let right_paren_sym = ast_stack.pop_panic();
        let params_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let function_sym = ast_stack.pop_panic();
        match (function_sym.clone(), left_paren_sym.clone(), params_sym.clone(), right_paren_sym.clone(), arrow_sym.clone(), return_left_paren_sym.clone()) {
          (AstSymbol::Token(function_token), AstSymbol::Token(left_paren), AstSymbol::FunctionTypeArguments(params), AstSymbol::Token(right_paren), AstSymbol::Token(arrow_token), AstSymbol::Token(return_left_paren)) => {
            ast_stack.push(AstSymbol::Type(Type::Function(FunctionType::new(function_token, left_paren, params, right_paren, Some(arrow_token), Some(return_left_paren), return_args, Some(current_token.clone())))));
            token_index += 1;
            parser_state_stack.goto(ParseState::TypeArray);
          }
          _ => panic!("invalid state :(\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", function_sym, left_paren_sym, params_sym, right_paren_sym, arrow_sym, return_left_paren_sym)
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
      //<editor-fold desc="> Let Assignment">
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
      //</editor-fold>
      //<editor-fold desc="> Expression">
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
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Type) |
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::TypePrefix) |
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
      (Some(ParseState::ExpressionStatement), Some(AstSymbol::Expression(expression_node)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Expression(expression_node))));
        parser_state_stack.pop();
      }
      //</editor-fold>
      //<editor-fold desc="> BranchStatement">
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Else) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.push(ParseState::BranchElseStatement);
      }
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Comment) |
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Newline) => {
        // TODO need to record these somewhere
        token_index += 1;
      }
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(compound_statement)), _) => {
        ast_stack.pop();
        let condition_sym = ast_stack.pop_panic();
        let if_token_sym = ast_stack.pop_panic();
        match (if_token_sym, condition_sym) {
          (AstSymbol::Token(if_token), AstSymbol::Expression(condition)) => {
            ast_stack.push(AstSymbol::Branch(Branch::new(if_token, condition, compound_statement, None)));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      (Some(ParseState::BranchStatement), Some(AstSymbol::ElseBranch(else_branch)), _) => {
        ast_stack.pop();
        let compound_statement_sym = ast_stack.pop_panic();
        let condition_sym = ast_stack.pop_panic();
        let if_token_sym = ast_stack.pop_panic();
        match (if_token_sym, condition_sym, compound_statement_sym) {
          (AstSymbol::Token(if_token), AstSymbol::Expression(condition), AstSymbol::CompoundStatement(compound_statement)) => {
            ast_stack.push(AstSymbol::Branch(Branch::new(if_token, condition, compound_statement, Some(else_branch))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      (Some(ParseState::BranchEndStatement), Some(AstSymbol::Branch(branch)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Branch(branch))));
        parser_state_stack.pop();
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::Comment) |
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::Newline) => {
        // TODO need to record these somewhere
        token_index += 1;
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::LeftCurly) => {
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.push(ParseState::BranchStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::CompoundStatement(compound_statement)), _) => {
        ast_stack.pop();
        let else_token_sym = ast_stack.pop_panic();
        match else_token_sym {
          AstSymbol::Token(else_token) => {
            ast_stack.push(AstSymbol::ElseBranch(ElseBranch::new(else_token, Either::Left(compound_statement))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Branch(branch)), _) => {
        ast_stack.pop();
        let else_token_sym = ast_stack.pop_panic();
        match else_token_sym {
          AstSymbol::Token(else_token) => {
            ast_stack.push(AstSymbol::ElseBranch(ElseBranch::new(else_token, Either::Right(Box::new(branch)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      //</editor-fold>
      //<editor-fold desc="> CompoundStatement">
      (Some(ParseState::CompoundStatement), Some(_), TokenType::Newline) => {
        token_index += 1;
      }
      (Some(ParseState::CompoundStatement), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::StatementList(vec![]));
        token_index += 1;
        parser_state_stack.push(ParseState::StatementList);
      }
      (Some(ParseState::CompoundStatement), Some(AstSymbol::StatementList(compound_statement)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        match left_curly_sym {
          AstSymbol::Token(left_curly) => {
            ast_stack.push(AstSymbol::CompoundStatement(CompoundStatement::new(left_curly, compound_statement, current_token.clone())));
            token_index += 1;
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      //</editor-fold>
      //<editor-fold desc="> WhileStatement">
      (Some(ParseState::WhileStatement), Some(AstSymbol::CompoundStatement(compound_statement)), _) => {
        ast_stack.pop();
        let condition_sym = ast_stack.pop_panic();
        let while_token_sym = ast_stack.pop_panic();
        match (while_token_sym, condition_sym) {
          (AstSymbol::Token(while_token), AstSymbol::Expression(expression_node)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::WhileLoop(WhileLoop::new(while_token, expression_node, compound_statement)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      //</editor-fold>
      //<editor-fold desc="> LoopStatement">
      (Some(ParseState::LoopStatement), Some(AstSymbol::CompoundStatement(compound_statement)), _) => {
        ast_stack.pop();
        let loop_token_sym = ast_stack.pop_panic();
        match loop_token_sym {
          AstSymbol::Token(loop_token) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Loop(Loop::new(loop_token, compound_statement)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      //</editor-fold>

      //<editor-fold desc="> ForStatement">
      (Some(ParseState::ForStatement), Some(AstSymbol::CompoundStatement(compound_statement)), _) => {
        ast_stack.pop();
        let iterable_sym = ast_stack.pop_panic();
        let in_token_sym = ast_stack.pop_panic();
        let iterator_sym = ast_stack.pop_panic();
        let for_token_sym = ast_stack.pop_panic();
        match (for_token_sym, iterator_sym, in_token_sym, iterable_sym) {
          (AstSymbol::Token(for_token), AstSymbol::Expression(iterator), AstSymbol::Token(in_token), AstSymbol::Expression(iterable)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::ForLoop(ForLoop::new(for_token, iterator, in_token, iterable, compound_statement)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(")
        }
      }
      (Some(ParseState::ForStatementIn), Some(AstSymbol::Expression(_)), TokenType::In) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.pop();
      }
      //</editor-fold>

      (Some(ParseState::StatementFinalize), Some(AstSymbol::OptStatement(optional_statement)), _) => {
        ast_stack.pop();
        let statement_data = ast_stack.pop_panic();
        let statements = ast_stack.pop_panic();
        match (statement_data.clone(), statements.clone()) { // TODO remove these clones once the parser is done
          (AstSymbol::StatementData(data), AstSymbol::StatementList(mut statements)) => {
            statements.push(StatementNode::new(data, optional_statement));
            ast_stack.push(AstSymbol::StatementList(statements));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :( {:?} {:?}", statement_data, statements),
        }
      }
      (a, b, c) => {
        ast_stack.print();
        parser_state_stack.print();
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
