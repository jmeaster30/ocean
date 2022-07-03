pub mod ast;
pub mod display;
pub mod span;

use crate::compiler::OceanError;
use crate::compiler::{Token, TokenType};
use ast::*;

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
  ParamList(ParameterList),
  Param(Parameter),
  ReturnList(ReturnList),
  ReturnEntry(ReturnEntry),
  IdList(Vec<Token>),
  ExprList(Vec<Expression>),
}

#[derive(Clone, Copy)]
pub enum AstState {
  StmtList,
  FinishStmt,
  StartUseIdList,
  FollowUseIdList,
  UseStmtAlias,
  FinalizeUseStmt,
  StartPackDecStmtName,
  StartPackDecStmtBody,
  StartPackDecEntry,
  PackDecColon,
  StartPackDecType,
  PackDecAutoTypeFollow,
  PackDecTypeResolveTypeChain,
  FinalizePackDecEntryTypeVar,
  PackDecFollow,
  FinishPackDecEntry,
}

pub struct AstStack {
  stack: Vec<AstStackSymbol>,
}

impl AstStack {
  fn new() -> Self {
    AstStack { stack: Vec::new() }
  }

  fn peek(&self) -> Option<AstStackSymbol> {
    if self.stack.is_empty() {
      None
    } else {
      Some(self.stack[self.stack.len() - 1].clone())
    }
  }

  fn push(&mut self, symbol: AstStackSymbol) {
    self.stack.push(symbol);
  }

  fn pop(&mut self) -> Option<AstStackSymbol> {
    if !self.stack.is_empty() {
      self.stack.pop()
    } else {
      None
    }
  }

  fn pop_panic(&mut self) -> Option<AstStackSymbol> {
    if self.stack.is_empty() {
      panic!("Ah crap I tried to pop an empty stack :(");
    }
    self.stack.pop()
  }
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
  let mut state = AstState::StmtList;
  let mut token_index = 0;
  for token in tokens {
    println!("{}", token);
  }

  println!("Start parse");

  ast_stack.push(AstStackSymbol::StmtList(Vec::new()));

