use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::parser::astsymbolstack::*;
use crate::ocean::frontend::parser::parsestatestack::*;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;
use crate::util::token::Token;
use itertools::Either;

pub fn parse_phase_one(tokens: &Vec<Token<TokenType>>) -> (Program, Vec<Error>) {
  let (ast_symbol, _, errors) = parse_phase_one_partial(tokens, 0, ParseState::StatementList, Some(AstSymbol::StatementList(Vec::new())));
  match ast_symbol {
    AstSymbol::StatementList(statements) => (Program { statements }, errors),
    _ => panic!("Unexpected parse state :(. Expected AstSymbol::StatementList but got {:?}", ast_symbol)
  }
}

pub fn parse_phase_one_partial(tokens: &Vec<Token<TokenType>>, initial_token_index: usize, initial_parse_state: ParseState, initial_ast_symbol: Option<AstSymbol>) -> (AstSymbol, usize, Vec<Error>) {
  let mut parser_state_stack = ParseStateStack::new();
  let mut ast_stack = AstSymbolStack::new();
  let mut token_index = initial_token_index;

  let mut errors = Vec::new();

  parser_state_stack.push(initial_parse_state);
  match initial_ast_symbol {
    Some(symbol) => ast_stack.push(symbol),
    None => {}
  };

  loop {
    if token_index >= tokens.len() {
      break;
    }

    let current_token = &tokens[token_index];
    let current_state = parser_state_stack.current_state();
    let current_ast_symbol = ast_stack.peek();

    //println!("{} > {} - {:?}", token_index, current_token, current_state);

    match (current_state, current_ast_symbol, &current_token.token_type) {
      (Some(ParseState::StatementList), Some(AstSymbol::StatementList(_)), TokenType::EndOfInput) => {
        break;
      }
      (Some(ParseState::StatementList), Some(AstSymbol::StatementList(_)), TokenType::RightCurly) => {
        parser_state_stack.pop();
      }
      (None, Some(AstSymbol::StatementList(_)), TokenType::RightCurly) => {
        parser_state_stack.push(ParseState::StatementList);
        errors.push(Error::new(Severity::Error, current_token.get_span(), "Unexpected right curly brace. Either this is extraneous or there is a missing left curly brace prior to this curly brace.".to_string()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::StatementList), _, TokenType::Newline) => {
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::StatementList), _, _) => {
        parser_state_stack.push(ParseState::PreStatement);
        ast_stack.push(AstSymbol::StatementData(Vec::new()));
      }

      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(mut statement_data)), TokenType::AnnotationBlock) => {
        ast_stack.pop();
        statement_data.push(StatementNodeData::Annotation(Annotation::new(current_token.clone())));
        ast_stack.push(AstSymbol::StatementData(statement_data));
        ast_stack.push(AstSymbol::OptStatement(None));
        parser_state_stack.goto(ParseState::StatementFinalize);
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(mut statement_data)), TokenType::Annotation) => {
        ast_stack.pop();
        statement_data.push(StatementNodeData::Annotation(Annotation::new(current_token.clone())));
        ast_stack.push(AstSymbol::StatementData(statement_data));
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(mut statement_data)), TokenType::Comment) => {
        ast_stack.pop();
        statement_data.push(StatementNodeData::Comment(Comment::new(current_token.clone())));
        ast_stack.push(AstSymbol::StatementData(statement_data));
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(_)), TokenType::Newline) => {
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::PreStatement), Some(AstSymbol::StatementData(_)), _) => {
        parser_state_stack.goto(ParseState::StatementFinalize);
        parser_state_stack.push(ParseState::Statement);
      }
      (Some(ParseState::PreStatement), _, _) => {
        panic!("Invalid parse state")
      }

      (Some(ParseState::Statement), Some(_), TokenType::EndOfInput) => {
        ast_stack.push(AstSymbol::OptStatement(None));
        parser_state_stack.pop();
      }
      (Some(ParseState::Statement), Some(AstSymbol::OptStatement(_)), _) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Statement), Some(_), TokenType::Using) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::UsingPathEntries(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::UsingPathIdentifier);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Let) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::LetAssignment);
        parser_state_stack.push(ParseState::IdentifierStart);
      }
      (Some(ParseState::Statement), Some(_), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::BranchEndStatement);
        parser_state_stack.push(ParseState::BranchStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Match) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::MatchBody);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(_), TokenType::While) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::WhileStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Loop) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::LoopStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::Statement), Some(_), TokenType::For) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::ForStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
        parser_state_stack.push(ParseState::ForStatementIn);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Break) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Break(Break::new(current_token.clone())))));
        token_index = consume_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::Statement), Some(_), TokenType::Continue) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Continue(Continue::new(current_token.clone())))));
        token_index = consume_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::Statement), Some(_), TokenType::Return) => {
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Return(Return::new(current_token.clone())))));
        token_index = consume_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::Statement), Some(_), TokenType::Pack) => {
        parser_state_stack.goto(ParseState::PackBodyStart);
        parser_state_stack.push(ParseState::InterfaceDeclaration);
        parser_state_stack.push(ParseState::PackIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Union) => {
        parser_state_stack.goto(ParseState::UnionBodyStart);
        parser_state_stack.push(ParseState::InterfaceDeclaration);
        parser_state_stack.push(ParseState::UnionIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Interface) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::InterfaceBodyStart);
        parser_state_stack.push(ParseState::InterfaceIdentifier);
      }
      (Some(ParseState::Statement), Some(_), TokenType::Function) => {
        parser_state_stack.goto(ParseState::FunctionBody);
        parser_state_stack.push(ParseState::FunctionReturnStart);
        parser_state_stack.push(ParseState::FunctionArrow);
        parser_state_stack.push(ParseState::FunctionParameterStart);
        parser_state_stack.push(ParseState::FunctionIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Statement), Some(_), TokenType::LeftParen)
      | (Some(ParseState::Statement), Some(_), TokenType::RightParen)
      | (Some(ParseState::Statement), Some(_), TokenType::String)
      | (Some(ParseState::Statement), Some(_), TokenType::InterpolatedString)
      | (Some(ParseState::Statement), Some(_), TokenType::Number)
      | (Some(ParseState::Statement), Some(_), TokenType::Identifier)
      | (Some(ParseState::Statement), Some(_), TokenType::True)
      | (Some(ParseState::Statement), Some(_), TokenType::False)
      | (Some(ParseState::Statement), Some(_), TokenType::As)
      | (Some(ParseState::Statement), Some(_), TokenType::LeftSquare)
      | (Some(ParseState::Statement), Some(_), TokenType::RightSquare)
      | (Some(ParseState::Statement), Some(_), TokenType::Dot)
      | (Some(ParseState::Statement), Some(_), TokenType::Comma)
      | (Some(ParseState::Statement), Some(_), TokenType::Colon)
      | (Some(ParseState::Statement), Some(_), TokenType::Symbol)
      | (Some(ParseState::Statement), Some(_), TokenType::Newline) => {
        parser_state_stack.goto(ParseState::ExpressionStatement);
        parser_state_stack.push(ParseState::Expression);
      }

      //<editor-fold desc="> Pack">
      (Some(ParseState::PackIdentifier), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeCustom);
        parser_state_stack.push(ParseState::TypeArguments);
      }
      (Some(ParseState::PackBodyStart), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::PackMembers(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::PackBody);
      }
      (Some(ParseState::PackBodyEnd), Some(AstSymbol::PackMembers(pack_members)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let interface_declaration_sym = ast_stack.pop_panic();
        let custom_type_sym = ast_stack.pop_panic();
        let pack_token_sym = ast_stack.pop_panic();
        match (pack_token_sym, custom_type_sym, interface_declaration_sym, left_curly_sym) {
          (AstSymbol::Token(pack_token), AstSymbol::Type(Type::Custom(custom_type)), AstSymbol::InterfaceDeclaration(interface_dec), AstSymbol::Token(left_curly)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Pack(Pack::new(pack_token, custom_type, interface_dec, left_curly, pack_members, current_token.clone())))));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
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
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid state"),
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
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::PackBody), _, TokenType::Comment) | (Some(ParseState::PackBody), _, TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      //</editor-fold>

      //<editor-fold desc="> Interface">
      (Some(ParseState::InterfaceIdentifier), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeCustom);
        parser_state_stack.push(ParseState::TypeArguments);
      }
      (Some(ParseState::InterfaceBodyStart), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::InterfaceMembers(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::InterfaceBody);
      }
      (Some(ParseState::InterfaceBodyEnd), Some(AstSymbol::InterfaceMembers(interface_members)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let custom_type_sym = ast_stack.pop_panic();
        let interface_token_sym = ast_stack.pop_panic();
        match (interface_token_sym, custom_type_sym, left_curly_sym) {
          (AstSymbol::Token(interface_token), AstSymbol::Type(Type::Custom(custom_type)), AstSymbol::Token(left_curly)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Interface(Interface::new(interface_token, custom_type, left_curly, interface_members, current_token.clone())))));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::InterfaceBody), Some(AstSymbol::InterfaceMembers(_)), TokenType::Function) => {
        parser_state_stack.push(ParseState::FunctionBody);
        parser_state_stack.push(ParseState::FunctionReturnStart);
        parser_state_stack.push(ParseState::FunctionArrow);
        parser_state_stack.push(ParseState::FunctionParameterStart);
        parser_state_stack.push(ParseState::FunctionIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::InterfaceBody), Some(AstSymbol::InterfaceMembers(_)), TokenType::RightCurly) => {
        parser_state_stack.goto(ParseState::InterfaceBodyEnd);
      }
      (Some(ParseState::InterfaceBody), Some(AstSymbol::OptStatement(Some(Statement::Function(function)))), TokenType::Comma) => {
        ast_stack.pop();
        let interface_members_sym = ast_stack.pop_panic();
        match interface_members_sym {
          AstSymbol::InterfaceMembers(mut interface_entries) => {
            interface_entries.push(InterfaceEntry::new(function, Some(current_token.clone())));
            ast_stack.push(AstSymbol::InterfaceMembers(interface_entries));
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::InterfaceBody), Some(AstSymbol::OptStatement(Some(Statement::Function(function)))), _) => {
        ast_stack.pop();
        let interface_members_sym = ast_stack.pop_panic();
        match interface_members_sym {
          AstSymbol::InterfaceMembers(mut interface_entries) => {
            interface_entries.push(InterfaceEntry::new(function, None));
            ast_stack.push(AstSymbol::InterfaceMembers(interface_entries));
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::InterfaceBody), _, TokenType::Comment) | (Some(ParseState::InterfaceBody), _, TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::InterfaceDeclaration), Some(_), TokenType::Colon) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::InterfaceImpls(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::InterfaceImplDeclarations);
      }
      (Some(ParseState::InterfaceDeclaration), Some(AstSymbol::InterfaceImpls(interface_impl_declarations)), _) => {
        ast_stack.pop();
        let colon_token = ast_stack.pop_panic();
        match colon_token {
          AstSymbol::Token(colon_token) => {
            ast_stack.push(AstSymbol::InterfaceDeclaration(Some(InterfaceDeclaration::new(colon_token, interface_impl_declarations))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::InterfaceDeclaration), Some(_), _) => {
        ast_stack.push(AstSymbol::InterfaceDeclaration(None));
        parser_state_stack.pop();
      }
      (Some(ParseState::InterfaceImplDeclarations), Some(AstSymbol::InterfaceImpls(_)), TokenType::LeftCurly) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::InterfaceImplDeclarations), Some(AstSymbol::InterfaceImpls(_)), _) => {
         parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::InterfaceImplDeclarations), Some(AstSymbol::Type(parsed_type)), TokenType::Comma) => {
        ast_stack.pop();
        let interface_impls = ast_stack.pop_panic();
        match interface_impls {
          AstSymbol::InterfaceImpls(mut interface_impls) => {
            interface_impls.push(InterfaceImplDeclaration::new(parsed_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::InterfaceImpls(interface_impls));
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid parse state"),
        }
      }
      (Some(ParseState::InterfaceImplDeclarations), Some(AstSymbol::Type(parsed_type)), _) => {
        ast_stack.pop();
        let interface_impls = ast_stack.pop_panic();
        match interface_impls {
          AstSymbol::InterfaceImpls(mut interface_impls) => {
            interface_impls.push(InterfaceImplDeclaration::new(parsed_type, None));
            ast_stack.push(AstSymbol::InterfaceImpls(interface_impls));
          }
          _ => panic!("Invalid parse state"),
        }
      }
      //</editor-fold>

      //<editor-fold desc="> Union Statement">
      (Some(ParseState::UnionIdentifier), _, TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeCustom);
        parser_state_stack.push(ParseState::TypeArguments);
      }
      (Some(ParseState::UnionBodyStart), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::UnionMembers(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::UnionBody);
      }
      (Some(ParseState::UnionBodyEnd), Some(AstSymbol::UnionMembers(union_members)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let interface_declaration = ast_stack.pop_panic();
        let custom_type_sym = ast_stack.pop_panic();
        let union_token_sym = ast_stack.pop_panic();
        match (union_token_sym, custom_type_sym, interface_declaration, left_curly_sym) {
          (AstSymbol::Token(union_token), AstSymbol::Type(Type::Custom(custom_type)), AstSymbol::InterfaceDeclaration(interface_declaration), AstSymbol::Token(left_curly)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Union(Union::new(union_token, custom_type, interface_declaration, left_curly, union_members, current_token.clone())))));
            token_index = consume_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::UnionBody), Some(AstSymbol::UnionMembers(_)), TokenType::Newline) | (Some(ParseState::UnionBody), Some(AstSymbol::UnionMembers(_)), TokenType::Comment) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::UnionBody), Some(AstSymbol::UnionMembers(_)), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::UnionSubTypeStart);
      }
      (Some(ParseState::UnionBody), Some(AstSymbol::UnionMembers(_)), TokenType::RightCurly) => {
        parser_state_stack.goto(ParseState::UnionBodyEnd);
      }
      (Some(ParseState::UnionSubTypeStart), Some(_), TokenType::Newline) | (Some(ParseState::UnionSubTypeStart), Some(_), TokenType::Comment) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::UnionSubTypeStart), _, TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::UnionSubTypeEntries(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::UnionSubType);
      }
      (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::Token(_)), TokenType::RightCurly) | (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::Token(_)), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::OptToken(None));
        parser_state_stack.goto(ParseState::UnionMemberNoSubType);
      }
      (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::Token(_)), TokenType::Comma) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::UnionMemberNoSubType);
      }
      (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::UnionSubTypes(_)), TokenType::RightCurly) | (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::UnionSubTypes(_)), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::OptToken(None));
        parser_state_stack.goto(ParseState::UnionMemberSubType);
      }
      (Some(ParseState::UnionSubTypeStart), Some(AstSymbol::UnionSubTypes(_)), TokenType::Comma) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::UnionMemberSubType);
      }
      (Some(ParseState::UnionSubType), Some(AstSymbol::UnionSubTypeEntries(_)), TokenType::Identifier) | (Some(ParseState::UnionSubType), Some(AstSymbol::UnionSubTypeEntries(_)), TokenType::Type) | (Some(ParseState::UnionSubType), Some(AstSymbol::UnionSubTypeEntries(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::UnionSubType), Some(_), TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::UnionSubType), Some(AstSymbol::Type(sub_type)), TokenType::Comma) => {
        ast_stack.pop();
        let union_member_subtypes = ast_stack.pop_panic();
        match union_member_subtypes {
          AstSymbol::UnionSubTypeEntries(mut union_member_subtypes) => {
            union_member_subtypes.push(UnionSubTypeEntry::new(sub_type, Some(current_token.clone())));
            token_index = consume_comments_newline(tokens, token_index);
            ast_stack.push(AstSymbol::UnionSubTypeEntries(union_member_subtypes));
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::UnionSubType), Some(AstSymbol::Type(sub_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let union_member_subtypes = ast_stack.pop_panic();
        match union_member_subtypes {
          AstSymbol::UnionSubTypeEntries(mut union_member_subtypes) => {
            union_member_subtypes.push(UnionSubTypeEntry::new(sub_type, None));
            ast_stack.push(AstSymbol::UnionSubTypeEntries(union_member_subtypes));
            parser_state_stack.goto(ParseState::UnionSubTypeEnd);
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::UnionSubTypeEnd), Some(AstSymbol::UnionSubTypeEntries(sub_type_entries)), TokenType::RightParen) => {
        ast_stack.pop();
        let left_paren_token = ast_stack.pop_panic();
        match left_paren_token {
          AstSymbol::Token(left_paren_token) => {
            ast_stack.push(AstSymbol::UnionSubTypes(UnionSubTypes::new(left_paren_token, sub_type_entries, current_token.clone())));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::UnionMemberNoSubType), Some(AstSymbol::OptToken(opt_comma_token)), _) => {
        ast_stack.pop();
        let union_member_id_sym = ast_stack.pop_panic();
        let union_members_sym = ast_stack.pop_panic();
        match (union_member_id_sym, union_members_sym) {
          (AstSymbol::Token(union_member_id), AstSymbol::UnionMembers(mut union_members)) => {
            union_members.push(UnionMember::new(union_member_id, None, opt_comma_token));
            ast_stack.push(AstSymbol::UnionMembers(union_members));
            parser_state_stack.pop();
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::UnionMemberSubType), Some(AstSymbol::OptToken(opt_comma_token)), _) => {
        ast_stack.pop();
        let union_member_sub_types = ast_stack.pop_panic();
        let union_member_id_sym = ast_stack.pop_panic();
        let union_members_sym = ast_stack.pop_panic();
        match (union_member_id_sym, union_member_sub_types, union_members_sym) {
          (AstSymbol::Token(union_member_id), AstSymbol::UnionSubTypes(union_member_sub_types), AstSymbol::UnionMembers(mut union_members)) => {
            union_members.push(UnionMember::new(union_member_id, Some(union_member_sub_types), opt_comma_token));
            ast_stack.push(AstSymbol::UnionMembers(union_members));
            parser_state_stack.pop();
          }
          _ => panic!("invalid state :("),
        }
      }
      //</editor-fold>

      //<editor-fold desc="> Function">
      (Some(ParseState::FunctionIdentifier), Some(_), TokenType::Identifier) => {
        parser_state_stack.pop();
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionParameterStart), Some(_), TokenType::LeftParen) => {
        parser_state_stack.goto(ParseState::FunctionParameter);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::FunctionParams(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionParameter), Some(_), TokenType::Comment) | (Some(ParseState::FunctionParameter), Some(_), TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionParameter), Some(AstSymbol::FunctionParams(_)), TokenType::RightParen) => {
        parser_state_stack.goto(ParseState::FunctionParameterEnd);
      }
      (Some(ParseState::FunctionParameter), Some(AstSymbol::FunctionParams(_)), TokenType::Identifier) | (Some(ParseState::FunctionParameter), Some(AstSymbol::FunctionParams(_)), TokenType::Type) | (Some(ParseState::FunctionParameter), Some(AstSymbol::FunctionParams(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::IdentifierStart);
      }
      (Some(ParseState::FunctionParameter), Some(AstSymbol::Identifier(identifier)), TokenType::Comma) => {
        ast_stack.pop();
        let function_params = ast_stack.pop_panic();
        match function_params {
          AstSymbol::FunctionParams(mut function_params) => {
            function_params.push(FunctionParam::new(identifier, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionParams(function_params));
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionParameter), Some(AstSymbol::Identifier(identifier)), TokenType::RightParen) => {
        ast_stack.pop();
        let function_params = ast_stack.pop_panic();
        match function_params {
          AstSymbol::FunctionParams(mut function_params) => {
            function_params.push(FunctionParam::new(identifier, None));
            ast_stack.push(AstSymbol::FunctionParams(function_params));
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionParameterEnd), Some(AstSymbol::FunctionParams(_)), TokenType::RightParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::FunctionArrow), Some(_), TokenType::Comment) | (Some(ParseState::FunctionArrow), Some(_), TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionArrow), Some(_), TokenType::Arrow) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::FunctionArrow), Some(_), _) => {
        ast_stack.push(AstSymbol::OptToken(None));
        parser_state_stack.pop();
      }
      (Some(ParseState::FunctionReturnStart), Some(AstSymbol::OptToken(Some(_))), TokenType::LeftParen) => {
        parser_state_stack.goto(ParseState::FunctionReturn);
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        ast_stack.push(AstSymbol::FunctionReturns(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionReturnStart), Some(AstSymbol::OptToken(None)), _) => {
        ast_stack.push(AstSymbol::OptToken(None));
        ast_stack.push(AstSymbol::FunctionReturns(Vec::new()));
        ast_stack.push(AstSymbol::OptToken(None));
        parser_state_stack.pop();
      }
      (Some(ParseState::FunctionReturn), Some(_), TokenType::Comment)
      | (Some(ParseState::FunctionReturn), Some(_), TokenType::Newline) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::FunctionReturns(_)), TokenType::RightParen) => {
        parser_state_stack.goto(ParseState::FunctionReturnEnd);
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::FunctionReturns(_)), TokenType::Identifier)
      | (Some(ParseState::FunctionReturn), Some(AstSymbol::FunctionReturns(_)), TokenType::Type)
      | (Some(ParseState::FunctionReturn), Some(AstSymbol::FunctionReturns(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::IdentifierStart);
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::Identifier(_)), TokenType::Symbol) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::ExpressionNoComma);
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::Identifier(identifier)), TokenType::Comma) => {
        ast_stack.pop();
        let function_params = ast_stack.pop_panic();
        match function_params {
          AstSymbol::FunctionReturns(mut function_returns) => {
            function_returns.push(FunctionReturn::new(identifier, None, None, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionReturns(function_returns));
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::Expression(expression_node)), TokenType::Comma) => {
        ast_stack.pop();
        let assignment_symbol = ast_stack.pop_panic();
        let identifier = ast_stack.pop_panic();
        let function_params = ast_stack.pop_panic();
        match (function_params, identifier, assignment_symbol) {
          (AstSymbol::FunctionReturns(mut function_returns), AstSymbol::Identifier(identifier), AstSymbol::Token(assignment_symbol)) => {
            function_returns.push(FunctionReturn::new(identifier, Some(assignment_symbol), Some(expression_node), Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionReturns(function_returns));
            token_index = consume_comments_newline(tokens, token_index);
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::Identifier(identifier)), TokenType::RightParen) => {
        ast_stack.pop();
        let function_params = ast_stack.pop_panic();
        match function_params {
          AstSymbol::FunctionReturns(mut function_returns) => {
            function_returns.push(FunctionReturn::new(identifier, None, None, None));
            ast_stack.push(AstSymbol::FunctionReturns(function_returns));
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionReturn), Some(AstSymbol::Expression(expression_node)), TokenType::RightParen) => {
        ast_stack.pop();
        let assignment_symbol = ast_stack.pop_panic();
        let identifier = ast_stack.pop_panic();
        let function_params = ast_stack.pop_panic();
        match (function_params, identifier, assignment_symbol) {
          (AstSymbol::FunctionReturns(mut function_returns), AstSymbol::Identifier(identifier), AstSymbol::Token(assignment_symbol)) => {
            function_returns.push(FunctionReturn::new(identifier, Some(assignment_symbol), Some(expression_node), None));
            ast_stack.push(AstSymbol::FunctionReturns(function_returns));
          }
          _ => panic!("Invalid State :("),
        }
      }
      (Some(ParseState::FunctionReturnEnd), Some(AstSymbol::FunctionReturns(_)), TokenType::RightParen) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::FunctionBody), Some(AstSymbol::OptToken(_)), TokenType::LeftCurly) => {
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::FunctionBody), Some(AstSymbol::CompoundStatement(function_body)), _) => {
        ast_stack.pop();
        let return_right_token = ast_stack.pop_panic();
        let returns = ast_stack.pop_panic();
        let return_left_token = ast_stack.pop_panic();
        let function_arrow = ast_stack.pop_panic();
        let param_right_token = ast_stack.pop_panic();
        let params = ast_stack.pop_panic();
        let param_left_token = ast_stack.pop_panic();
        let function_identifier = ast_stack.pop_panic();
        let function_token = ast_stack.pop_panic();
        match (function_token, function_identifier, param_left_token, params, param_right_token, function_arrow, return_left_token, returns, return_right_token) {
          (AstSymbol::Token(function_token), AstSymbol::Token(function_identifier), AstSymbol::Token(param_left_token), AstSymbol::FunctionParams(params), AstSymbol::Token(param_right_token), AstSymbol::OptToken(function_arrow), AstSymbol::OptToken(returns_left_token), AstSymbol::FunctionReturns(returns), AstSymbol::OptToken(returns_right_token)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Function(Function::new(function_token, function_identifier, param_left_token, params, param_right_token, function_arrow, returns_left_token, returns, returns_right_token, Some(function_body))))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::FunctionBody), Some(_), _) => {
        let return_right_token = ast_stack.pop_panic();
        let returns = ast_stack.pop_panic();
        let return_left_token = ast_stack.pop_panic();
        let function_arrow = ast_stack.pop_panic();
        let param_right_token = ast_stack.pop_panic();
        let params = ast_stack.pop_panic();
        let param_left_token = ast_stack.pop_panic();
        let function_identifier = ast_stack.pop_panic();
        let function_token = ast_stack.pop_panic();
        match (function_token.clone(), function_identifier.clone(), param_left_token.clone(), params.clone(), param_right_token.clone(), function_arrow.clone(), return_left_token.clone(), returns.clone(), return_right_token.clone()) {
          (AstSymbol::Token(function_token), AstSymbol::Token(function_identifier), AstSymbol::Token(param_left_token), AstSymbol::FunctionParams(params), AstSymbol::Token(param_right_token), AstSymbol::OptToken(function_arrow), AstSymbol::OptToken(returns_left_token), AstSymbol::FunctionReturns(returns), AstSymbol::OptToken(returns_right_token)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Function(Function::new(function_token, function_identifier, param_left_token, params, param_right_token, function_arrow, returns_left_token, returns, returns_right_token, None)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :(\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", function_token, function_identifier, param_left_token, params, param_right_token, function_arrow, return_left_token, returns, return_right_token),
        }
      }
      //</editor-fold>

      //<editor-fold desc="> Using Statement">
      (Some(ParseState::UsingPathIdentifier), Some(AstSymbol::UsingPathEntries(_)), TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::UsingPathOptionalDot);
      }
      (Some(ParseState::UsingPathIdentifier), Some(AstSymbol::UsingPathEntries(entries)), _) => {
        ast_stack.pop();
        let using_token = ast_stack.pop_panic();
        match using_token {
          AstSymbol::Token(using_token) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Using(Using::new(using_token, entries.clone())))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
        }
      }
      (Some(ParseState::UsingPathOptionalDot), Some(AstSymbol::Token(identifier)), TokenType::Dot) => {
        ast_stack.pop();
        let path_entries = ast_stack.pop_panic();
        match path_entries {
          AstSymbol::UsingPathEntries(mut entries) => {
            entries.push(UsingPathEntry::new(identifier, Some(current_token.clone())));
            ast_stack.push(AstSymbol::UsingPathEntries(entries));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state"),
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
          _ => panic!("Invalid state"),
        }
      }
      //</editor-fold>
      //<editor-fold desc="> Identifier">
      (Some(ParseState::IdentifierStart), _, TokenType::Identifier) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::IdentifierOptionalColon);
      }
      (Some(ParseState::IdentifierOptionalColon), Some(AstSymbol::Token(_)), TokenType::Colon) => {
        ast_stack.push(AstSymbol::OptToken(Some(current_token.clone())));
        token_index = consume_comments_newline(tokens, token_index);
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
      (Some(ParseState::Type), _, TokenType::Identifier) => {
        parser_state_stack.goto(ParseState::TypeArray);
        parser_state_stack.push(ParseState::TypeCustom);
        parser_state_stack.push(ParseState::TypeArguments);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Type), _, TokenType::Type) => {
        parser_state_stack.goto(ParseState::TypeArray);
        ast_stack.push(AstSymbol::Type(Type::Base(BaseType::new(current_token.clone()))));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Type), _, TokenType::RightSquare) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Type), _, TokenType::TypePrefix) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeArray);
        match current_token.lexeme.as_str() {
          "auto" => {
            parser_state_stack.push(ParseState::TypeAuto);
            parser_state_stack.push(ParseState::TypeIdentifier);
          }
          "lazy" => {
            parser_state_stack.push(ParseState::TypeLazy);
            parser_state_stack.push(ParseState::Type);
          }
          "ref" => {
            parser_state_stack.push(ParseState::TypeRef);
            parser_state_stack.push(ParseState::Type);
          }
          "mut" => {
            parser_state_stack.push(ParseState::TypeMut);
            parser_state_stack.push(ParseState::Type);
          }
          _ => panic!("Unexpected type prefix"),
        }
      }
      (Some(ParseState::Type), _, TokenType::FunctionType) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeFunctionParams);
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) | (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) | (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Newline) | (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Comment) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index = consume_comments_newline(tokens, token_index);
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
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
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::RightParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
      }
      (Some(ParseState::TypeFunctionOptArrow), Some(_), TokenType::Arrow) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
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
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) | (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) | (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Newline) | (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Comment) => {
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index = consume_comments_newline(tokens, token_index);
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        ast_stack.push(AstSymbol::FunctionTypeArguments(Vec::new()));
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, None));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
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
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.goto(ParseState::TypeArray);
          }
          _ => panic!("invalid state :(\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", function_sym, left_paren_sym, params_sym, right_paren_sym, arrow_sym, return_left_paren_sym),
        }
      }
      (Some(ParseState::TypeIdentifier), Some(_), TokenType::Identifier) => {
        token_index = consume_comments_newline(tokens, token_index);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeCustom), Some(AstSymbol::Token(identifier_token)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Type(Type::Custom(CustomType::new(identifier_token, None))));
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeCustom), Some(AstSymbol::TypeParameters(type_parameters)), _) => {
        ast_stack.pop();
        let identifier_token = ast_stack.pop_panic();
        match identifier_token {
          AstSymbol::Token(identifier_token) => {
            ast_stack.push(AstSymbol::Type(Type::Custom(CustomType::new(identifier_token, Some(type_parameters)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state :("),
        }
      }
      (Some(ParseState::TypeArguments), Some(AstSymbol::Token(_)), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::TypeArguments(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::TypeArguments), Some(AstSymbol::Type(type_symbol)), TokenType::Comma) => {
        ast_stack.pop();
        let mut type_arguments = ast_stack.pop_panic();
        match type_arguments {
          AstSymbol::TypeArguments(mut type_arguments) => {
            type_arguments.push(TypeArgument::new(type_symbol, Some(current_token.clone())));
            token_index = consume_comments_newline(tokens, token_index);
            ast_stack.push(AstSymbol::TypeArguments(type_arguments));
          }
          _ => panic!("Invalid parse state :(")
        }
      }
      (Some(ParseState::TypeArguments), Some(AstSymbol::Type(type_symbol)), TokenType::RightParen) => {
        ast_stack.pop();
        let mut type_arguments = ast_stack.pop_panic();
        match type_arguments {
          AstSymbol::TypeArguments(mut type_arguments) => {
            type_arguments.push(TypeArgument::new(type_symbol, None));
            ast_stack.push(AstSymbol::TypeArguments(type_arguments));
          }
          _ => panic!("Invalid parse state :(")
        }
      }
      (Some(ParseState::TypeArguments), Some(AstSymbol::TypeArguments(type_arguments)), TokenType::RightParen) => {
        ast_stack.pop();
        let left_paren_token = ast_stack.pop_panic();
        match left_paren_token {
          AstSymbol::Token(left_paren_token) => {
            ast_stack.push(AstSymbol::TypeParameters(TypeParameters::new(left_paren_token.clone(), type_arguments, current_token.clone())));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state :("),
        }
      }
      (Some(ParseState::TypeArguments), Some(AstSymbol::TypeArguments(_)), _) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeArguments), Some(_), _) => {
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
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.goto(ParseState::TypeArrayEnd);
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeArray), Some(AstSymbol::Type(base_type)), TokenType::Spread) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Type(Type::VariableType(VariableType::new(Box::new(base_type), current_token.clone()))));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArray), Some(_), _) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Type(subtype)), TokenType::RightSquare) => {
        ast_stack.pop();
        let left_square_sym = ast_stack.pop_panic();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index = consume_comments_newline(tokens, token_index);
        match (main_type_sym, left_square_sym) {
          (AstSymbol::Type(main_type), AstSymbol::Token(left_square)) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, Some(Box::new(subtype)), current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :("),
        }
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Token(left_square)), TokenType::RightSquare) => {
        ast_stack.pop();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index = consume_comments_newline(tokens, token_index);
        match main_type_sym {
          AstSymbol::Type(main_type) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, None, current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :("),
        }
      }
      //</editor-fold>
      //<editor-fold desc="> Let Assignment">
      (Some(ParseState::LetAssignment), Some(AstSymbol::Identifier(_)), TokenType::Symbol) => {
        // TODO may need to determine if we have an assignment token here
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::ExpressionStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::LetAssignment), Some(AstSymbol::OptStatement(Some(Statement::Expression(expression_statement)))), _) => {
        ast_stack.pop();
        let assignment_token_sym = ast_stack.pop_panic();
        let identifier_sym = ast_stack.pop_panic();
        let let_token_sym = ast_stack.pop_panic();
        match (&let_token_sym, &identifier_sym, &assignment_token_sym) {
          (AstSymbol::Token(let_token), AstSymbol::Identifier(identifier), AstSymbol::Token(assignment_token)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Assignment(Assignment::new(Either::Left(LetTarget::new(let_token.clone(), identifier.clone())), assignment_token.clone(), expression_statement)))));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state :( {:?} {:?} {:?}", let_token_sym, identifier_sym, assignment_token_sym),
        }
      }
      //</editor-fold>
      //<editor-fold desc="> Expression">
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::LeftParen) => {
        parser_state_stack.push(ParseState::SubExpression);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Match) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::MatchBody);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Function) => {
        parser_state_stack.push(ParseState::FunctionBody);
        parser_state_stack.push(ParseState::FunctionReturnStart);
        parser_state_stack.push(ParseState::FunctionArrow);
        parser_state_stack.push(ParseState::FunctionParameterStart);
        parser_state_stack.push(ParseState::FunctionIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::BranchEndStatement);
        parser_state_stack.push(ParseState::BranchStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::While) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::WhileStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Loop) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::LoopStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(_)), TokenType::For) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::ForStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
        parser_state_stack.push(ParseState::ForStatementIn);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::OptStatement(Some(statement))), _) => {
        ast_stack.pop();
        let token_list = ast_stack.pop_panic();
        match (token_list, statement) {
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Match(match_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Match(Box::new(match_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Function(function_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Function(Box::new(function_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::ForLoop(for_loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::ForLoop(Box::new(for_loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Loop(loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Loop(Box::new(loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::WhileLoop(while_loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::WhileLoop(Box::new(while_loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Branch(branch_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Branch(Box::new(branch_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          _ => panic!("Invalid parser state")
        }
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Comment)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::String)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::InterpolatedString)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Number)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Identifier)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::True)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::False)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::As)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Type)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::TypePrefix)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::LeftSquare)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::RightSquare)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Dot)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Colon)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Symbol)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Is)
      | (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Newline) => {
        ast_stack.pop();
        token_list.push(Either::Left(current_token.clone()));
        ast_stack.push(AstSymbol::ExpressionTokenList(token_list.clone()));
        token_index += 1;
      }
      (Some(ParseState::ExpressionNoComma), Some(AstSymbol::ExpressionTokenList(token_list)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Expression(ExpressionNode::new(token_list)));
        parser_state_stack.pop();
      }
      (Some(ParseState::ExpressionNoComma), Some(_), _) => {
        ast_stack.push(AstSymbol::ExpressionTokenList(Vec::new()));
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::LeftParen) => {
        parser_state_stack.push(ParseState::SubExpression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::RightParen) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Match) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::MatchBody);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Function) => {
        parser_state_stack.push(ParseState::FunctionBody);
        parser_state_stack.push(ParseState::FunctionReturnStart);
        parser_state_stack.push(ParseState::FunctionArrow);
        parser_state_stack.push(ParseState::FunctionParameterStart);
        parser_state_stack.push(ParseState::FunctionIdentifier);
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::BranchEndStatement);
        parser_state_stack.push(ParseState::BranchStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::While) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::WhileStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::Loop) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::LoopStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(_)), TokenType::For) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::ForStatement);
        parser_state_stack.push(ParseState::CompoundStatement);
        parser_state_stack.push(ParseState::Expression);
        parser_state_stack.push(ParseState::ForStatementIn);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::Expression), Some(AstSymbol::OptStatement(Some(statement))), _) => {
        ast_stack.pop();
        let token_list = ast_stack.pop_panic();
        match (token_list, statement) {
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Match(match_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Match(Box::new(match_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Function(function_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Function(Box::new(function_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::ForLoop(for_loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::ForLoop(Box::new(for_loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Loop(loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Loop(Box::new(loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::WhileLoop(while_loop_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::WhileLoop(Box::new(while_loop_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          (AstSymbol::ExpressionTokenList(mut token_list), Statement::Branch(branch_expr)) => {
            token_list.push(Either::Right(AstNodeExpression::Branch(Box::new(branch_expr))));
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
          }
          _ => panic!("Invalid parser state")
        }
      }
      (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Comment)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::String)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::InterpolatedString)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Number)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Identifier)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::True)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::False)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::As)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Type)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::TypePrefix)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::LeftSquare)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::RightSquare)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Dot)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Comma)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Colon)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Symbol)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Is)
      | (Some(ParseState::Expression), Some(AstSymbol::ExpressionTokenList(mut token_list)), TokenType::Newline) => {
        ast_stack.pop();
        token_list.push(Either::Left(current_token.clone()));
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
        ast_stack.push(AstSymbol::ExpressionTokenList(vec![Either::Left(current_token.clone())]));
        parser_state_stack.push(ParseState::Expression);
        token_index += 1;
      }
      (Some(ParseState::SubExpression), Some(AstSymbol::ExpressionTokenList(mut sub_token_list)), TokenType::RightParen) => {
        ast_stack.pop();
        let token_list_sym = ast_stack.pop_panic();
        match token_list_sym {
          AstSymbol::ExpressionTokenList(mut token_list) => {
            sub_token_list.push(Either::Left(current_token.clone()));
            token_list.append(&mut sub_token_list);
            ast_stack.push(AstSymbol::ExpressionTokenList(token_list));
            token_index += 1;
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::ExpressionStatement), Some(AstSymbol::Expression(expression_node)), TokenType::Semicolon) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Expression(ExpressionStatement::new(expression_node, current_token.clone())))));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      (Some(ParseState::ExpressionStatement), Some(AstSymbol::Expression(_)), _) => {
        errors.push(Error::new(Severity::Error, current_token.get_span(), "Unexpected token following expression. Likely missing a semicolon.".to_string()));
        parser_state_stack.pop_until(ParseState::StatementList);
        ast_stack.pop();
        ast_stack.pop(); // resets ast_stack to empty statement list so we can continue parsing statements
      }
      //</editor-fold>
      //<editor-fold desc="> MatchStatement">
      (Some(ParseState::MatchBody), Some(AstSymbol::Expression(_)), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::MatchCases(Vec::new()));
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::MatchBody), Some(AstSymbol::Token(_)), _) => {
        parser_state_stack.push(ParseState::MatchCaseArrow);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::MatchBody), Some(AstSymbol::MatchCases(match_cases)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_token = ast_stack.pop_panic();
        let match_expression = ast_stack.pop_panic();
        let match_token = ast_stack.pop_panic();
        match (match_token, match_expression, left_curly_token) {
          (AstSymbol::Token(match_token), AstSymbol::Expression(match_expression), AstSymbol::Token(left_curly_token)) => {
            ast_stack.push(AstSymbol::OptStatement(Some(Statement::Match(Match::new(match_token, match_expression, left_curly_token, match_cases, current_token.clone())))));
            token_index = consume_comments_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state")
        }
      }
      (Some(ParseState::MatchBody), Some(AstSymbol::MatchCases(_)), _) => {
        parser_state_stack.push(ParseState::MatchCaseArrow);
        parser_state_stack.push(ParseState::Expression);
      }
      (Some(ParseState::MatchCaseArrow), Some(AstSymbol::Expression(_)), TokenType::DoubleArrow) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::MatchCaseBody);
      }
      (Some(ParseState::MatchCaseArrow), Some(AstSymbol::OptStatement(Some(statement))), _) => {
        ast_stack.pop();
        let double_arrow_token = ast_stack.pop_panic();
        let case_expression = ast_stack.pop_panic();
        let match_cases = ast_stack.pop_panic();
        match (match_cases, case_expression, double_arrow_token, statement) {
          (AstSymbol::MatchCases(mut match_cases), AstSymbol::Expression(expression_node), AstSymbol::Token(double_arrow_token), statement) => {
            match_cases.push(MatchCase::new(expression_node, double_arrow_token, Either::Left(statement)));
            ast_stack.push(AstSymbol::MatchCases(match_cases));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state")
        }
      }
      (Some(ParseState::MatchCaseArrow), Some(AstSymbol::CompoundStatement(statement)), _) => {
        ast_stack.pop();
        let double_arrow_token = ast_stack.pop_panic();
        let case_expression = ast_stack.pop_panic();
        let match_cases = ast_stack.pop_panic();
        match (match_cases, case_expression, double_arrow_token, statement) {
          (AstSymbol::MatchCases(mut match_cases), AstSymbol::Expression(expression_node), AstSymbol::Token(double_arrow_token), statement) => {
            match_cases.push(MatchCase::new(expression_node, double_arrow_token, Either::Right(statement)));
            ast_stack.push(AstSymbol::MatchCases(match_cases));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid parse state")
        }
      }
      (Some(ParseState::MatchCaseBody), Some(AstSymbol::Token(_)), TokenType::LeftCurly) => {
        parser_state_stack.goto(ParseState::CompoundStatement);
      }
      (Some(ParseState::MatchCaseBody), Some(AstSymbol::Token(_)), _) => {
        parser_state_stack.goto(ParseState::Statement);
      }
      //</editor-fold>
      //<editor-fold desc="> BranchStatement">
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Else) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::BranchElseStatement);
      }
      (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Comment) | (Some(ParseState::BranchStatement), Some(AstSymbol::CompoundStatement(_)), TokenType::Newline) => {
        // TODO need to record these somewhere
        token_index = consume_comments_newline(tokens, token_index);
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
          _ => panic!("Invalid state :("),
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
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::BranchEndStatement), Some(AstSymbol::Branch(branch)), _) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::OptStatement(Some(Statement::Branch(branch))));
        parser_state_stack.pop();
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::Comment) | (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::Newline) => {
        // TODO need to record these somewhere
        token_index = consume_comments_newline(tokens, token_index);
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::LeftCurly) => {
        parser_state_stack.push(ParseState::CompoundStatement);
      }
      (Some(ParseState::BranchElseStatement), Some(AstSymbol::Token(_)), TokenType::If) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
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
        token_index = consume_newline(tokens, token_index);
      }
      (Some(ParseState::CompoundStatement), Some(_), TokenType::LeftCurly) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        ast_stack.push(AstSymbol::StatementList(vec![]));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.push(ParseState::StatementList);
      }
      (Some(ParseState::CompoundStatement), Some(AstSymbol::StatementList(compound_statement)), TokenType::RightCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        match left_curly_sym {
          AstSymbol::Token(left_curly) => {
            ast_stack.push(AstSymbol::CompoundStatement(CompoundStatement::new(left_curly, compound_statement, current_token.clone())));
            token_index = consume_newline(tokens, token_index);
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::CompoundStatement), Some(_), _) => {
        errors.push(Error::new(Severity::Error, current_token.get_span(), "Unexpected token. Expected an opening curly brace for the compound statement.".to_string()));
        // here add in a dummy left curly and set up the ast stack and parse state stack to continue parsing
        ast_stack.push(AstSymbol::Token(current_token.clone())); // TODO is this right to copy the current_token even though it isn't a left curly??
        ast_stack.push(AstSymbol::StatementList(vec![]));
        parser_state_stack.push(ParseState::StatementList);
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
          _ => panic!("Invalid state :("),
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
          _ => panic!("Invalid state :("),
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
          _ => panic!("Invalid state :("),
        }
      }
      (Some(ParseState::ForStatementIn), Some(AstSymbol::Expression(_)), TokenType::In) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index = consume_comments_newline(tokens, token_index);
        parser_state_stack.pop();
      }
      //</editor-fold>
      (Some(ParseState::StatementFinalize), Some(AstSymbol::OptStatement(optional_statement)), _) => {
        ast_stack.pop();
        let statement_data = ast_stack.pop_panic();
        let statements = ast_stack.pop_panic();
        match (statement_data.clone(), statements.clone()) {
          (AstSymbol::StatementData(data), AstSymbol::StatementList(mut statements)) => {
            statements.push(StatementNode::new(data, optional_statement));
            ast_stack.push(AstSymbol::StatementList(statements));
            parser_state_stack.pop();
          }
          _ => panic!("Invalid state :( {:?} {:?}", statement_data, statements),
        }
      }
      (None, _, _) => {
        break // Idk if this would cause issues but we would probably catch any issues below
      }
      (a, b, c) => {
        ast_stack.print();
        parser_state_stack.print();
        println!("TOKEN:::: {}", current_token);
        panic!("THERE WAS A PARSE ERROR BUT WE DON'T REPORT THE ERROR PROPERLY. REPORT THIS ISSUE WITH THE COMPILER PLEASE :) \nParseState: {:?}\nCurrent AstSymbol: {:#?}\nCurrent Token: {:?}", a, b, c);
      }
    }
  }

  if ast_stack.size() != 1 {
    panic!("Too many things in the ast stack");
  }

  let symbol = ast_stack.pop_panic();
  (symbol, token_index, errors)
}

fn consume_comments_newline(tokens: &Vec<Token<TokenType>>, current_index: usize) -> usize {
  let mut result = current_index + 1;
  while result < tokens.len() && (tokens[result].token_type == TokenType::Newline || tokens[result].token_type == TokenType::Comment) {
    result += 1;
  }
  result
}

fn consume_newline(tokens: &Vec<Token<TokenType>>, current_index: usize) -> usize {
  let mut result = current_index + 1;
  while result < tokens.len() && tokens[result].token_type == TokenType::Newline {
    result += 1;
  }
  result
}
