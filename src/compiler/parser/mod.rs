pub mod ast;
pub mod display;
pub mod helpers;
pub mod span;

use crate::compiler::OceanError;
use crate::compiler::{Token, TokenType};
use ast::*;
use helpers::*;

#[derive(Clone)]
pub enum AstStackSymbol {
  Token(Token),
  Program(Program),
  StmtList(Vec<Statement>),
  Stmt(Statement),
  PackDec(PackDeclaration),
  PackDecList(Vec<PackDeclaration>),
  EnumDec(EnumDeclaration),
  EnumStore(EnumStorage),
  MatchEntry(MatchEntry),
  Expr(Expression),
  Literal(Literal),
  TypeVar(TypeVar),
  Var(Var),
  Type(Type),
  OptType(Option<Type>),
  ParamList(ParameterList),
  Param(Parameter),
  ReturnList(ReturnList),
  ReturnEntry(ReturnEntry),
  IdList(Vec<Token>),
  ExprList(Vec<Expression>),
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
  SubType,
}

fn consume_optional_newline(tokens: &Vec<Token>, token_index: usize) -> usize {
  let mut new_token_index = token_index;
  while tokens[new_token_index].token_type == TokenType::Newline {
    new_token_index += 1;
  }
  new_token_index
}

pub fn parse(tokens: &Vec<Token>) -> (Option<Program>, Vec<OceanError>) {
  let mut ast_stack = AstStack::new();
  let mut errors: Vec<OceanError> = Vec::new();
  let mut state_stack = StateStack::new();
  let mut token_index = 0;
  for token in tokens {
    println!("{}", token);
  }

  println!("Start parse");

  ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
  state_stack.goto(AstState::StmtList);

  loop {
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
        } else {
          panic!("Unknown keyword {} :(", current_token);
        }
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
        TokenType::Newline,
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
          panic!("function type not done :(")
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
      (Some(AstState::SubType), _, TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
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
      (_, _, _) => {
        panic!("Unknown case :(");
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
