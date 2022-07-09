pub mod ast;
pub mod display;
pub mod helpers;
pub mod span;

use crate::compiler::OceanError;
use crate::compiler::{Token, TokenType};
use ast::*;
use helpers::*;

use super::errors::Severity;

#[derive(Clone)]
pub enum AstStackSymbol {
  Token(Token),
  Program(Program),
  StmtList(Vec<Statement>),
  Stmt(Statement),
  PackDec(PackDeclaration),
  PackDecList(Vec<PackDeclaration>),
  UnionDec(UnionDeclaration),
  UnionDecList(Vec<UnionDeclaration>),
  MatchEntry(MatchEntry),
  Expr(Expression),
  ExprList(Vec<Box<Expression>>),
  TypeVar(TypeVar),
  Var(Var),
  Type(Type),
  TypeList(Vec<Box<Type>>),
  OptType(Option<Type>),
  ParamList(ParameterList),
  Param(Parameter),
  ReturnList(ReturnList),
  ReturnEntry(ReturnEntry),
  IdList(Vec<Token>),
}

#[derive(Debug, Clone, Copy)]
pub enum AstState {
  StmtList,
  StmtFinalize,
  UseStmtIdList,
  UseStmtIdListFollow,
  UseStmtAlias,
  UseStmtFinalize,
  PackDecName,
  PackDecStartEntryList,
  PackDecEntry,
  PackDecEntryFinalize,
  PackDecFinalize,
  TypeVar,
  TypeVarColon,
  TypeVarFinalize,
  Type,
  BaseTypeFollow,
  BaseTypeFollowEnd,
  TypeChainResolve,
  AutoTypeFollow,
  FuncTypeParamListStart,
  FuncTypeParamList,
  FuncTypeParamListFollow,
  FuncTypeReturnList,
  FuncTypeReturnListFollow,
  SubType,
  UnionDecName,
  UnionDecStartEntryList,
  UnionDecEntry,
  UnionDecStorageStart,
  UnionDecStorage,
  UnionDecStorageFollow,
  UnionDecStorageEnd,
  UnionDecEntryFinalize,
  UnionDecFinalize,

  ExprStmt,
  Expression,
  ArrayLiteralContents,
  ArrayLiteralContentsFollow,
  Primary,
  PrimaryFollow,
  MemberAccess,
  ArrayAccess,
  Prefix, 
  PrefixContents,

}

fn consume_optional_newline(tokens: &Vec<Token>, token_index: usize) -> usize {
  let mut new_token_index = token_index;
  while tokens[new_token_index].token_type == TokenType::Newline {
    new_token_index += 1;
  }
  new_token_index
}