  loop {
    let current_token = &tokens[token_index];
    let stack_top = ast_stack.peek();
    match (state, stack_top, &current_token.token_type) {
      // PARSE MAIN STATEMENT LIST
      (AstState::StmtList, Some(AstStackSymbol::StmtList(contents)), TokenType::EndOfInput) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Program(Program::new(contents.to_vec())));
        break;
      }
      // FIND STATEMENTS THAT START WITH KEYWORDS
      (AstState::StmtList, Some(_), TokenType::Keyword) => {
        if current_token.lexeme == "use" {
          println!("found use!!!");
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          ast_stack.push(AstStackSymbol::IdList(Vec::new()));
          token_index += 1;
          state = AstState::StartUseIdList;
        } else if current_token.lexeme == "break" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Break(BreakStatement::new(
            current_token.clone(),
          ))));
          token_index += 1;
          state = AstState::FinishStmt;
        } else if current_token.lexeme == "continue" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Continue(
            ContinueStatement::new(current_token.clone()),
          )));
          token_index += 1;
          state = AstState::FinishStmt;
        } else if current_token.lexeme == "return" {
          ast_stack.push(AstStackSymbol::Stmt(Statement::Return(
            ReturnStatement::new(current_token.clone()),
          )));
          token_index += 1;
          state = AstState::FinishStmt;
        } else if current_token.lexeme == "enum" {
          panic!("i don't know what an enum is");
        } else if current_token.lexeme == "pack" {
          println!("Found pack :)");
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state = AstState::StartPackDecStmtName;
        } else {
          panic!("I am so scared about this keyword :O");
        }
      }
      // Consume newline tokens until next statement start
      (AstState::StmtList, Some(_), TokenType::Newline) => token_index += 1,
      (AstState::StmtList, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::StmtList", current_token);
      }
      // CONSUME END OF STATEMENT
      (AstState::FinishStmt, Some(AstStackSymbol::Stmt(stmt)), TokenType::Newline) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::StmtList(mut stmt_list)) => {
            ast_stack.pop();
            stmt_list.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(stmt_list));
            state = AstState::StmtList;
          }
          _ => panic!("ah crap when adding statement to statement list"),
        }
        token_index += 1;
      }
      (AstState::FinishStmt, Some(AstStackSymbol::Stmt(stmt)), TokenType::EndOfInput) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::StmtList(mut stmt_list)) => {
            ast_stack.pop();
            stmt_list.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(stmt_list));
            state = AstState::StmtList;
          }
          _ => panic!("ah crap when adding statement to statement list eoi"),
        }
      }
      (AstState::FinishStmt, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::FinishStmt", current_token);
      }
      // CONSUME IDENTIFIER LIST FOR USE STATEMENT
      (AstState::StartUseIdList, Some(AstStackSymbol::IdList(mut contents)), TokenType::Identifier) => {
        ast_stack.pop();
        contents.push(current_token.clone());
        ast_stack.push(AstStackSymbol::IdList(contents));
        token_index += 1;
        state = AstState::FollowUseIdList;
      }
      (AstState::StartUseIdList, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::StartUseIdList", current_token);
      }
      (AstState::FollowUseIdList, Some(AstStackSymbol::IdList(_)), TokenType::Dot) => {
        token_index += 1;
        state = AstState::StartUseIdList;
      }
      (AstState::FollowUseIdList, Some(AstStackSymbol::IdList(_)), TokenType::Keyword) => {
        if current_token.lexeme == "as" {
          println!("found as!!!");
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state = AstState::UseStmtAlias;
        } else {
          panic!("unexpected keyword, expected 'as'"); // TODO ERROR
        }
      }
      (AstState::FollowUseIdList, Some(AstStackSymbol::IdList(contents)), _) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::Token(use_token)) => {
            ast_stack.pop();
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(
              use_token, contents, None, None,
            ))));
            state = AstState::FinishStmt;
          }
          _ => panic!("end use stmt next token newline"), // TODO ERROR
        }
      }
      (AstState::FollowUseIdList, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::FollowUseIdList", current_token);
      }
      // PARSE USE STATEMENT ALIAS
      (AstState::UseStmtAlias, Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state = AstState::FinalizeUseStmt;
      }
      (AstState::UseStmtAlias, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::UseStmtAlias", current_token);
      }
      (AstState::FinalizeUseStmt, Some(_), _) => {
        // build up use statement
        let alias_token = ast_stack.pop_panic();
        let as_token = ast_stack.pop_panic();
        let id_list = ast_stack.pop_panic();
        let use_token = ast_stack.pop_panic();
        match (use_token, id_list, as_token, alias_token) {
          (
            Some(AstStackSymbol::Token(ut)),
            Some(AstStackSymbol::IdList(idl)),
            Some(AstStackSymbol::Token(astk)),
            Some(AstStackSymbol::Token(alt)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(
              ut,
              idl,
              Some(astk),
              Some(alt),
            ))));
            state = AstState::FinishStmt;
          }
          _ => panic!("AAAAAAAAAAAAAAAAA end aliased use stmt stack busted"),
        }
      }
      (AstState::FinalizeUseStmt, _, _) => {
        panic!("Unexpected token {}!!!!! state AstState::UseStmtAlias", current_token);
      }
      // DONE USE STATEMENT
      // PACK DEC STATEMENT IDENTIFIER
      (AstState::StartPackDecStmtName, Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state = AstState::StartPackDecStmtBody;
      }
      (AstState::StartPackDecStmtName, _, _) => {
        panic!(
          "Unexpected token {} expected identifier",
          current_token.clone()
        );
      }
      (AstState::StartPackDecStmtBody, Some(_), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::PackDecList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state = AstState::StartPackDecEntry;
      }
      (AstState::StartPackDecEntry, Some(AstStackSymbol::PackDecList(_)), TokenType::RCurly) => {
        let pdl = ast_stack.pop_panic();
        let left_curly = ast_stack.pop_panic();
        let name = ast_stack.pop_panic();
        let pack_token = ast_stack.pop_panic();
        match (pack_token, name, left_curly, pdl) {
          (
            Some(AstStackSymbol::Token(pt)),
            Some(AstStackSymbol::Token(n)),
            Some(AstStackSymbol::Token(lc)),
            Some(AstStackSymbol::PackDecList(contents)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::PackDec(
              PackDecStatement::new(pt, n, lc, contents, current_token.clone()),
            )));
            token_index += 1;
            state = AstState::FinishStmt;
          }
          (_, _, _, _) => panic!("pack dec stack busted"),
        }
      }
      (AstState::StartPackDecEntry, Some(AstStackSymbol::PackDecList(_)), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state = AstState::PackDecColon;
      }
      (AstState::StartPackDecEntry, Some(AstStackSymbol::PackDecList(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (AstState::StartPackDecEntry, _, _) => {
        panic!("Unexpected token {} expected identifier or right curly", current_token);
      }
      (AstState::PackDecColon, Some(_), TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state = AstState::StartPackDecType;
      }
      // pack dec entry type parsing!!!!!!!!!!!!!!!!!
      (AstState::StartPackDecType, Some(_), TokenType::Type) => {
        if current_token.lexeme == "auto" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state = AstState::PackDecAutoTypeFollow;
        } else if current_token.lexeme == "func" {
          panic!("function type parsing not done :(");
        } else if current_token.lexeme == "comp"
          || current_token.lexeme == "lazy"
          || current_token.lexeme == "ref"
          || current_token.lexeme == "optional"
        {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
        } else {
          //base type
          ast_stack.push(AstStackSymbol::Type(Type::Base(BaseType::new(
            current_token.clone(),
          ))));
          token_index += 1;
          state = AstState::PackDecTypeResolveTypeChain;
          // TODO add new state for array types
        }
      }
      (AstState::StartPackDecType, Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Type(Type::Base(BaseType::new(
          current_token.clone(),
        ))));
        token_index += 1;
        state = AstState::PackDecTypeResolveTypeChain;
        // TODO add new state for array types
      }
      (AstState::StartPackDecType, _, _) => {
        panic!("Unexpected token {} expected type or identifier", current_token);
      }
      // auto type
      (AstState::PackDecAutoTypeFollow, Some(AstStackSymbol::Token(auto_token)), _) => {
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
        state = AstState::PackDecTypeResolveTypeChain; // resolve typechain
      }
      (AstState::PackDecAutoTypeFollow, _, _) => panic!("I don't understand what happened here in state 11"),
      // type modifier
      (AstState::PackDecTypeResolveTypeChain, Some(AstStackSymbol::Type(base)), _) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::Token(token)) => match token.token_type {
            TokenType::Type => {
              ast_stack.pop();
              if token.lexeme == "comp" {
                ast_stack.push(AstStackSymbol::Type(Type::Comp(CompType::new(token, Box::new(base)))));
              } else if token.lexeme == "lazy" {
                ast_stack.push(AstStackSymbol::Type(Type::Lazy(LazyType::new(token, Box::new(base)))));
              } else if token.lexeme == "ref" {
                ast_stack.push(AstStackSymbol::Type(Type::Ref(RefType::new(token, Box::new(base)))));
              } else if token.lexeme == "optional" {
                ast_stack.push(AstStackSymbol::Type(Type::Optional(OptionalType::new(
                  token, Box::new(base),
                ))));
              } else {
                panic!("unexpected type token. Expected a type modifier");
              }
            }
            _ => {
              ast_stack.push(AstStackSymbol::Type(base));
              state = AstState::FinalizePackDecEntryTypeVar; // done
            }
          },
          Some(_) => {
            ast_stack.push(AstStackSymbol::Type(base));
            state = AstState::FinalizePackDecEntryTypeVar; /* done */
          }
          None => {
            panic!("Stack shouldn't be empty here at state AstState::PackDecTypeResolveTypeChain");
          }
        }
      }
      (AstState::FinalizePackDecEntryTypeVar, Some(AstStackSymbol::Type(found_type)), _) => {
        ast_stack.pop();
        let colon = ast_stack.pop_panic();
        let name = ast_stack.pop_panic();
        match (name, colon) {
          (Some(AstStackSymbol::Token(n)), Some(AstStackSymbol::Token(c))) => {
            if n.token_type == TokenType::Identifier && c.token_type == TokenType::Colon {
              ast_stack.push(AstStackSymbol::TypeVar(TypeVar::new(
                UntypedVar::new(n),
                c,
                Box::new(found_type),
              )));
            } else {
              panic!("Unexpected stack layout 2 :( state AstState::FinalizePackDecEntryTypeVar");
            }
          }
          _ => panic!("unexpected stack layout :( state AstState::FinalizePackDecEntryTypeVar"),
        }
        state = AstState::PackDecFollow; // go to potential default assignment
      }
      (AstState::FinalizePackDecEntryTypeVar, _, _) => panic!("idk why i am here finalize pack dec entry type var {}", current_token),
      (AstState::PackDecFollow, Some(AstStackSymbol::TypeVar(type_var)), TokenType::Newline)
      | (AstState::PackDecFollow, Some(AstStackSymbol::TypeVar(type_var)), TokenType::RCurly) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::PackDec(PackDeclaration::new(
          type_var, None, None,
        )));
        state = AstState::FinishPackDecEntry; // combine pack dec into the pack dec list
      }
      (AstState::PackDecFollow, _, _) => { panic!("Unexpected token {}. expected newline or right curly or default assignment", current_token); }
      //Finalize pack dec list
      (AstState::FinishPackDecEntry, Some(AstStackSymbol::PackDec(pack_dec)), _) => {
        ast_stack.pop();
        let pack_dec_list = ast_stack.pop_panic();
        match pack_dec_list {
          Some(AstStackSymbol::PackDecList(mut contents)) => {
            contents.push(pack_dec);
            ast_stack.push(AstStackSymbol::PackDecList(contents));
            token_index = consume_optional_newline(tokens, token_index);
            state = AstState::StartPackDecEntry;
          }
          _ => panic!("Unexpected stack layout :( finalize pack dec list"),
        }
      }
      (AstState::FinishPackDecEntry, _, _) => panic!("aw crap:("),
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
