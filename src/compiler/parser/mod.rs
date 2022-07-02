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

/* States
 0 - statement list
 1 - statement \n
 2..5 - use statement
*/

pub fn parse(tokens: &Vec<Token>) -> (Option<Program>, Vec<OceanError>) {
  let mut ast_stack = AstStack::new();
  let mut errors: Vec<OceanError> = Vec::new();
  let mut state = 0;
  let mut token_index = 0;
  for token in tokens {
    println!("{}", token);
  }

  println!("Start parse");

  ast_stack.push(AstStackSymbol::StmtList(Vec::new()));

  loop {
    let current_token = &tokens[token_index];
    let stack_top = ast_stack.peek();
    match (
      state,
      stack_top,
      &current_token.token_type
    ) {
      // PARSE MAIN STATEMENT LIST
      (0, Some(AstStackSymbol::StmtList(contents)), TokenType::EndOfInput) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Program(Program::new(contents.to_vec())));
        break;
      }
      // FIND STATEMENTS THAT START WITH KEYWORDS
      (0, Some(_), TokenType::Keyword) => {
        if current_token.lexeme == "use" {
          println!("found use!!!");
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          ast_stack.push(AstStackSymbol::IdList(Vec::new()));
          token_index += 1;
          state = 2;
        } else {
          panic!("I am so scared about this keyword :O");
        }
      }
      (0, _, _) => { panic!("Unexpected token {}!!!!! state 0", current_token); }
      // CONSUME END OF STATEMENT
      (1, Some(AstStackSymbol::Stmt(stmt)), TokenType::Newline) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::StmtList(mut stmt_list)) => {
            ast_stack.pop();
            stmt_list.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(stmt_list));
            state = 0;
          },
          _ => panic!("ah crap when adding statement to statement list")
        }
        token_index += 1;
      }
      (1, Some(AstStackSymbol::Stmt(stmt)), TokenType::EndOfInput) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::StmtList(mut stmt_list)) => {
            ast_stack.pop();
            stmt_list.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(stmt_list));
            state = 0;
          },
          _ => panic!("ah crap when adding statement to statement list eoi")
        }
      }
      (1, _, _) => { panic!("Unexpected token {}!!!!! state 1", current_token); }
      // CONSUME IDENTIFIER LIST FOR USE STATEMENT
      (2, Some(AstStackSymbol::IdList(mut contents)), TokenType::Identifier) => {
        ast_stack.pop();
        contents.push(current_token.clone());
        ast_stack.push(AstStackSymbol::IdList(contents));
        token_index += 1;
        state = 3;
      }
      (2, _, _) => { panic!("Unexpected token {}!!!!! state 2", current_token); }
      (3, Some(AstStackSymbol::IdList(_)), TokenType::Dot) => {
        token_index += 1;
        state = 2;
      },
      (3, Some(AstStackSymbol::IdList(contents)), TokenType::Newline) |
      (3, Some(AstStackSymbol::IdList(contents)), TokenType::EndOfInput) => {
        ast_stack.pop();
        match ast_stack.peek() {
          Some(AstStackSymbol::Token(use_token)) => {
            ast_stack.pop();
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(use_token, contents, None, None))));
            state = 1;
          },
          _ => panic!("end use stmt next token newline") // TODO ERROR
        }
      },
      // FOUND END OF IDENTIFIER LIST USE STMT
      (3, Some(_), TokenType::Keyword) => {
        if current_token.lexeme == "as" {
          println!("found as!!!");
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state = 4;
        } else {
          panic!("unexpected keyword, expected 'as'"); // TODO ERROR
        }
      },
      (3, _, _) => { panic!("Unexpected token {}!!!!! state 3", current_token); }
      // PARSE USE STATEMENT ALIAS
      (4, Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state = 5;
      }
      (4, _, _) => { panic!("Unexpected token {}!!!!! state 4", current_token); }
      (5, Some(_), TokenType::EndOfInput) |
      (5, Some(_), TokenType::Newline) => {
        // build up use statement
        let alias_token = ast_stack.pop_panic();
        let as_token = ast_stack.pop_panic();
        let id_list = ast_stack.pop_panic();
        let use_token = ast_stack.pop_panic();
        match (use_token, id_list, as_token, alias_token) {
          (Some(AstStackSymbol::Token(ut)), Some(AstStackSymbol::IdList(idl)), Some(AstStackSymbol::Token(astk)), Some(AstStackSymbol::Token(alt))) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::Use(UseStatement::new(ut, idl, Some(astk), Some(alt)))));
            state = 1;
          }
          _ => panic!("AAAAAAAAAAAAAAAAA end aliased use stmt stack busted")
        }
      }
      (5, _, _) => { panic!("Unexpected token {}!!!!! state 5", current_token); }
      // DONE USE STATEMENT
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