pub fn parse(tokens: &Vec<Token>, start_index: Option<usize>) -> (Option<Program>, Vec<OceanError>) {
  let mut ast_stack = Stack::new();
  let mut errors: Vec<OceanError> = Vec::new();
  let mut state_stack = StateStack::new();
  let mut token_index = match start_index {
    Some(x) => x,
    None => 0
  };

  println!("Start parse");

  ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
  state_stack.goto(AstState::StmtList);

  loop {
    ast_stack.print();
    state_stack.print();
    let current_token = &tokens[token_index];
    let stack_top = ast_stack.peek();
    let state = state_stack.current_state();
    match (state, stack_top, &current_token.token_type) {
      (
        Some(AstState::StmtList),
        Some(AstStackSymbol::StmtList(contents)),
        TokenType::EndOfInput,
      ) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Program(Program::new(contents.to_vec())));
        break;
      }
      (Some(AstState::StmtList), Some(_), TokenType::Keyword) => {
        if current_token.lexeme == "use" {
          state_stack.push(AstState::UseStmtIdList);
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          ast_stack.push(AstStackSymbol::IdList(Vec::new()));
          token_index += 1;
        } else if current_token.lexeme == "pack" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::PackDecName);
        } else if current_token.lexeme == "union" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::UnionDecName);
        } else if current_token.lexeme == "break" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Break(BreakStatement::new(
            current_token.clone(),
          ))));
          token_index += 1;
          state_stack.push(AstState::StmtFinalize);
        } else if current_token.lexeme == "return" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Return(
            ReturnStatement::new(current_token.clone()),
          )));
          token_index += 1;
          state_stack.push(AstState::StmtFinalize);
        } else if current_token.lexeme == "continue" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Continue(
            ContinueStatement::new(current_token.clone()),
          )));
          token_index += 1;
          state_stack.push(AstState::StmtFinalize);
        } else if current_token.lexeme == "true" || current_token.lexeme == "false" {
          state_stack.push(AstState::ExprStmt);
        } else {
          panic!("Unknown keyword {} :(", current_token);
        }
      }
      (Some(AstState::StmtList), Some(_), TokenType::LSquare) |
      (Some(AstState::StmtList), Some(_), TokenType::LParen) |
      (Some(AstState::StmtList), Some(_), TokenType::Symbol) |
      (Some(AstState::StmtList), Some(_), TokenType::Number) |
      (Some(AstState::StmtList), Some(_), TokenType::Identifier) |
      (Some(AstState::StmtList), Some(_), TokenType::InterpolatedString) |
      (Some(AstState::StmtList), Some(_), TokenType::String) => {
        state_stack.push(AstState::ExprStmt);
      }
      (Some(AstState::StmtList), Some(_), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::StmtFinalize), Some(AstStackSymbol::Stmt(stmt)), TokenType::EndOfInput)
      | (Some(AstState::StmtFinalize), Some(AstStackSymbol::Stmt(stmt)), TokenType::Newline) => {
        ast_stack.pop();
        token_index += 1;
        let mut stmt_list_sym = ast_stack.pop_panic();
        match stmt_list_sym {
          Some(AstStackSymbol::StmtList(mut contents)) => {
            contents.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(contents));
            state_stack.pop();
          }
          _ => panic!("Stmt finalize stack busted"),
        }
      }
      (
        Some(AstState::UseStmtIdList),
        Some(AstStackSymbol::IdList(mut contents)),
        TokenType::Identifier,
      ) => {
        ast_stack.pop();
        contents.push(current_token.clone());
        ast_stack.push(AstStackSymbol::IdList(contents));
        state_stack.push(AstState::UseStmtIdListFollow);
        token_index += 1;
      }
      (Some(AstState::UseStmtIdList), _, _) => {
        panic!("Unexpected token {}!! expected identifier", current_token)
      }
      (Some(AstState::UseStmtIdListFollow), Some(AstStackSymbol::IdList(_)), TokenType::Dot) => {
        token_index += 1;
        state_stack.pop();
      }
      (
        Some(AstState::UseStmtIdListFollow),
        Some(AstStackSymbol::IdList(contents)),
        TokenType::Keyword,
      ) => {
        if current_token.lexeme == "as" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.pop();
          state_stack.goto(AstState::UseStmtAlias);
        } else {
          let mut id_list = ast_stack.pop_panic();
          let mut use_token = ast_stack.pop_panic();
          match (use_token, id_list) {
            (Some(AstStackSymbol::Token(ut)), Some(AstStackSymbol::IdList(contents))) => {
              ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(
                ut, contents, None, None,
              ))));
            }
            _ => panic!("Unexpected stack contents use no alias"),
          }
          state_stack.pop();
          state_stack.goto(AstState::StmtFinalize);
        }
      }
      (Some(AstState::UseStmtIdListFollow), Some(AstStackSymbol::IdList(_)), _) => {
        let mut id_list = ast_stack.pop_panic();
        let mut use_token = ast_stack.pop_panic();
        match (use_token, id_list) {
          (Some(AstStackSymbol::Token(ut)), Some(AstStackSymbol::IdList(contents))) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(
              ut, contents, None, None,
            ))));
          }
          _ => panic!("Unexpected stack contents use no alias 2"),
        }
        state_stack.pop();
        state_stack.goto(AstState::StmtFinalize);
      }
      (Some(AstState::UseStmtIdListFollow), _, _) => {
        panic!("Unexpected token {}!! expected identifier", current_token)
      }
      (Some(AstState::UseStmtAlias), Some(_), TokenType::Identifier) => {
        let mut as_token = ast_stack.pop_panic();
        let mut id_list = ast_stack.pop_panic();
        let mut use_token = ast_stack.pop_panic();
        match (use_token, id_list, as_token) {
          (
            Some(AstStackSymbol::Token(ut)),
            Some(AstStackSymbol::IdList(contents)),
            Some(AstStackSymbol::Token(at)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(
              ut,
              contents,
              Some(at),
              Some(current_token.clone()),
            ))));
          }
          _ => panic!("Unexpected stack contents use no alias"),
        }
        token_index += 1;
        state_stack.goto(AstState::StmtFinalize);
      }
      (Some(AstState::UseStmtAlias), _, _) => {
        panic!("Unexpected token {}!! expected identifier", current_token)
      }
      (Some(AstState::PackDecName), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::PackDecStartEntryList);
      }
      (Some(AstState::PackDecName), _, _) => {
        panic!("Unexpected token {}!! expected identifier", current_token)
      }
      (Some(AstState::PackDecStartEntryList), Some(_), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::PackDecList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::PackDecEntry);
      }
      (Some(AstState::PackDecStartEntryList), _, _) => {
        panic!("Unexpected token {}!! expected left curly", current_token)
      }
      (
        Some(AstState::PackDecEntry),
        Some(AstStackSymbol::PackDecList(_)),
        TokenType::Identifier,
      ) => {
        state_stack.push(AstState::TypeVar);
      }
      (Some(AstState::PackDecEntry), Some(AstStackSymbol::PackDecList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::PackDecEntry),
        Some(AstStackSymbol::TypeVar(type_var)),
        _,
      ) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::PackDec(PackDeclaration::new(
          type_var, None, None,
        )));
        state_stack.goto(AstState::PackDecEntryFinalize);
      }
      (Some(AstState::PackDecEntry), Some(AstStackSymbol::PackDecList(_)), TokenType::RCurly) => {
        state_stack.goto(AstState::PackDecFinalize);
      }
      (Some(AstState::PackDecEntry), _, _) => panic!("PackDecEntry error {}", current_token),
      (
        Some(AstState::PackDecEntryFinalize),
        Some(AstStackSymbol::PackDec(entry)),
        TokenType::Comma,
      )
      | (
        Some(AstState::PackDecEntryFinalize),
        Some(AstStackSymbol::PackDec(entry)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let pack_dec_list_sym = ast_stack.pop_panic();
        match pack_dec_list_sym {
          Some(AstStackSymbol::PackDecList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::PackDecList(contents));
            state_stack.goto(AstState::PackDecEntry);
            token_index += 1; //consumes newline or comma
            token_index = consume_optional_newline(tokens, token_index);
          }
          _ => panic!("Pack dec finalize has busted stack"),
        }
      }
      (
        Some(AstState::PackDecEntryFinalize),
        Some(AstStackSymbol::PackDec(entry)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let pack_dec_list_sym = ast_stack.pop_panic();
        match pack_dec_list_sym {
          Some(AstStackSymbol::PackDecList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::PackDecList(contents));
            state_stack.goto(AstState::PackDecFinalize);
          }
          _ => panic!("Pack dec finalize has busted stack rcurly"),
        }
      }
      (Some(AstState::PackDecEntryFinalize), _, _) => {
        panic!("unexpected pack dec entry finalize {}", current_token)
      }
      (
        Some(AstState::PackDecFinalize),
        Some(AstStackSymbol::PackDecList(contents)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let left_curly_token = ast_stack.pop_panic();
        let pack_name = ast_stack.pop_panic();
        let pack_token = ast_stack.pop_panic();
        match (pack_token, pack_name, left_curly_token) {
          (
            Some(AstStackSymbol::Token(pt)),
            Some(AstStackSymbol::Token(name)),
            Some(AstStackSymbol::Token(lct)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::PackDec(
              PackDecStatement::new(pt, name, lct, contents, current_token.clone()),
            )));
            token_index += 1;
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("Unexpected stack contents!!!! pack dec finalize"),
        }
      }
      (Some(AstState::PackDecFinalize), _, _) => panic!("Pack dec finalize mismove"),
      (Some(AstState::TypeVar), Some(AstStackSymbol::PackDecList(_)), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.push(AstState::TypeVarColon);
      }
      (Some(AstState::TypeVar), Some(AstStackSymbol::Type(_)), _) => {
        state_stack.goto(AstState::TypeVarFinalize);
      }
      (Some(AstState::TypeVar), _, _) => panic!("aw crap :("),
      (Some(AstState::TypeVarColon), Some(_), TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::Type);
      }
      (Some(AstState::TypeVarColon), _, _) => {
        panic!("Unexpected token {}!! expected colon", current_token)
      }
      (Some(AstState::TypeVarFinalize), Some(AstStackSymbol::Type(x)), _) => {
        ast_stack.pop();
        let colon = ast_stack.pop_panic();
        let name = ast_stack.pop_panic();
        match (name, colon) {
          (Some(AstStackSymbol::Token(name_token)), Some(AstStackSymbol::Token(colon_token))) => {
            ast_stack.push(AstStackSymbol::TypeVar(TypeVar::new(
              UntypedVar::new(name_token),
              colon_token,
              Box::new(x),
            )));
            state_stack.pop();
          }
          _ => panic!("busted type var finalize stack"),
        }
      }
      (Some(AstState::TypeVarFinalize), _, _) => panic!("type var finalize error"),
      (Some(AstState::Type), _, TokenType::Type) => {
        if current_token.lexeme == "auto" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.goto(AstState::AutoTypeFollow);
        } else if current_token.lexeme == "comp"
          || current_token.lexeme == "lazy"
          || current_token.lexeme == "ref"
          || current_token.lexeme == "optional"
        {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
        } else if current_token.lexeme == "func" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.goto(AstState::FuncTypeParamListStart);
        } else {
          ast_stack.push(AstStackSymbol::Type(Type::Base(BaseType::new(
            current_token.clone(),
          ))));
          token_index += 1;
          state_stack.goto(AstState::BaseTypeFollow)
        }
      }
      (Some(AstState::Type), _, TokenType::LParen) => {
        state_stack.goto(AstState::SubType);
      }
      (Some(AstState::Type), _, TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Type(Type::Base(BaseType::new(
          current_token.clone(),
        ))));
        token_index += 1;
        state_stack.goto(AstState::BaseTypeFollow)
      }
      (Some(AstState::SubType), _, TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::SubType), Some(AstStackSymbol::Type(sub_type)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::SubType), Some(AstStackSymbol::Type(sub_type)), TokenType::RParen) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        match left_paren_sym {
          Some(AstStackSymbol::Token(left_paren)) => {
            ast_stack.push(AstStackSymbol::Type(Type::Sub(SubType::new(
              left_paren,
              Box::new(sub_type),
              current_token.clone(),
            ))));
            token_index += 1;
            token_index = consume_optional_newline(tokens, token_index);
            state_stack.goto(AstState::BaseTypeFollow);
          }
          _ => panic!(":("),
        }
      }
      (Some(AstState::SubType), _, TokenType::Type) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::SubType), _, _) => panic!("aw crap :( subtype"),
      (Some(AstState::AutoTypeFollow), Some(AstStackSymbol::Token(auto_token)), _) => {
        ast_stack.pop();
        if current_token.token_type == TokenType::Identifier {
          ast_stack.push(AstStackSymbol::Type(Type::Auto(AutoType::new(
            auto_token,
            Some(current_token.clone()),
          ))));
          token_index += 1;
        } else {
          ast_stack.push(AstStackSymbol::Type(Type::Auto(AutoType::new(
            auto_token, None,
          ))));
        }
        state_stack.goto(AstState::BaseTypeFollow); // resolve typechain
      }
      (Some(AstState::AutoTypeFollow), _, _) => {
        panic!("I don't understand what happened here in state 11")
      }
      (Some(AstState::BaseTypeFollow), Some(AstStackSymbol::Type(_)), TokenType::LSquare) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::OptType(None));
        token_index += 1;
        state_stack.goto(AstState::BaseTypeFollowEnd);
      }
      (Some(AstState::BaseTypeFollow), Some(AstStackSymbol::Type(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::BaseTypeFollow), Some(AstStackSymbol::Type(_)), _) => {
        state_stack.goto(AstState::TypeChainResolve);
      }
      (Some(AstState::BaseTypeFollow), _, _) => {
        panic!("whoops :(");
      }
      (
        Some(AstState::BaseTypeFollowEnd),
        Some(AstStackSymbol::OptType(opt_type)),
        TokenType::RSquare,
      ) => {
        ast_stack.pop();
        let left_square_sym = ast_stack.pop_panic();
        let base_type_sym = ast_stack.pop_panic();
        match (base_type_sym, left_square_sym) {
          (Some(AstStackSymbol::Type(base_type)), Some(AstStackSymbol::Token(left_square))) => {
            ast_stack.push(AstStackSymbol::Type(Type::Array(ArrayType::new(
              Box::new(base_type),
              left_square,
              Box::new(opt_type),
              current_token.clone(),
            ))));
            token_index += 1;
            state_stack.goto(AstState::TypeChainResolve);
          }
          _ => panic!("busted stack base type follow end"),
        }
      }
      (Some(AstState::BaseTypeFollowEnd), Some(AstStackSymbol::Type(x)), TokenType::RSquare) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::OptType(_)) => {
            ast_stack.pop();
            ast_stack.push(AstStackSymbol::OptType(Some(x)));
          }
          _ => panic!("Unexpected case base type follow end"),
        }
      }
      (Some(AstState::BaseTypeFollowEnd), Some(AstStackSymbol::OptType(x)), TokenType::Type) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::TypeChainResolve), Some(AstStackSymbol::Type(base)), _) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::Token(token)) => match token.token_type {
            TokenType::Type => {
              ast_stack.pop();
              if token.lexeme == "comp" {
                ast_stack.push(AstStackSymbol::Type(Type::Comp(CompType::new(
                  token,
                  Box::new(base),
                ))));
              } else if token.lexeme == "lazy" {
                ast_stack.push(AstStackSymbol::Type(Type::Lazy(LazyType::new(
                  token,
                  Box::new(base),
                ))));
              } else if token.lexeme == "ref" {
                ast_stack.push(AstStackSymbol::Type(Type::Ref(RefType::new(
                  token,
                  Box::new(base),
                ))));
              } else if token.lexeme == "optional" {
                ast_stack.push(AstStackSymbol::Type(Type::Optional(OptionalType::new(
                  token,
                  Box::new(base),
                ))));
              } else {
                panic!("unexpected type token. Expected a type modifier");
              }
            }
            _ => {
              ast_stack.push(AstStackSymbol::Type(base));
              state_stack.pop(); // done
            }
          },
          Some(_) => {
            ast_stack.push(AstStackSymbol::Type(base));
            state_stack.pop(); /* done */
          }
          None => {
            panic!("Stack shouldn't be empty here at state AstState::type chain resolve");
          }
        }
      }
      (Some(AstState::FuncTypeParamListStart), Some(_), TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        state_stack.goto(AstState::FuncTypeParamList);
      }
      (Some(AstState::FuncTypeParamListStart), Some(_), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FuncTypeParamListStart), Some(AstStackSymbol::Token(func_keyword)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(func_keyword, None, Vec::new(), None, Vec::new(), None))));
        state_stack.goto(AstState::BaseTypeFollow);
      }
      (Some(AstState::FuncTypeParamListStart), _, _) => panic!("func type param list start :("),
      (Some(AstState::FuncTypeParamList), Some(AstStackSymbol::TypeList(_)), TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        state_stack.goto(AstState::FuncTypeReturnList);
      }
      (Some(AstState::FuncTypeParamList), Some(AstStackSymbol::TypeList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FuncTypeParamList), Some(AstStackSymbol::TypeList(_)), _) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::FuncTypeParamList), Some(AstStackSymbol::Type(param_type)), _) => {
        ast_stack.pop();
        let type_list_sym = ast_stack.pop_panic();
        match type_list_sym {
          Some(AstStackSymbol::TypeList(mut contents)) => {
            contents.push(Box::new(param_type));
            ast_stack.push(AstStackSymbol::TypeList(contents));
            state_stack.goto(AstState::FuncTypeParamListFollow);
          }
          _ => panic!("busted state stack func type param list after type"),
        }
      }
      (Some(AstState::FuncTypeParamList), _, _) => {
        panic!("unexpected token {} at func type param list", current_token);
      }
      (Some(AstState::FuncTypeParamListFollow), _, TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FuncTypeParamListFollow), _, TokenType::Comma) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FuncTypeParamList);
      }
      (Some(AstState::FuncTypeParamListFollow), _, TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FuncTypeReturnList);
      }
      (Some(AstState::FuncTypeParamListFollow), Some(AstStackSymbol::TypeList(type_list)), TokenType::RParen) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        let func_keyword_sym = ast_stack.pop_panic();
        match (func_keyword_sym, left_paren_sym) {
          (Some(AstStackSymbol::Token(func_keyword)), Some(AstStackSymbol::Token(left_paren))) => {
            ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(func_keyword, Some(left_paren), type_list, None, Vec::new(), Some(current_token.clone())))));
            token_index += 1;
            state_stack.goto(AstState::BaseTypeFollow);
          }
          _ => panic!("wacked out stack")
        }
      }
      (Some(AstState::FuncTypeParamListFollow), _, _) => panic!("unexpected token {} at func type param list follow", current_token),
      (Some(AstState::FuncTypeReturnList), Some(AstStackSymbol::TypeList(_)), TokenType::RParen) => {
        state_stack.goto(AstState::FuncTypeReturnListFollow);
      }
      (Some(AstState::FuncTypeReturnList), Some(AstStackSymbol::TypeList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FuncTypeReturnList), Some(AstStackSymbol::TypeList(_)), _) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::FuncTypeReturnList), Some(AstStackSymbol::Type(param_type)), _) => {
        ast_stack.pop();
        let type_list_sym = ast_stack.pop_panic();
        match type_list_sym {
          Some(AstStackSymbol::TypeList(mut contents)) => {
            contents.push(Box::new(param_type));
            ast_stack.push(AstStackSymbol::TypeList(contents));
            state_stack.goto(AstState::FuncTypeReturnListFollow);
          }
          _ => panic!("busted state stack func type return list after type"),
        }
      }
      (Some(AstState::FuncTypeReturnList), _, _) => {
        panic!("unexpected token {} at func type return list", current_token);
      }
      (Some(AstState::FuncTypeReturnListFollow), _, TokenType::Comma) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FuncTypeReturnList);
      }
      (Some(AstState::FuncTypeReturnListFollow), _, TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FuncTypeReturnListFollow), Some(AstStackSymbol::TypeList(type_list)), TokenType::RParen) => {
        ast_stack.pop();
        let colon_sym = ast_stack.pop_panic();
        let param_type_list_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let func_keyword_sym = ast_stack.pop_panic();
        match (func_keyword_sym, left_paren_sym, param_type_list_sym, colon_sym) {
          (Some(AstStackSymbol::Token(func_keyword)), Some(AstStackSymbol::Token(left_paren)), Some(AstStackSymbol::TypeList(param_types)), Some(AstStackSymbol::Token(colon))) => {
            ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(func_keyword, Some(left_paren), param_types, Some(colon), type_list, Some(current_token.clone())))));
            token_index += 1;
            state_stack.goto(AstState::BaseTypeFollow);
          }
          _ => panic!("wacked out stack")
        }
      }
      (Some(AstState::FuncTypeReturnListFollow), _, _) => panic!("unexpected token {} at func type return list follow", current_token),
      (Some(AstState::UnionDecName), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecStartEntryList);
      }
      (Some(AstState::UnionDecName), _, _) => {
        panic!("Unexpected token {}!! expected identifier", current_token)
      }
      (Some(AstState::UnionDecStartEntryList), Some(_), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::UnionDecList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecEntry);
      }
      (Some(AstState::UnionDecStartEntryList), _, _) => {
        panic!("Unexpected token {}!! expected left curly", current_token)
      }
      (
        Some(AstState::UnionDecEntry),
        Some(AstStackSymbol::UnionDecList(_)),
        TokenType::Identifier,
      ) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::UnionDecStorageStart)
      }
      (Some(AstState::UnionDecEntry), Some(AstStackSymbol::UnionDecList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::UnionDecEntry), Some(AstStackSymbol::UnionDecList(_)), TokenType::RCurly) => {
        state_stack.goto(AstState::UnionDecFinalize);
      }
      (Some(AstState::UnionDecEntry), _, _) => panic!("UnionDecEntry error {}", current_token),
      (Some(AstState::UnionDecStorageStart), Some(_), TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecStorage);
      }
      (Some(AstState::UnionDecStorageStart), Some(AstStackSymbol::Token(dec_entry_id)), TokenType::Comma) |
      (Some(AstState::UnionDecStorageStart), Some(AstStackSymbol::Token(dec_entry_id)), TokenType::Newline) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::UnionDec(UnionDeclaration::new(dec_entry_id, None, Vec::new(), None)));
        state_stack.goto(AstState::UnionDecEntryFinalize);
      }
      (Some(AstState::UnionDecStorageStart), _, _) => panic!("aw crap no paren {}", current_token),
      (Some(AstState::UnionDecStorage), Some(AstStackSymbol::TypeList(_)), TokenType::RParen) => {
        state_stack.goto(AstState::UnionDecStorageFollow);
      }
      (Some(AstState::UnionDecStorage), Some(AstStackSymbol::TypeList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::UnionDecStorage), Some(AstStackSymbol::TypeList(_)), _) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::UnionDecStorage), Some(AstStackSymbol::Type(found_type)), _) => {
        ast_stack.pop();
        let type_list_sym = ast_stack.pop_panic();
        match type_list_sym {
          Some(AstStackSymbol::TypeList(mut contents)) => {
            contents.push(Box::new(found_type));
            ast_stack.push(AstStackSymbol::TypeList(contents));
            state_stack.goto(AstState::UnionDecStorageFollow);
          } 
          _ => panic!("stack bust union dec storage")
        }
      }
      (Some(AstState::UnionDecStorageFollow), Some(AstStackSymbol::TypeList(type_list)), TokenType::RParen) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        let name_sym = ast_stack.pop_panic();
        match (name_sym, left_paren_sym) {
          (Some(AstStackSymbol::Token(name)), Some(AstStackSymbol::Token(left_paren))) => {
            ast_stack.push(AstStackSymbol::UnionDec(UnionDeclaration::new(name, Some(left_paren), type_list, Some(current_token.clone()))));
            token_index += 1;
            state_stack.goto(AstState::UnionDecEntryFinalize);
          }
          _ => panic!("union dec storage follow error")
        }
      }
      (Some(AstState::UnionDecStorageFollow), Some(AstStackSymbol::TypeList(type_list)), TokenType::Comma) => {
        token_index += 1;
        state_stack.goto(AstState::UnionDecStorage);
      }
      (Some(AstState::UnionDecStorageFollow), Some(AstStackSymbol::TypeList(type_list)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecStorage);
      }
      (Some(AstState::UnionDecStorageFollow), _, _) => panic!("whoops :("),
      (
        Some(AstState::UnionDecEntryFinalize),
        Some(AstStackSymbol::UnionDec(entry)),
        TokenType::Comma,
      )
      | (
        Some(AstState::UnionDecEntryFinalize),
        Some(AstStackSymbol::UnionDec(entry)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let union_dec_list_sym = ast_stack.pop_panic();
        match union_dec_list_sym {
          Some(AstStackSymbol::UnionDecList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::UnionDecList(contents));
            state_stack.goto(AstState::UnionDecEntry);
            token_index += 1; //consumes newline or comma
            token_index = consume_optional_newline(tokens, token_index);
          }
          _ => panic!("Union dec finalize has busted stack"),
        }
      }
      (
        Some(AstState::UnionDecEntryFinalize),
        Some(AstStackSymbol::UnionDec(entry)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let union_dec_list_sym = ast_stack.pop_panic();
        match union_dec_list_sym {
          Some(AstStackSymbol::UnionDecList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::UnionDecList(contents));
            state_stack.goto(AstState::UnionDecFinalize);
          }
          _ => panic!("Union dec finalize has busted stack rcurly"),
        }
      }
      (Some(AstState::UnionDecEntryFinalize), _, _) => {
        panic!("unexpected union dec entry finalize {}", current_token)
      }
      (
        Some(AstState::UnionDecFinalize),
        Some(AstStackSymbol::UnionDecList(contents)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let left_curly_token = ast_stack.pop_panic();
        let union_name = ast_stack.pop_panic();
        let union_token = ast_stack.pop_panic();
        match (union_token, union_name, left_curly_token) {
          (
            Some(AstStackSymbol::Token(ut)),
            Some(AstStackSymbol::Token(name)),
            Some(AstStackSymbol::Token(lct)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::UnionDec(
              UnionDecStatement::new(ut, name, lct, contents, current_token.clone()),
            )));
            token_index += 1;
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("Unexpected stack contents!!!! union dec finalize"),
        }
      }
      (Some(AstState::UnionDecFinalize), _, _) => panic!("Pack dec finalize mismove"),
      
      //* EXPRESSION SECTION
      
      (Some(AstState::ExprStmt), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Stmt(Statement::Expression(ExpressionStatement::new(expression))));
        state_stack.goto(AstState::StmtFinalize);
      }
      (Some(AstState::ExprStmt), Some(_), _) => {
        state_stack.push(AstState::Expression);
      } 
      (Some(AstState::ExprStmt), _, _) => {
        panic!("expr stmt error :(");
      }

      (Some(AstState::Expression), _, TokenType::RParen) |
      (Some(AstState::Expression), _, TokenType::RSquare) |
      (Some(AstState::Expression), _, TokenType::Comma) |
      (Some(AstState::Expression), _, TokenType::Newline) => {
        state_stack.pop();
      }

      (Some(AstState::Expression), _, TokenType::LSquare) |
      (Some(AstState::Expression), _, TokenType::Identifier) |
      (Some(AstState::Expression), _, TokenType::Number) |
      (Some(AstState::Expression), _, TokenType::String) |
      (Some(AstState::Expression), _, TokenType::InterpolatedString) |
      (Some(AstState::Expression), _, TokenType::Keyword) => {
        state_stack.push(AstState::Primary);
      }
      (Some(AstState::Expression), _, TokenType::Symbol) => {
        state_stack.push(AstState::Prefix);
      }
      
      (Some(AstState::Primary), _, TokenType::RParen) |
      (Some(AstState::Primary), _, TokenType::RSquare) |
      (Some(AstState::Primary), _, TokenType::Comma) |
      (Some(AstState::Primary), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::Primary), _, TokenType::LSquare) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::ExprList(Vec::new()));
        state_stack.goto(AstState::ArrayLiteralContents);
      }
      (Some(AstState::ArrayLiteralContents), Some(AstStackSymbol::ExprList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::ArrayLiteralContents), Some(AstStackSymbol::ExprList(_)), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::ArrayLiteralContents), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let expr_stmt_sym = ast_stack.pop_panic();
        match expr_stmt_sym {
          Some(AstStackSymbol::ExprList(mut contents)) => {
            contents.push(Box::new(expression));
            ast_stack.push(AstStackSymbol::ExprList(contents));
            state_stack.goto(AstState::ArrayLiteralContentsFollow);
          }
          _ => panic!("Expression list not on stack :(")
        }
      }
      (Some(AstState::ArrayLiteralContents), _, _) => panic!("unexpected token array literal contents"),
      (Some(AstState::ArrayLiteralContentsFollow), Some(AstStackSymbol::ExprList(_)), TokenType::Comma) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::ArrayLiteralContents);
      }
      (Some(AstState::ArrayLiteralContentsFollow), Some(AstStackSymbol::ExprList(_)), TokenType::Newline) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::ArrayLiteralContentsFollow), Some(AstStackSymbol::ExprList(contents)), TokenType::RSquare) => {
        ast_stack.pop();
        let l_square_sym = ast_stack.pop_panic();
        match l_square_sym {
          Some(AstStackSymbol::Token(left_square)) => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Array(ArrayLiteral::new(left_square, contents, current_token.clone())))));
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("panic array literal contents follow stack bad")
        }
      }
      (Some(AstState::ArrayLiteralContentsFollow), x, _) => {
        panic!("unexpected array literal contents follow {}", current_token);
      }
      (Some(AstState::Primary), _, TokenType::Identifier) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Var(UntypedVar::new(current_token.clone()))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::Number) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Number(current_token.clone()))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::InterpolatedString) |
      (Some(AstState::Primary), _, TokenType::String) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::String(current_token.clone()))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::Keyword) => {
        if current_token.lexeme == "true" || current_token.lexeme == "false" {
          token_index += 1;
          ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Boolean(current_token.clone()))));
          state_stack.goto(AstState::PrimaryFollow);
        } else {
          panic!("Did not expected this keyword in an expression {} :(", current_token);
        }
      }
      (Some(AstState::PrimaryFollow), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol) => {
        if current_token.lexeme == "?" {
          ast_stack.pop();
          ast_stack.push(AstStackSymbol::Expr(Expression::Postfix(PostfixExpression::new(Box::new(expression), current_token.clone()))));
          token_index += 1;
          state_stack.pop();
        } else if  current_token.lexeme == "." {
          panic!("member access");
        } else {
          state_stack.pop();
        }
      }
      (Some(AstState::PrimaryFollow), _, TokenType::Dot) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        state_stack.goto(AstState::MemberAccess);
        token_index += 1;
      }
      (Some(AstState::PrimaryFollow), _, TokenType::LSquare) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        state_stack.goto(AstState::ArrayAccess);
        token_index += 1;
      }
      (Some(AstState::PrimaryFollow), _, _) => {
        state_stack.pop();
      }
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare) => {
        ast_stack.pop();
        let lsq_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        match (expr_sym, lsq_sym) {
          (Some(AstStackSymbol::Expr(lhs_expr)), Some(AstStackSymbol::Token(lsq))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::ArrayAccess(ArrayAccess::new(Box::new(lhs_expr), lsq, Box::new(expression), current_token.clone()))));
            token_index += 1;
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("bad array access stack")
        }
      }
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Expr(_)), _) => panic!("aa bad"),
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Expression);
      } 
      (Some(AstState::MemberAccess), Some(AstStackSymbol::Token(dot)), TokenType::Identifier) => {
        ast_stack.pop();
        let expr_sym = ast_stack.pop_panic();
        match expr_sym {
          Some(AstStackSymbol::Expr(expression)) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Member(MemberAccess::new(Box::new(expression), dot, current_token.clone()))));
            token_index += 1;
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("could not find an expression to match the member access"),
        }
      }
      (Some(AstState::MemberAccess), _, _) => {
        panic!("unexpected token in member access")
      }
      (Some(AstState::Prefix), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        match operator_sym {
          Some(AstStackSymbol::Token(operator)) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Prefix(PrefixExpression::new(operator, Box::new(expression)))));
            state_stack.pop();
          }
          _ => panic!("Did not find prefix operator")
        }
      }
      (Some(AstState::Prefix), _, TokenType::Symbol) => {
        if current_token.lexeme == "~" || current_token.lexeme == "!" || current_token.lexeme == "-" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::PrefixContents);
        } else {
          panic!("Unexpected symbol {}", current_token);
        }
      }
      
      (Some(AstState::PrefixContents), _, TokenType::Symbol) => {
        state_stack.goto(AstState::Prefix);
      }
      (Some(AstState::PrefixContents), _, _) => {
        state_stack.goto(AstState::Primary);
      }

      //* END EXPRESSION SECTION

      (_, _, _) => {
        panic!("Unknown case :( {}", current_token);
      }
    };
  }

  match ast_stack.pop() {
    Some(x) => match x {
      AstStackSymbol::Program(program) => (Some(program), errors),
      _ => (None, errors),
    },
    None => (None, errors),
  }
}
