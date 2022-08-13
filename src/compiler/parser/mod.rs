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
  StmtList(Vec<Statement>),
  Stmt(Statement),
  PackDec(PackDeclaration),
  PackDecList(Vec<PackDeclaration>),
  UnionDec(UnionDeclaration),
  UnionDecList(Vec<UnionDeclaration>),
  Expr(Expression),
  ExprList(Vec<Box<Expression>>),
  TypeVar(TypeVar),
  Var(Var),
  Type(Type),
  TypeList(Vec<Box<Type>>),
  OptType(Option<Type>),
  ParamList(Vec<Parameter>),
  ReturnList(Vec<ReturnEntry>),
  ReturnEntry(ReturnEntry),
  IdList(Vec<Token>),
  TupleEntry(TupleEntry),
  TupleEntryList(Vec<TupleEntry>),
}

#[derive(Debug, Clone, Copy)]
pub enum AstState {
  StmtList,
  StmtFinalize,
  Error,

  UseStmtIdList,
  UseStmtIdListFollow,
  UseStmtAlias,

  PackDecName,
  PackDecStartEntryList,
  PackDecEntry,
  PackDecEntryFinalize,
  PackDecEntryExpression,
  PackDecFinalize,

  Var,
  VarFollow,
  TypeVar,
  TypeVarColon,
  TypeVarFinalize,

  Type,
  BaseTypeFollow,
  BaseTypeFollowEnd,
  TypeChainResolve,
  TypeFinalize,
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
  UnionDecEntryFinalize,
  UnionDecFinalize,

  LetStmt,
  LetVar,
  LetExpression,

  InfiniteLoopStmt,
  InfiniteLoopBody,

  WhileLoopStmt,
  WhileLoopBody,

  ForLoopStmt,
  ForLoopIterator,
  ForLoopExpression,
  ForLoopBody,

  IfStmt,
  IfStmtBody,
  IfStmtFollow,
  IfStmtElseBody,
  IfStmtFinalize,

  CastDecStmt,
  CastDecExpr,

  ExprStmt,
  Expression,
  SubExpression,
  ArrayLiteralContents,
  ArrayLiteralContentsFollow,
  PrefixOrPrimary,
  Primary,
  PrimaryFollow,
  FunctionCall,
  CastExpr,
  MemberAccess,
  ArrayAccess,
  Prefix,
  PrefixContents,

  TupleStart,
  TupleContents,
  TupleEntry,
  TupleEntryFollow,
  TupleEntryNamedEnd,
  TupleEntryUnnamedEnd,
  TupleEnd,

  FunctionPrimary,
  FunctionParameters,
  FunctionArrow,
  FunctionReturns,
  FunctionReturnEntry,
  FunctionSignatureFollow,
  FunctionBody,

  ExpressionFollow,
  ExpressionFold,
  EqualityFollow,
  EqualityFold,
  ComparisonFollow,
  ComparisonFold,
  ShiftFollow,
  ShiftFold,
  LogicalFollow,
  LogicalFold,
  BitwiseFollow,
  BitwiseFold,
  AdditiveFollow,
  AdditiveFold,
  MultiplicativeFollow,
  MultiplicativeFold,
  ArrayFollow,
  ArrayFold,
  RangeFollow,
  RangeFold,
  DefaultFollow,
  DefaultFold,
}

fn consume_optional_newline(tokens: &Vec<Token>, token_index: usize) -> usize {
  let mut new_token_index = token_index;
  while tokens[new_token_index].token_type == TokenType::Newline {
    new_token_index += 1;
  }
  new_token_index
}

pub fn parse(
  tokens: &Vec<Token>,
  start_index: Option<usize>,
) -> (Option<Program>, Vec<OceanError>) {
  let mut ast_stack = Stack::new();
  let mut errors: Vec<OceanError> = Vec::new();
  let mut state_stack = StateStack::new();
  let mut token_index = match start_index {
    Some(x) => x,
    None => 0,
  };

  //println!("Start parse");

  ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
  state_stack.goto(AstState::StmtList);

  loop {
    ast_stack.print();
    state_stack.print();
    let current_token = &tokens[token_index];
    println!("CURRENT TOKEN: {:?}", current_token.clone());
    let stack_top = ast_stack.peek();
    let state = state_stack.current_state();
    match (state, stack_top, &current_token.token_type) {
      (Some(AstState::StmtList), Some(AstStackSymbol::StmtList(_)), TokenType::EndOfInput) => {
        break;
      }
      (Some(AstState::StmtList), Some(AstStackSymbol::StmtList(_)), TokenType::RCurly) => {
        state_stack.pop();
      }
      (Some(AstState::StmtList), Some(_), TokenType::Macro) => {
        ast_stack.push(AstStackSymbol::Stmt(Statement::Macro(MacroStatement::new(
          current_token.clone(),
        ))));
        token_index += 1;
        state_stack.push(AstState::StmtFinalize);
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
        } else if current_token.lexeme == "let" {
          state_stack.push(AstState::LetStmt);
        } else if current_token.lexeme == "cast" {
          state_stack.push(AstState::CastDecStmt);
        } else if current_token.lexeme == "loop" {
          state_stack.push(AstState::InfiniteLoopStmt);
        } else if current_token.lexeme == "while" {
          state_stack.push(AstState::WhileLoopStmt);
        } else if current_token.lexeme == "for" {
          state_stack.push(AstState::ForLoopStmt);
        } else if current_token.lexeme == "if" {
          state_stack.push(AstState::StmtFinalize);
          state_stack.push(AstState::IfStmt);
        } else {
          panic!("Unknown keyword {} :(", current_token);
        }
      }
      (Some(AstState::StmtList), Some(_), TokenType::LSquare)
      | (Some(AstState::StmtList), Some(_), TokenType::LParen)
      | (Some(AstState::StmtList), Some(_), TokenType::Symbol)
      | (Some(AstState::StmtList), Some(_), TokenType::Number)
      | (Some(AstState::StmtList), Some(_), TokenType::Identifier)
      | (Some(AstState::StmtList), Some(_), TokenType::InterpolatedString)
      | (Some(AstState::StmtList), Some(_), TokenType::String) => {
        state_stack.push(AstState::ExprStmt);
      }
      (Some(AstState::StmtList), Some(_), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::StmtFinalize), Some(AstStackSymbol::Stmt(stmt)), _) => {
        ast_stack.pop();
        if current_token.token_type == TokenType::Newline {
          token_index += 1;
        }
        let stmt_list_sym = ast_stack.pop_panic();
        match stmt_list_sym {
          Some(AstStackSymbol::StmtList(mut contents)) => {
            contents.push(stmt);
            ast_stack.push(AstStackSymbol::StmtList(contents));
            state_stack.pop();
          }
          _ => panic!("Stmt finalize stack busted"),
        }
      }
      (Some(AstState::InfiniteLoopStmt), _, TokenType::Keyword) => {
        if current_token.lexeme == "loop" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::InfiniteLoopBody);
          token_index += 1;
          token_index = consume_optional_newline(tokens, token_index);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::InfiniteLoopBody), _, TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        state_stack.push(AstState::StmtList);
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::InfiniteLoopBody),
        Some(AstStackSymbol::StmtList(stmts)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let loop_sym = ast_stack.pop_panic();
        match (loop_sym, left_curly_sym) {
          (Some(AstStackSymbol::Token(loop_token)), Some(AstStackSymbol::Token(left_curly))) => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Stmt(Statement::InfiniteLoop(
              InfiniteLoopStatement::new(loop_token, left_curly, stmts, current_token.clone()),
            )));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("bad stack infinite loop body"),
        }
      }
      (Some(AstState::WhileLoopStmt), Some(AstStackSymbol::Expr(_)), _) => {
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::WhileLoopBody);
      }
      (Some(AstState::WhileLoopStmt), _, TokenType::Keyword) => {
        if current_token.lexeme == "while" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.push(AstState::Expression);
          token_index += 1;
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::WhileLoopBody), _, TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        state_stack.push(AstState::StmtList);
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::WhileLoopBody), Some(AstStackSymbol::StmtList(stmts)), TokenType::RCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let while_expr_sym = ast_stack.pop_panic();
        let loop_sym = ast_stack.pop_panic();
        match (loop_sym, while_expr_sym, left_curly_sym) {
          (
            Some(AstStackSymbol::Token(loop_token)),
            Some(AstStackSymbol::Expr(expression)),
            Some(AstStackSymbol::Token(left_curly)),
          ) => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Stmt(Statement::WhileLoop(
              WhileStatement::new(
                loop_token,
                expression,
                left_curly,
                stmts,
                current_token.clone(),
              ),
            )));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("bad stack infinite loop body"),
        }
      }
      (Some(AstState::ForLoopStmt), _, TokenType::Keyword) => {
        if current_token.lexeme == "for" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::ForLoopIterator);
          token_index += 1;
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::ForLoopIterator), _, TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        state_stack.goto(AstState::ForLoopExpression);
        token_index += 1;
      }
      (Some(AstState::ForLoopExpression), Some(AstStackSymbol::Expr(_)), _) => {
        state_stack.goto(AstState::ForLoopBody);
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::ForLoopExpression), _, TokenType::Keyword) => {
        if current_token.lexeme == "in" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::Expression);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::ForLoopBody), _, TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        state_stack.push(AstState::StmtList);
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::ForLoopBody), Some(AstStackSymbol::StmtList(stmts)), TokenType::RCurly) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let loop_expr_sym = ast_stack.pop_panic();
        let in_token_sym = ast_stack.pop_panic();
        let iter_token_sym = ast_stack.pop_panic();
        let loop_sym = ast_stack.pop_panic();
        match (
          loop_sym,
          iter_token_sym,
          in_token_sym,
          loop_expr_sym,
          left_curly_sym,
        ) {
          (
            Some(AstStackSymbol::Token(loop_token)),
            Some(AstStackSymbol::Token(iter_token)),
            Some(AstStackSymbol::Token(in_token)),
            Some(AstStackSymbol::Expr(loop_expr)),
            Some(AstStackSymbol::Token(left_curly)),
          ) => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Stmt(Statement::ForLoop(
              ForLoopStatement::new(
                loop_token,
                iter_token,
                in_token,
                loop_expr,
                left_curly,
                stmts,
                current_token.clone(),
              ),
            )));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("bad stack infinite loop body"),
        }
      }
      (Some(AstState::IfStmt), Some(AstStackSymbol::Expr(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::IfStmt), Some(AstStackSymbol::Expr(_)), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::IfStmtBody);
      }
      (Some(AstState::IfStmt), Some(_), TokenType::Keyword) => {
        if current_token.lexeme == "if" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::Expression);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::IfStmtBody), Some(AstStackSymbol::StmtList(_)), TokenType::RCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::IfStmtFollow);
      }
      (Some(AstState::IfStmtBody), Some(_), _) => {
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        state_stack.push(AstState::StmtList);
      }
      (Some(AstState::IfStmtFollow), Some(AstStackSymbol::Token(_)), TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::IfStmtFollow),
        Some(AstStackSymbol::Token(right_curly)),
        TokenType::Keyword,
      ) => {
        if current_token.lexeme == "else" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          token_index = consume_optional_newline(tokens, token_index);
          state_stack.goto(AstState::IfStmtElseBody);
        } else {
          ast_stack.pop();
          let stmt_list_sym = ast_stack.pop_panic();
          let left_curly_sym = ast_stack.pop_panic();
          let expr_sym = ast_stack.pop_panic();
          let if_sym = ast_stack.pop_panic();
          match (if_sym, expr_sym, left_curly_sym, stmt_list_sym) {
            (
              Some(AstStackSymbol::Token(if_token)),
              Some(AstStackSymbol::Expr(expression)),
              Some(AstStackSymbol::Token(left_curly)),
              Some(AstStackSymbol::StmtList(body)),
            ) => {
              ast_stack.push(AstStackSymbol::Stmt(Statement::If(IfStatement::new(
                if_token,
                expression,
                left_curly,
                body,
                right_curly,
                None,
                None,
                Vec::new(),
                None,
              ))));
              state_stack.pop();
            }
            _ => panic!(),
          }
        }
      }
      (Some(AstState::IfStmtFollow), Some(AstStackSymbol::Token(right_curly)), _) => {
        ast_stack.pop();
        let stmt_list_sym = ast_stack.pop_panic();
        let left_curly_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        let if_sym = ast_stack.pop_panic();
        match (if_sym, expr_sym, left_curly_sym, stmt_list_sym) {
          (
            Some(AstStackSymbol::Token(if_token)),
            Some(AstStackSymbol::Expr(expression)),
            Some(AstStackSymbol::Token(left_curly)),
            Some(AstStackSymbol::StmtList(body)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::If(IfStatement::new(
              if_token,
              expression,
              left_curly,
              body,
              right_curly,
              None,
              None,
              Vec::new(),
              None,
            ))));
            state_stack.pop();
          }
          _ => panic!(),
        }
      }
      (Some(AstState::IfStmtElseBody), Some(AstStackSymbol::Token(_)), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::IfStmtFinalize);
        state_stack.push(AstState::StmtList);
      }
      (Some(AstState::IfStmtElseBody), Some(AstStackSymbol::Token(_)), TokenType::Keyword) => {
        if current_token.lexeme == "if" {
          state_stack.push(AstState::IfStmt);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::IfStmtElseBody), Some(AstStackSymbol::Stmt(statement)), _) => {
        ast_stack.pop();
        let else_sym = ast_stack.pop_panic();
        let right_curly_sym = ast_stack.pop_panic();
        let body_sym = ast_stack.pop_panic();
        let left_curly_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        let if_sym = ast_stack.pop_panic();
        match (
          if_sym,
          expr_sym,
          left_curly_sym,
          body_sym,
          right_curly_sym,
          else_sym,
        ) {
          (
            Some(AstStackSymbol::Token(if_token)),
            Some(AstStackSymbol::Expr(expression)),
            Some(AstStackSymbol::Token(left_curly)),
            Some(AstStackSymbol::StmtList(body)),
            Some(AstStackSymbol::Token(right_curly)),
            Some(AstStackSymbol::Token(else_token)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::If(IfStatement::new(
              if_token,
              expression,
              left_curly,
              body,
              right_curly,
              Some(else_token),
              None,
              vec![statement],
              None,
            ))));
            state_stack.pop();
          }
          _ => panic!(),
        }
      }
      (
        Some(AstState::IfStmtFinalize),
        Some(AstStackSymbol::StmtList(statements)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let else_lcurly_sym = ast_stack.pop_panic();
        let else_sym = ast_stack.pop_panic();
        let right_curly_sym = ast_stack.pop_panic();
        let body_sym = ast_stack.pop_panic();
        let left_curly_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        let if_sym = ast_stack.pop_panic();
        match (
          if_sym,
          expr_sym,
          left_curly_sym,
          body_sym,
          right_curly_sym,
          else_sym,
          else_lcurly_sym,
        ) {
          (
            Some(AstStackSymbol::Token(if_token)),
            Some(AstStackSymbol::Expr(expression)),
            Some(AstStackSymbol::Token(left_curly)),
            Some(AstStackSymbol::StmtList(body)),
            Some(AstStackSymbol::Token(right_curly)),
            Some(AstStackSymbol::Token(else_token)),
            Some(AstStackSymbol::Token(else_lcurly)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::If(IfStatement::new(
              if_token,
              expression,
              left_curly,
              body,
              right_curly,
              Some(else_token),
              Some(else_lcurly),
              statements,
              Some(current_token.clone()),
            ))));
            token_index += 1;
            state_stack.pop();
          }
          _ => panic!("bad stack"),
        }
      }
      (Some(AstState::CastDecStmt), _, TokenType::Keyword) => {
        if current_token.lexeme == "cast" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::CastDecExpr);
          token_index += 1;
        } else {
          panic!("I DID NOT EXPECT TO GET HERE CAST DEC STMT");
        }
      }
      (Some(AstState::CastDecStmt), _, _) => panic!("I DID NOT EXPECT TO GET HERE CAST DEC STMT"),
      (Some(AstState::CastDecExpr), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::CastDecExpr), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let cast_token_sym = ast_stack.pop_panic();
        match cast_token_sym {
          Some(AstStackSymbol::Token(cast_token)) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::Cast(CastStatement::new(
              cast_token, expression,
            ))));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("bad stack cast dec expr"),
        }
      }
      (Some(AstState::LetStmt), _, TokenType::Keyword) => {
        if current_token.lexeme == "let" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::LetVar);
          token_index += 1;
        } else {
          panic!("I SHOULD NOT BE HERE LET STMT");
        }
      }
      (Some(AstState::LetStmt), _, _) => panic!("I SHOULD NOT BE HERE LET STMT"),
      (Some(AstState::LetVar), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Var);
      }
      (Some(AstState::LetVar), Some(AstStackSymbol::Var(var)), TokenType::Symbol) => {
        if current_token.lexeme == "=" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.goto(AstState::LetExpression);
        } else {
          ast_stack.pop();
          let let_token_sym = ast_stack.pop_panic();
          match let_token_sym {
            Some(AstStackSymbol::Token(let_token)) => {
              ast_stack.push(AstStackSymbol::Stmt(Statement::VarDec(
                VarDecStatement::new(let_token, var, None, None),
              )));
              state_stack.goto(AstState::StmtFinalize);
            }
            _ => panic!("not good :( let var"),
          }
        }
      }
      (Some(AstState::LetVar), Some(AstStackSymbol::Var(var)), _) => {
        ast_stack.pop();
        let let_token_sym = ast_stack.pop_panic();
        match let_token_sym {
          Some(AstStackSymbol::Token(let_token)) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::VarDec(
              VarDecStatement::new(let_token, var, None, None),
            )));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("not good :( let var"),
        }
      }
      (Some(AstState::LetVar), Some(AstStackSymbol::TypeVar(type_var)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Var(Var::Typed(type_var)));
      }
      (Some(AstState::LetVar), _, _) => panic!("creeper aw man :( let var"),
      (Some(AstState::LetExpression), Some(AstStackSymbol::Token(_)), TokenType::Newline) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::LetExpression), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::LetExpression), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let assignment_token_sym = ast_stack.pop_panic();
        let var_sym = ast_stack.pop_panic();
        let let_sym = ast_stack.pop_panic();
        match (let_sym, var_sym, assignment_token_sym) {
          (
            Some(AstStackSymbol::Token(let_token)),
            Some(AstStackSymbol::Var(var)),
            Some(AstStackSymbol::Token(assignment_token)),
          ) => {
            ast_stack.push(AstStackSymbol::Stmt(Statement::VarDec(
              VarDecStatement::new(let_token, var, Some(assignment_token), Some(expression)),
            )));
            state_stack.goto(AstState::StmtFinalize);
          }
          _ => panic!("uhoh we weren't meant to get here :("),
        }
      }
      (
        Some(AstState::UseStmtIdList),
        Some(AstStackSymbol::IdList(mut contents)),
        TokenType::Identifier,
      )
      | (
        Some(AstState::UseStmtIdList),
        Some(AstStackSymbol::IdList(mut contents)),
        TokenType::Type,
      ) => {
        ast_stack.pop();
        contents.push(current_token.clone());
        ast_stack.push(AstStackSymbol::IdList(contents));
        state_stack.push(AstState::UseStmtIdListFollow);
        token_index += 1;
      }
      (Some(AstState::UseStmtIdListFollow), Some(AstStackSymbol::IdList(_)), TokenType::Dot) => {
        token_index += 1;
        state_stack.pop();
      }
      (
        Some(AstState::UseStmtIdListFollow),
        Some(AstStackSymbol::IdList(_)),
        TokenType::Keyword,
      ) => {
        if current_token.lexeme == "as" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.pop();
          state_stack.goto(AstState::UseStmtAlias);
        } else {
          let id_list = ast_stack.pop_panic();
          let use_token = ast_stack.pop_panic();
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
        let id_list = ast_stack.pop_panic();
        let use_token = ast_stack.pop_panic();
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
      (Some(AstState::UseStmtAlias), Some(_), TokenType::Identifier) => {
        let as_token = ast_stack.pop_panic();
        let id_list = ast_stack.pop_panic();
        let use_token = ast_stack.pop_panic();
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
      (Some(AstState::PackDecName), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::PackDecStartEntryList);
      }
      (Some(AstState::PackDecStartEntryList), Some(_), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::PackDecList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::PackDecEntry);
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
      (Some(AstState::PackDecEntry), Some(AstStackSymbol::TypeVar(type_var)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::PackDec(PackDeclaration::new(
          type_var, None, None,
        )));
        state_stack.goto(AstState::PackDecEntryFinalize);
      }
      (Some(AstState::PackDecEntry), Some(AstStackSymbol::PackDecList(_)), TokenType::RCurly) => {
        state_stack.goto(AstState::PackDecFinalize);
      }
      (Some(AstState::PackDecEntryExpression), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let assign_token_sym = ast_stack.pop_panic();
        let pack_dec_sym = ast_stack.pop_panic();
        match (assign_token_sym, pack_dec_sym) {
          (Some(AstStackSymbol::Token(assign_token)), Some(AstStackSymbol::PackDec(mut entry))) => {
            entry.assignment = Some(assign_token);
            entry.expression = Some(expression);
            ast_stack.push(AstStackSymbol::PackDec(entry));
            state_stack.pop();
          }
          _ => panic!("weird stack after pack dec entry expression"),
        }
      }
      (Some(AstState::PackDecEntryExpression), Some(_), _) => {
        state_stack.push(AstState::Expression);
      }
      (
        Some(AstState::PackDecEntryFinalize),
        Some(AstStackSymbol::PackDec(_)),
        TokenType::Symbol,
      ) => {
        if current_token.lexeme == "=" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.push(AstState::PackDecEntryExpression);
          token_index += 1;
        } else {
          state_stack.push(AstState::Error);
        }
      }
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
      (Some(AstState::Var), _, TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::VarFollow);
      }
      (Some(AstState::Var), _, _) => panic!("Unexpected token in var"),
      (Some(AstState::VarFollow), Some(AstStackSymbol::Token(_)), TokenType::Colon) => {
        state_stack.push(AstState::TypeVarColon);
      }
      (Some(AstState::VarFollow), Some(AstStackSymbol::Type(_)), _) => {
        state_stack.goto(AstState::TypeVarFinalize);
      }
      (Some(AstState::VarFollow), Some(AstStackSymbol::Token(id)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Var(Var::Untyped(UntypedVar::new(
          id.clone(),
        ))));
        state_stack.pop();
      }
      (Some(AstState::TypeVar), _, TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.push(AstState::TypeVarColon);
      }
      (Some(AstState::TypeVar), Some(AstStackSymbol::Type(_)), _) => {
        state_stack.goto(AstState::TypeVarFinalize);
      }
      (Some(AstState::TypeVarColon), Some(_), TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::Type);
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
      (Some(AstState::Type), _, TokenType::Type) => {
        if current_token.lexeme == "auto" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.goto(AstState::AutoTypeFollow);
        } else if current_token.lexeme == "comp"
          || current_token.lexeme == "lazy"
          || current_token.lexeme == "ref"
          || current_token.lexeme == "mut"
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
      (Some(AstState::SubType), Some(AstStackSymbol::Type(_)), TokenType::Newline) => {
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
        state_stack.goto(AstState::BaseTypeFollow);
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
      (Some(AstState::BaseTypeFollowEnd), Some(AstStackSymbol::OptType(_)), TokenType::Type) => {
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
              } else if token.lexeme == "mut" {
                ast_stack.push(AstStackSymbol::Type(Type::Mutable(MutableType::new(
                  token,
                  Box::new(base),
                ))));
              } else {
                state_stack.push(AstState::Error);
              }
            }
            _ => {
              ast_stack.push(AstStackSymbol::Type(base));
              state_stack.goto(AstState::TypeFinalize);
            }
          },
          Some(_) => {
            ast_stack.push(AstStackSymbol::Type(base));
            state_stack.goto(AstState::TypeFinalize);
          }
          None => {
            panic!("Stack shouldn't be empty here at state AstState::type chain resolve");
          }
        }
      }
      (Some(AstState::TypeFinalize), Some(AstStackSymbol::Type(found_type)), TokenType::Symbol) => {
        if current_token.lexeme == "..." {
          token_index += 1;
          ast_stack.pop();
          ast_stack.push(AstStackSymbol::Type(Type::VarType(VarType::new(
            Box::new(found_type),
            current_token.clone(),
          ))));
          state_stack.pop();
        } else {
          state_stack.pop();
        }
      }
      (Some(AstState::TypeFinalize), _, _) => {
        state_stack.pop();
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
        ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(
          func_keyword,
          None,
          Vec::new(),
          None,
          Vec::new(),
          None,
        ))));
        state_stack.goto(AstState::BaseTypeFollow);
      }
      (Some(AstState::FuncTypeParamList), Some(AstStackSymbol::TypeList(_)), TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        state_stack.goto(AstState::FuncTypeReturnList);
      }
      (
        Some(AstState::FuncTypeParamList),
        Some(AstStackSymbol::TypeList(_)),
        TokenType::Newline,
      ) => {
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
      (
        Some(AstState::FuncTypeParamListFollow),
        Some(AstStackSymbol::TypeList(type_list)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        let func_keyword_sym = ast_stack.pop_panic();
        match (func_keyword_sym, left_paren_sym) {
          (Some(AstStackSymbol::Token(func_keyword)), Some(AstStackSymbol::Token(left_paren))) => {
            ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(
              func_keyword,
              Some(left_paren),
              type_list,
              None,
              Vec::new(),
              Some(current_token.clone()),
            ))));
            token_index += 1;
            state_stack.goto(AstState::BaseTypeFollow);
          }
          _ => panic!("wacked out stack"),
        }
      }
      (
        Some(AstState::FuncTypeReturnList),
        Some(AstStackSymbol::TypeList(_)),
        TokenType::RParen,
      ) => {
        state_stack.goto(AstState::FuncTypeReturnListFollow);
      }
      (
        Some(AstState::FuncTypeReturnList),
        Some(AstStackSymbol::TypeList(_)),
        TokenType::Newline,
      ) => {
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
      (Some(AstState::FuncTypeReturnListFollow), _, TokenType::Comma) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FuncTypeReturnList);
      }
      (Some(AstState::FuncTypeReturnListFollow), _, TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::FuncTypeReturnListFollow),
        Some(AstStackSymbol::TypeList(type_list)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let colon_sym = ast_stack.pop_panic();
        let param_type_list_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let func_keyword_sym = ast_stack.pop_panic();
        match (
          func_keyword_sym,
          left_paren_sym,
          param_type_list_sym,
          colon_sym,
        ) {
          (
            Some(AstStackSymbol::Token(func_keyword)),
            Some(AstStackSymbol::Token(left_paren)),
            Some(AstStackSymbol::TypeList(param_types)),
            Some(AstStackSymbol::Token(colon)),
          ) => {
            ast_stack.push(AstStackSymbol::Type(Type::Func(FuncType::new(
              func_keyword,
              Some(left_paren),
              param_types,
              Some(colon),
              type_list,
              Some(current_token.clone()),
            ))));
            token_index += 1;
            state_stack.goto(AstState::BaseTypeFollow);
          }
          _ => panic!("wacked out stack"),
        }
      }
      (Some(AstState::UnionDecName), Some(_), TokenType::Identifier) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecStartEntryList);
      }
      (Some(AstState::UnionDecStartEntryList), Some(_), TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::UnionDecList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecEntry);
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
      (
        Some(AstState::UnionDecEntry),
        Some(AstStackSymbol::UnionDecList(_)),
        TokenType::Newline,
      ) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::UnionDecEntry), Some(AstStackSymbol::UnionDecList(_)), TokenType::RCurly) => {
        state_stack.goto(AstState::UnionDecFinalize);
      }
      (Some(AstState::UnionDecEntry), _, _) => state_stack.push(AstState::Error),
      (Some(AstState::UnionDecStorageStart), Some(_), TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TypeList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::UnionDecStorage);
      }
      (
        Some(AstState::UnionDecStorageStart),
        Some(AstStackSymbol::Token(dec_entry_id)),
        TokenType::Comma,
      )
      | (
        Some(AstState::UnionDecStorageStart),
        Some(AstStackSymbol::Token(dec_entry_id)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::UnionDec(UnionDeclaration::new(
          dec_entry_id,
          None,
          Vec::new(),
          None,
        )));
        state_stack.goto(AstState::UnionDecEntryFinalize);
      }
      (Some(AstState::UnionDecStorageStart), _, _) => state_stack.push(AstState::Error),
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
          _ => panic!("stack bust union dec storage"),
        }
      }
      (
        Some(AstState::UnionDecStorageFollow),
        Some(AstStackSymbol::TypeList(type_list)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        let name_sym = ast_stack.pop_panic();
        match (name_sym, left_paren_sym) {
          (Some(AstStackSymbol::Token(name)), Some(AstStackSymbol::Token(left_paren))) => {
            ast_stack.push(AstStackSymbol::UnionDec(UnionDeclaration::new(
              name,
              Some(left_paren),
              type_list,
              Some(current_token.clone()),
            )));
            token_index += 1;
            state_stack.goto(AstState::UnionDecEntryFinalize);
          }
          _ => panic!("union dec storage follow error"),
        }
      }
      (
        Some(AstState::UnionDecStorageFollow),
        Some(AstStackSymbol::TypeList(_)),
        TokenType::Comma,
      ) => {
        token_index += 1;
        state_stack.goto(AstState::UnionDecStorage);
      }
      (
        Some(AstState::UnionDecStorageFollow),
        Some(AstStackSymbol::TypeList(_)),
        TokenType::Newline,
      ) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::UnionDecStorageFollow), _, _) => state_stack.push(AstState::Error),
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
        state_stack.push(AstState::Error);
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
      (Some(AstState::UnionDecFinalize), _, _) => state_stack.push(AstState::Error),

      //* EXPRESSION SECTION
      (Some(AstState::ExprStmt), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Stmt(Statement::Expression(
          ExpressionStatement::new(expression),
        )));
        state_stack.goto(AstState::StmtFinalize);
      }
      (Some(AstState::ExprStmt), Some(_), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::ExprStmt), _, _) => {
        state_stack.push(AstState::Error);
      }

      (Some(AstState::Expression), _, TokenType::LParen)
      | (Some(AstState::Expression), _, TokenType::LSquare)
      | (Some(AstState::Expression), _, TokenType::LCurly)
      | (Some(AstState::Expression), _, TokenType::Identifier)
      | (Some(AstState::Expression), _, TokenType::Number)
      | (Some(AstState::Expression), _, TokenType::String)
      | (Some(AstState::Expression), _, TokenType::InterpolatedString)
      | (Some(AstState::Expression), _, TokenType::Keyword)
      | (Some(AstState::Expression), _, TokenType::Symbol)
      | (Some(AstState::Expression), _, TokenType::Type) => {
        state_stack.goto(AstState::ExpressionFollow);
        state_stack.push(AstState::PrefixOrPrimary);
      }

      (Some(AstState::ExpressionFollow), _, TokenType::Symbol) => {
        if is_assignment(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::ExpressionFold);
          state_stack.push(AstState::ExpressionFollow);
          state_stack.push(AstState::ComparisonFollow);
          state_stack.push(AstState::ShiftFollow);
          state_stack.push(AstState::LogicalFollow);
          state_stack.push(AstState::BitwiseFollow);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else {
          state_stack.push(AstState::EqualityFollow);
        }
      }
      (Some(AstState::ExpressionFollow), _, TokenType::EndOfInput)
      | (Some(AstState::ExpressionFollow), _, TokenType::LCurly)
      | (Some(AstState::ExpressionFollow), _, TokenType::RCurly)
      | (Some(AstState::ExpressionFollow), _, TokenType::RParen)
      | (Some(AstState::ExpressionFollow), _, TokenType::RSquare)
      | (Some(AstState::ExpressionFollow), _, TokenType::Comma)
      | (Some(AstState::ExpressionFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::LCurly,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RCurly,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RParen,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RSquare,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Comma,
      )
      | (
        Some(AstState::ExpressionFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack expression"),
        }
      }

      (Some(AstState::EqualityFollow), _, TokenType::Symbol) => {
        if is_equality(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::EqualityFollow);
          state_stack.push(AstState::EqualityFold);
          state_stack.push(AstState::ComparisonFollow);
          state_stack.push(AstState::ShiftFollow);
          state_stack.push(AstState::LogicalFollow);
          state_stack.push(AstState::BitwiseFollow);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone()) {
          state_stack.pop();
        } else {
          state_stack.push(AstState::ComparisonFollow);
        }
      }
      (Some(AstState::EqualityFollow), _, TokenType::EndOfInput)
      | (Some(AstState::EqualityFollow), _, TokenType::LCurly)
      | (Some(AstState::EqualityFollow), _, TokenType::RCurly)
      | (Some(AstState::EqualityFollow), _, TokenType::RParen)
      | (Some(AstState::EqualityFollow), _, TokenType::RSquare)
      | (Some(AstState::EqualityFollow), _, TokenType::Comma)
      | (Some(AstState::EqualityFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::EqualityFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::EqualityFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::EqualityFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::EqualityFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::EqualityFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (
        Some(AstState::EqualityFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RSquare,
      )
      | (Some(AstState::EqualityFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (
        Some(AstState::EqualityFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack equality"),
        }
      }

      (Some(AstState::ComparisonFollow), _, TokenType::Symbol) => {
        if is_comparison(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::ComparisonFollow);
          state_stack.push(AstState::ComparisonFold);
          state_stack.push(AstState::ShiftFollow);
          state_stack.push(AstState::LogicalFollow);
          state_stack.push(AstState::BitwiseFollow);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::ShiftFollow);
        }
      }
      (Some(AstState::ComparisonFollow), _, TokenType::EndOfInput)
      | (Some(AstState::ComparisonFollow), _, TokenType::LCurly)
      | (Some(AstState::ComparisonFollow), _, TokenType::RCurly)
      | (Some(AstState::ComparisonFollow), _, TokenType::RParen)
      | (Some(AstState::ComparisonFollow), _, TokenType::RSquare)
      | (Some(AstState::ComparisonFollow), _, TokenType::Comma)
      | (Some(AstState::ComparisonFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::LCurly,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RCurly,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Symbol,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RParen,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RSquare,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Comma,
      )
      | (
        Some(AstState::ComparisonFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack comparison"),
        }
      }

      (Some(AstState::ShiftFollow), _, TokenType::Symbol) => {
        if is_shift(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::ShiftFollow);
          state_stack.push(AstState::ShiftFold);
          state_stack.push(AstState::LogicalFollow);
          state_stack.push(AstState::BitwiseFollow);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::LogicalFollow);
        }
      }
      (Some(AstState::ShiftFollow), _, TokenType::EndOfInput)
      | (Some(AstState::ShiftFollow), _, TokenType::LCurly)
      | (Some(AstState::ShiftFollow), _, TokenType::RCurly)
      | (Some(AstState::ShiftFollow), _, TokenType::RParen)
      | (Some(AstState::ShiftFollow), _, TokenType::RSquare)
      | (Some(AstState::ShiftFollow), _, TokenType::Comma)
      | (Some(AstState::ShiftFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::ShiftFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::ShiftFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack shift"),
        }
      }

      (Some(AstState::LogicalFollow), _, TokenType::Symbol) => {
        if is_logical(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::LogicalFold);
          state_stack.push(AstState::LogicalFollow);
          state_stack.push(AstState::BitwiseFollow);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::BitwiseFollow);
        }
      }
      (Some(AstState::LogicalFollow), _, TokenType::EndOfInput)
      | (Some(AstState::LogicalFollow), _, TokenType::LCurly)
      | (Some(AstState::LogicalFollow), _, TokenType::RCurly)
      | (Some(AstState::LogicalFollow), _, TokenType::RParen)
      | (Some(AstState::LogicalFollow), _, TokenType::RSquare)
      | (Some(AstState::LogicalFollow), _, TokenType::Comma)
      | (Some(AstState::LogicalFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::LogicalFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::LogicalFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) =>
      {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack logical"),
        }
      }

      (Some(AstState::BitwiseFollow), _, TokenType::Symbol) => {
        if is_bitwise(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::BitwiseFollow);
          state_stack.push(AstState::BitwiseFold);
          state_stack.push(AstState::AdditiveFollow);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::AdditiveFollow);
        }
      }
      (Some(AstState::BitwiseFollow), _, TokenType::EndOfInput)
      | (Some(AstState::BitwiseFollow), _, TokenType::LCurly)
      | (Some(AstState::BitwiseFollow), _, TokenType::RCurly)
      | (Some(AstState::BitwiseFollow), _, TokenType::RParen)
      | (Some(AstState::BitwiseFollow), _, TokenType::RSquare)
      | (Some(AstState::BitwiseFollow), _, TokenType::Comma)
      | (Some(AstState::BitwiseFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::BitwiseFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::BitwiseFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) =>
      {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack bitwise"),
        }
      }

      (Some(AstState::AdditiveFollow), _, TokenType::Symbol) => {
        if is_additive(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::AdditiveFollow);
          state_stack.push(AstState::AdditiveFold);
          state_stack.push(AstState::MultiplicativeFollow);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
          || is_bitwise(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::MultiplicativeFollow);
        }
      }
      (Some(AstState::AdditiveFollow), _, TokenType::EndOfInput)
      | (Some(AstState::AdditiveFollow), _, TokenType::LCurly)
      | (Some(AstState::AdditiveFollow), _, TokenType::RCurly)
      | (Some(AstState::AdditiveFollow), _, TokenType::RParen)
      | (Some(AstState::AdditiveFollow), _, TokenType::RSquare)
      | (Some(AstState::AdditiveFollow), _, TokenType::Comma)
      | (Some(AstState::AdditiveFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::AdditiveFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::AdditiveFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::AdditiveFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::AdditiveFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::AdditiveFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (
        Some(AstState::AdditiveFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RSquare,
      )
      | (Some(AstState::AdditiveFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (
        Some(AstState::AdditiveFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack add"),
        }
      }

      (Some(AstState::MultiplicativeFollow), _, TokenType::Symbol) => {
        if is_multiplicative(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::MultiplicativeFollow);
          state_stack.push(AstState::MultiplicativeFold);
          state_stack.push(AstState::ArrayFollow);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
          || is_bitwise(current_token.lexeme.clone())
          || is_additive(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::ArrayFollow);
        }
      }
      (Some(AstState::MultiplicativeFollow), _, TokenType::EndOfInput)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::LCurly)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::RCurly)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::RParen)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::RSquare)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::Comma)
      | (Some(AstState::MultiplicativeFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::LCurly,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RCurly,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Symbol,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RParen,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RSquare,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Comma,
      )
      | (
        Some(AstState::MultiplicativeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      ) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack multiply"),
        }
      }

      (Some(AstState::ArrayFollow), _, TokenType::Symbol) => {
        if is_array(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::ArrayFollow);
          state_stack.push(AstState::ArrayFold);
          state_stack.push(AstState::RangeFollow);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
          || is_bitwise(current_token.lexeme.clone())
          || is_additive(current_token.lexeme.clone())
          || is_multiplicative(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::RangeFollow);
        }
      }
      (Some(AstState::ArrayFollow), _, TokenType::EndOfInput)
      | (Some(AstState::ArrayFollow), _, TokenType::LCurly)
      | (Some(AstState::ArrayFollow), _, TokenType::RCurly)
      | (Some(AstState::ArrayFollow), _, TokenType::RParen)
      | (Some(AstState::ArrayFollow), _, TokenType::RSquare)
      | (Some(AstState::ArrayFollow), _, TokenType::Comma)
      | (Some(AstState::ArrayFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::ArrayFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::ArrayFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack array"),
        }
      }

      (Some(AstState::RangeFollow), _, TokenType::Symbol) => {
        if is_range(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::RangeFollow);
          state_stack.push(AstState::RangeFold);
          state_stack.push(AstState::DefaultFollow);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
          || is_bitwise(current_token.lexeme.clone())
          || is_additive(current_token.lexeme.clone())
          || is_multiplicative(current_token.lexeme.clone())
          || is_array(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          state_stack.push(AstState::DefaultFollow);
        }
      }
      (Some(AstState::RangeFollow), _, TokenType::EndOfInput)
      | (Some(AstState::RangeFollow), _, TokenType::LCurly)
      | (Some(AstState::RangeFollow), _, TokenType::RCurly)
      | (Some(AstState::RangeFollow), _, TokenType::RParen)
      | (Some(AstState::RangeFollow), _, TokenType::RSquare)
      | (Some(AstState::RangeFollow), _, TokenType::Comma)
      | (Some(AstState::RangeFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::RangeFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::RangeFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack range"),
        }
      }

      (Some(AstState::DefaultFollow), _, TokenType::Symbol) => {
        if is_default(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::DefaultFollow);
          state_stack.push(AstState::DefaultFold);
          state_stack.push(AstState::PrefixOrPrimary);
          token_index += 1;
        } else if is_assignment(current_token.lexeme.clone())
          || is_equality(current_token.lexeme.clone())
          || is_comparison(current_token.lexeme.clone())
          || is_shift(current_token.lexeme.clone())
          || is_logical(current_token.lexeme.clone())
          || is_bitwise(current_token.lexeme.clone())
          || is_additive(current_token.lexeme.clone())
          || is_multiplicative(current_token.lexeme.clone())
          || is_array(current_token.lexeme.clone())
          || is_range(current_token.lexeme.clone())
        {
          state_stack.pop();
        } else {
          panic!("Did not expect to get here :("); // does this actually get hit?
        }
      }
      (Some(AstState::DefaultFollow), _, TokenType::EndOfInput)
      | (Some(AstState::DefaultFollow), _, TokenType::LCurly)
      | (Some(AstState::DefaultFollow), _, TokenType::RCurly)
      | (Some(AstState::DefaultFollow), _, TokenType::RParen)
      | (Some(AstState::DefaultFollow), _, TokenType::RSquare)
      | (Some(AstState::DefaultFollow), _, TokenType::Comma)
      | (Some(AstState::DefaultFollow), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::Symbol)
      | (
        Some(AstState::DefaultFold),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::EndOfInput,
      )
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::RCurly)
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::RParen)
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::LCurly)
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare)
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::Comma)
      | (Some(AstState::DefaultFold), Some(AstStackSymbol::Expr(expression)), TokenType::Newline) =>
      {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        let lhs_sym = ast_stack.pop_panic();
        match (lhs_sym, operator_sym) {
          (Some(AstStackSymbol::Expr(lhs)), Some(AstStackSymbol::Token(op))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Binary(
              BinaryExpression::new(Box::new(lhs), op, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("unexpected stack default"),
        }
      }

      (Some(AstState::PrefixOrPrimary), _, TokenType::LParen)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::LSquare)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::LCurly)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::Identifier)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::Number)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::String)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::InterpolatedString)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::Keyword)
      | (Some(AstState::PrefixOrPrimary), _, TokenType::Type) => {
        state_stack.goto(AstState::Primary);
      }

      (Some(AstState::PrefixOrPrimary), _, TokenType::Symbol) => {
        state_stack.goto(AstState::Prefix);
      }

      (Some(AstState::Primary), _, TokenType::LCurly) => {
        state_stack.goto(AstState::TupleStart);
      }

      (Some(AstState::Primary), _, TokenType::EndOfInput)
      | (Some(AstState::Primary), _, TokenType::RParen)
      | (Some(AstState::Primary), _, TokenType::RSquare)
      | (Some(AstState::Primary), _, TokenType::Comma)
      | (Some(AstState::Primary), _, TokenType::Newline) => {
        state_stack.pop();
      }
      (Some(AstState::Primary), _, TokenType::Type) => {
        state_stack.goto(AstState::FunctionPrimary);
      }
      (Some(AstState::Primary), _, TokenType::LParen) => {
        state_stack.goto(AstState::SubExpression);
      }
      (Some(AstState::Primary), _, TokenType::LSquare) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::ExprList(Vec::new()));
        state_stack.goto(AstState::ArrayLiteralContents);
      }

      (Some(AstState::TupleStart), _, TokenType::LCurly) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::TupleEntryList(Vec::new()));
        state_stack.goto(AstState::TupleContents);
        state_stack.push(AstState::TupleEntry);
      }
      (Some(AstState::TupleEntry), _, TokenType::LParen)
      | (Some(AstState::TupleEntry), _, TokenType::LSquare)
      | (Some(AstState::TupleEntry), _, TokenType::LCurly)
      | (Some(AstState::TupleEntry), _, TokenType::Number)
      | (Some(AstState::TupleEntry), _, TokenType::String)
      | (Some(AstState::TupleEntry), _, TokenType::InterpolatedString)
      | (Some(AstState::TupleEntry), _, TokenType::Keyword)
      | (Some(AstState::TupleEntry), _, TokenType::Symbol)
      | (Some(AstState::TupleEntry), _, TokenType::Type) => {
        state_stack.goto(AstState::TupleEntryUnnamedEnd);
        state_stack.push(AstState::ExpressionFollow);
        state_stack.push(AstState::PrefixOrPrimary);
      }
      (Some(AstState::TupleEntry), _, TokenType::Identifier) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        state_stack.goto(AstState::TupleEntryFollow);
      }
      (Some(AstState::TupleEntry), _, TokenType::RCurly) => {
        state_stack.pop();
      }
      (Some(AstState::TupleEntryFollow), _, TokenType::Colon) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        state_stack.goto(AstState::TupleEntryNamedEnd);
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::TupleEntryFollow), Some(AstStackSymbol::Token(id)), _) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::Expr(Expression::Var(UntypedVar::new(id))));
        state_stack.goto(AstState::TupleEntryUnnamedEnd);
        state_stack.push(AstState::ExpressionFollow);
        state_stack.push(AstState::PrimaryFollow);
      }
      (
        Some(AstState::TupleEntryUnnamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      )
      | (
        Some(AstState::TupleEntryUnnamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RCurly,
      )
      | (
        Some(AstState::TupleEntryUnnamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Comma,
      ) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::TupleEntry(TupleEntry::new(
          None, None, expression,
        )));
        state_stack.pop();
      }

      (
        Some(AstState::TupleEntryNamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Newline,
      )
      | (
        Some(AstState::TupleEntryNamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::RCurly,
      )
      | (
        Some(AstState::TupleEntryNamedEnd),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Comma,
      ) => {
        ast_stack.pop();
        let colon_sym = ast_stack.pop_panic();
        let name_sym = ast_stack.pop_panic();
        match (name_sym, colon_sym) {
          (Some(AstStackSymbol::Token(name)), Some(AstStackSymbol::Token(colon))) => {
            ast_stack.push(AstStackSymbol::TupleEntry(TupleEntry::new(
              Some(name),
              Some(colon),
              expression,
            )));
            state_stack.pop();
          }
          _ => panic!("bad stack tuple entry"),
        }
      }

      (Some(AstState::TupleContents), _, TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::TupleContents),
        Some(AstStackSymbol::TupleEntryList(_)),
        TokenType::RCurly,
      ) => {
        state_stack.goto(AstState::TupleEnd);
      }
      (
        Some(AstState::TupleContents),
        Some(AstStackSymbol::TupleEntry(entry)),
        TokenType::Comma,
      ) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.pop();
        let mut entry_list = ast_stack.pop_panic();
        match entry_list {
          Some(AstStackSymbol::TupleEntryList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::TupleEntryList(contents));
            state_stack.push(AstState::TupleEntry);
          }
          _ => panic!("bad stack"),
        }
      }
      (
        Some(AstState::TupleContents),
        Some(AstStackSymbol::TupleEntry(entry)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let entry_list = ast_stack.pop_panic();
        match entry_list {
          Some(AstStackSymbol::TupleEntryList(mut contents)) => {
            contents.push(entry);
            ast_stack.push(AstStackSymbol::TupleEntryList(contents));
            state_stack.goto(AstState::TupleEnd);
          }
          _ => panic!("bad stack"),
        }
      }
      (
        Some(AstState::TupleEnd),
        Some(AstStackSymbol::TupleEntryList(entry_list)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let l_curly_sym = ast_stack.pop_panic();
        match l_curly_sym {
          Some(AstStackSymbol::Token(x)) if x.token_type == TokenType::LCurly => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Tuple(
              Tuple::new(x, entry_list, current_token.clone()),
            ))));
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("bad stack"),
        }
      }

      // add in tuple contents, tuple entry, tuple entry value, and tuple end here
      (Some(AstState::FunctionPrimary), _, TokenType::Type) => {
        if current_token.lexeme == "func" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          token_index = consume_optional_newline(tokens, token_index);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (Some(AstState::FunctionPrimary), Some(AstStackSymbol::Token(_)), TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.push(AstStackSymbol::ParamList(Vec::new()));
        state_stack.push(AstState::FunctionParameters);
      }
      (Some(AstState::FunctionPrimary), Some(AstStackSymbol::ParamList(_)), TokenType::RParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FunctionArrow);
      }
      (
        Some(AstState::FunctionParameters),
        Some(AstStackSymbol::ParamList(_)),
        TokenType::RParen,
      ) => {
        state_stack.pop();
      }
      (Some(AstState::FunctionParameters), Some(AstStackSymbol::ParamList(_)), _) => {
        state_stack.push(AstState::TypeVar);
      }
      (
        Some(AstState::FunctionParameters),
        Some(AstStackSymbol::TypeVar(_)),
        TokenType::Newline,
      ) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::FunctionParameters),
        Some(AstStackSymbol::TypeVar(param)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let param_list_sym = ast_stack.pop_panic();
        match param_list_sym {
          Some(AstStackSymbol::ParamList(mut param_list)) => {
            param_list.push(Parameter::new(param));
            ast_stack.push(AstStackSymbol::ParamList(param_list));
          }
          _ => panic!("Stack bust function parameters"),
        }
      }
      (
        Some(AstState::FunctionParameters),
        Some(AstStackSymbol::TypeVar(param)),
        TokenType::Comma,
      ) => {
        ast_stack.pop();
        let param_list_sym = ast_stack.pop_panic();
        match param_list_sym {
          Some(AstStackSymbol::ParamList(mut param_list)) => {
            param_list.push(Parameter::new(param));
            ast_stack.push(AstStackSymbol::ParamList(param_list));
            token_index += 1;
            token_index = consume_optional_newline(tokens, token_index);
          }
          _ => panic!("Stack bust function parameters"),
        }
      }
      (Some(AstState::FunctionArrow), _, TokenType::Arrow) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (Some(AstState::FunctionArrow), _, TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        ast_stack.push(AstStackSymbol::ReturnList(Vec::new()));
        state_stack.push(AstState::FunctionReturns);
      }
      (Some(AstState::FunctionArrow), _, TokenType::RParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        state_stack.goto(AstState::FunctionSignatureFollow);
      }
      (Some(AstState::FunctionReturns), Some(AstStackSymbol::ReturnList(_)), TokenType::RParen) => {
        state_stack.pop();
      }
      (Some(AstState::FunctionReturns), Some(AstStackSymbol::ReturnList(_)), _) => {
        state_stack.push(AstState::FunctionReturnEntry);
      }
      (
        Some(AstState::FunctionReturns),
        Some(AstStackSymbol::ReturnEntry(_)),
        TokenType::Newline,
      ) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::FunctionReturns),
        Some(AstStackSymbol::ReturnEntry(entry)),
        TokenType::Comma,
      ) => {
        ast_stack.pop();
        let return_list_sym = ast_stack.pop_panic();
        match return_list_sym {
          Some(AstStackSymbol::ReturnList(mut return_list)) => {
            return_list.push(entry);
            ast_stack.push(AstStackSymbol::ReturnList(return_list));
            token_index += 1;
            token_index = consume_optional_newline(tokens, token_index);
          }
          _ => panic!("stack bust function returns"),
        }
      }
      (
        Some(AstState::FunctionReturns),
        Some(AstStackSymbol::ReturnEntry(entry)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let return_list_sym = ast_stack.pop_panic();
        match return_list_sym {
          Some(AstStackSymbol::ReturnList(mut return_list)) => {
            return_list.push(entry);
            ast_stack.push(AstStackSymbol::ReturnList(return_list));
          }
          _ => panic!("stack bust function returns"),
        }
      }
      (Some(AstState::FunctionReturnEntry), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let equal_sym = ast_stack.pop_panic();
        let type_var_sym = ast_stack.pop_panic();
        match (type_var_sym, equal_sym) {
          (Some(AstStackSymbol::TypeVar(type_var)), Some(AstStackSymbol::Token(equal))) => {
            ast_stack.push(AstStackSymbol::ReturnEntry(ReturnEntry::new(
              type_var,
              Some(equal),
              Some(Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("bad stack :("),
        }
      }
      (
        Some(AstState::FunctionReturnEntry),
        Some(AstStackSymbol::TypeVar(var)),
        TokenType::Symbol,
      ) => {
        if current_token.lexeme == "=" {
          token_index += 1;
          token_index = consume_optional_newline(tokens, token_index);
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.push(AstState::Expression);
        } else {
          ast_stack.pop();
          ast_stack.push(AstStackSymbol::ReturnEntry(ReturnEntry::new(
            var, None, None,
          )));
          state_stack.pop();
        }
      }
      (Some(AstState::FunctionReturnEntry), _, TokenType::Newline) => {
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::FunctionReturnEntry),
        Some(AstStackSymbol::TypeVar(var)),
        TokenType::RParen,
      )
      | (
        Some(AstState::FunctionReturnEntry),
        Some(AstStackSymbol::TypeVar(var)),
        TokenType::Comma,
      ) => {
        ast_stack.pop();
        ast_stack.push(AstStackSymbol::ReturnEntry(ReturnEntry::new(
          var, None, None,
        )));
        state_stack.pop();
      }
      (Some(AstState::FunctionReturnEntry), _, TokenType::Identifier) => {
        state_stack.push(AstState::TypeVar);
      }
      (Some(AstState::FunctionSignatureFollow), _, TokenType::Colon) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::FunctionBody);
      }
      (Some(AstState::FunctionSignatureFollow), _, _) => {
        let return_rparen_sym = ast_stack.pop_panic();
        let return_list_sym = ast_stack.pop_panic();
        let return_lparen_sym = ast_stack.pop_panic();
        let arrow_sym = ast_stack.pop_panic();
        let param_rparen_sym = ast_stack.pop_panic();
        let param_list_sym = ast_stack.pop_panic();
        let param_lparen_sym = ast_stack.pop_panic();
        let func_sym = ast_stack.pop_panic();
        match (
          func_sym,
          param_lparen_sym,
          param_list_sym,
          param_rparen_sym,
          arrow_sym,
          return_lparen_sym,
          return_list_sym,
          return_rparen_sym,
        ) {
          (
            Some(AstStackSymbol::Token(func)),
            Some(AstStackSymbol::Token(param_lparen)),
            Some(AstStackSymbol::ParamList(parameter_list)),
            Some(AstStackSymbol::Token(param_rparen)),
            Some(AstStackSymbol::Token(arrow)),
            Some(AstStackSymbol::Token(return_lparen)),
            Some(AstStackSymbol::ReturnList(return_list)),
            Some(AstStackSymbol::Token(return_rparen)),
          ) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Literal(
              Literal::Function(Function::new(
                func,
                param_lparen,
                ParameterList::new(parameter_list),
                param_rparen,
                arrow,
                return_lparen,
                ReturnList::new(return_list),
                return_rparen,
                None,
                None,
                Vec::new(),
                None,
              )),
            )));
            state_stack.pop();
          }
          _ => panic!("bad stack"),
        }
      }
      (Some(AstState::FunctionBody), _, TokenType::LCurly) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::StmtList(Vec::new()));
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.push(AstState::StmtList);
      }
      (
        Some(AstState::FunctionBody),
        Some(AstStackSymbol::StmtList(statements)),
        TokenType::RCurly,
      ) => {
        ast_stack.pop();
        let left_curly_sym = ast_stack.pop_panic();
        let colon_sym = ast_stack.pop_panic();
        let return_rparen_sym = ast_stack.pop_panic();
        let return_list_sym = ast_stack.pop_panic();
        let return_lparen_sym = ast_stack.pop_panic();
        let arrow_sym = ast_stack.pop_panic();
        let param_rparen_sym = ast_stack.pop_panic();
        let param_list_sym = ast_stack.pop_panic();
        let param_lparen_sym = ast_stack.pop_panic();
        let func_sym = ast_stack.pop_panic();
        match (
          func_sym,
          param_lparen_sym,
          param_list_sym,
          param_rparen_sym,
          arrow_sym,
          return_lparen_sym,
          return_list_sym,
          return_rparen_sym,
          colon_sym,
          left_curly_sym,
        ) {
          (
            Some(AstStackSymbol::Token(func)),
            Some(AstStackSymbol::Token(param_lparen)),
            Some(AstStackSymbol::ParamList(parameter_list)),
            Some(AstStackSymbol::Token(param_rparen)),
            Some(AstStackSymbol::Token(arrow)),
            Some(AstStackSymbol::Token(return_lparen)),
            Some(AstStackSymbol::ReturnList(return_list)),
            Some(AstStackSymbol::Token(return_rparen)),
            Some(AstStackSymbol::Token(colon)),
            Some(AstStackSymbol::Token(left_curly)),
          ) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Literal(
              Literal::Function(Function::new(
                func,
                param_lparen,
                ParameterList::new(parameter_list),
                param_rparen,
                arrow,
                return_lparen,
                ReturnList::new(return_list),
                return_rparen,
                Some(colon),
                Some(left_curly),
                statements,
                Some(current_token.clone()),
              )),
            )));
            state_stack.pop();
            token_index += 1;
          }
          _ => panic!("bad stack"),
        }
      }
      (Some(AstState::SubExpression), _, TokenType::LParen) => {
        token_index += 1;
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::SubExpression), _, TokenType::RParen) => {
        token_index += 1;
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::SubExpression), _, _) => state_stack.push(AstState::Error),
      (
        Some(AstState::ArrayLiteralContents),
        Some(AstStackSymbol::ExprList(_)),
        TokenType::Newline,
      ) => {
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
          _ => panic!("Expression list not on stack :("),
        }
      }
      (Some(AstState::ArrayLiteralContents), _, _) => {
        state_stack.push(AstState::Error);
      }
      (
        Some(AstState::ArrayLiteralContentsFollow),
        Some(AstStackSymbol::ExprList(_)),
        TokenType::Comma,
      ) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
        state_stack.goto(AstState::ArrayLiteralContents);
      }
      (
        Some(AstState::ArrayLiteralContentsFollow),
        Some(AstStackSymbol::ExprList(_)),
        TokenType::Newline,
      ) => {
        token_index += 1;
        token_index = consume_optional_newline(tokens, token_index);
      }
      (
        Some(AstState::ArrayLiteralContentsFollow),
        Some(AstStackSymbol::ExprList(contents)),
        TokenType::RSquare,
      ) => {
        ast_stack.pop();
        let l_square_sym = ast_stack.pop_panic();
        match l_square_sym {
          Some(AstStackSymbol::Token(left_square)) => {
            token_index += 1;
            ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Array(
              ArrayLiteral::new(left_square, contents, current_token.clone()),
            ))));
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("panic array literal contents follow stack bad"),
        }
      }
      (Some(AstState::ArrayLiteralContentsFollow), _, _) => {
        state_stack.push(AstState::Error);
      }
      (Some(AstState::Primary), _, TokenType::Identifier) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Var(UntypedVar::new(
          current_token.clone(),
        ))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::Number) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Number(
          current_token.clone(),
        ))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::InterpolatedString)
      | (Some(AstState::Primary), _, TokenType::String) => {
        token_index += 1;
        ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::String(
          current_token.clone(),
        ))));
        state_stack.goto(AstState::PrimaryFollow);
      }
      (Some(AstState::Primary), _, TokenType::Keyword) => {
        if current_token.lexeme == "true" || current_token.lexeme == "false" {
          token_index += 1;
          ast_stack.push(AstStackSymbol::Expr(Expression::Literal(Literal::Boolean(
            current_token.clone(),
          ))));
          state_stack.goto(AstState::PrimaryFollow);
        } else {
          state_stack.push(AstState::Error);
        }
      }
      (
        Some(AstState::PrimaryFollow),
        Some(AstStackSymbol::Expr(expression)),
        TokenType::Symbol,
      ) => {
        if current_token.lexeme == "?" {
          ast_stack.pop();
          ast_stack.push(AstStackSymbol::Expr(Expression::Postfix(
            PostfixExpression::new(Box::new(expression), current_token.clone()),
          )));
          token_index += 1;
          state_stack.pop();
        } else {
          state_stack.pop();
        }
      }
      (Some(AstState::PrimaryFollow), _, TokenType::Keyword) => {
        if current_token.lexeme == "as" {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          state_stack.goto(AstState::CastExpr);
          token_index += 1;
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
      (Some(AstState::PrimaryFollow), _, TokenType::LParen) => {
        ast_stack.push(AstStackSymbol::Token(current_token.clone()));
        ast_stack.push(AstStackSymbol::ExprList(Vec::new()));
        state_stack.goto(AstState::FunctionCall);
        token_index += 1;
      }
      (Some(AstState::PrimaryFollow), _, _) => {
        state_stack.pop();
      }
      (
        Some(AstState::FunctionCall),
        Some(AstStackSymbol::ExprList(arguments)),
        TokenType::RParen,
      ) => {
        ast_stack.pop();
        let left_paren_sym = ast_stack.pop_panic();
        let target_sym = ast_stack.pop_panic();
        match (target_sym, left_paren_sym) {
          (Some(AstStackSymbol::Expr(target)), Some(AstStackSymbol::Token(left_paren))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::FunctionCall(
              FunctionCall::new(
                Box::new(target),
                left_paren,
                arguments,
                current_token.clone(),
              ),
            )));
            token_index += 1;
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("bad stack function call"),
        }
      }
      (Some(AstState::FunctionCall), Some(AstStackSymbol::Expr(expression)), TokenType::RParen) => {
        ast_stack.pop();
        let expr_list = ast_stack.pop_panic();
        match expr_list {
          Some(AstStackSymbol::ExprList(mut contents)) => {
            contents.push(Box::new(expression));
            ast_stack.push(AstStackSymbol::ExprList(contents));
          }
          _ => panic!("bad stack function call contents"),
        }
      }
      (Some(AstState::FunctionCall), Some(AstStackSymbol::Expr(expression)), TokenType::Comma) => {
        ast_stack.pop();
        let expr_list = ast_stack.pop_panic();
        match expr_list {
          Some(AstStackSymbol::ExprList(mut contents)) => {
            contents.push(Box::new(expression));
            ast_stack.push(AstStackSymbol::ExprList(contents));
            token_index += 1;
          }
          _ => panic!("bad stack function call contents"),
        }
      }
      (Some(AstState::FunctionCall), Some(AstStackSymbol::ExprList(_)), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::FunctionCall), _, _) => {
        state_stack.push(AstState::Error);
      }
      (Some(AstState::CastExpr), Some(AstStackSymbol::Type(cast_type)), _) => {
        ast_stack.pop();
        let as_token_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        match (expr_sym, as_token_sym) {
          (Some(AstStackSymbol::Expr(expression)), Some(AstStackSymbol::Token(as_token))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Cast(CastExpression::new(
              Box::new(expression),
              as_token,
              cast_type,
            ))));
            state_stack.pop();
          }
          _ => panic!("bad stack cast expr"),
        }
      }
      (Some(AstState::CastExpr), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Type);
      }
      (Some(AstState::CastExpr), _, _) => state_stack.push(AstState::Error),
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Expr(expression)), TokenType::RSquare) => {
        ast_stack.pop();
        let lsq_sym = ast_stack.pop_panic();
        let expr_sym = ast_stack.pop_panic();
        match (expr_sym, lsq_sym) {
          (Some(AstStackSymbol::Expr(lhs_expr)), Some(AstStackSymbol::Token(lsq))) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::ArrayAccess(
              ArrayAccess::new(
                Box::new(lhs_expr),
                lsq,
                Box::new(expression),
                current_token.clone(),
              ),
            )));
            token_index += 1;
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("bad array access stack"),
        }
      }
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Expr(_)), _) => state_stack.push(AstState::Error),
      (Some(AstState::ArrayAccess), Some(AstStackSymbol::Token(_)), _) => {
        state_stack.push(AstState::Expression);
      }
      (Some(AstState::MemberAccess), Some(AstStackSymbol::Token(dot)), TokenType::Identifier) => {
        ast_stack.pop();
        let expr_sym = ast_stack.pop_panic();
        match expr_sym {
          Some(AstStackSymbol::Expr(expression)) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Member(MemberAccess::new(
              Box::new(expression),
              dot,
              current_token.clone(),
            ))));
            token_index += 1;
            state_stack.goto(AstState::PrimaryFollow);
          }
          _ => panic!("could not find an expression to match the member access"),
        }
      }
      (Some(AstState::Prefix), Some(AstStackSymbol::Expr(expression)), _) => {
        ast_stack.pop();
        let operator_sym = ast_stack.pop_panic();
        match operator_sym {
          Some(AstStackSymbol::Token(operator)) => {
            ast_stack.push(AstStackSymbol::Expr(Expression::Prefix(
              PrefixExpression::new(operator, Box::new(expression)),
            )));
            state_stack.pop();
          }
          _ => panic!("Did not find prefix operator"),
        }
      }
      (Some(AstState::Prefix), _, TokenType::Symbol) => {
        if is_prefix(current_token.lexeme.clone()) {
          ast_stack.push(AstStackSymbol::Token(current_token.clone()));
          token_index += 1;
          state_stack.push(AstState::PrefixContents);
        } else {
          state_stack.push(AstState::Error);
        }
      }

      (Some(AstState::PrefixContents), _, _) => {
        state_stack.goto(AstState::PrefixOrPrimary);
      }

      //* END EXPRESSION SECTION
      (Some(AstState::Error), _, _) => {
        let (sev, msg, tkns, idx) = build_error(&mut state_stack, &mut ast_stack, &tokens, token_index);
        let error = ErrorStatement::new(msg, sev, tkns);
        ast_stack.push(AstStackSymbol::Stmt(Statement::Error(error.clone())));
        state_stack.push(AstState::StmtFinalize);
        token_index = idx;
        errors.push(OceanError::ParseError(error.clone()))
      }
      (_, _, _) => {
        state_stack.push(AstState::Error);
      }
    };
  }

  if ast_stack.size() == 1 {
    match ast_stack.pop() {
      Some(AstStackSymbol::StmtList(stmts)) => (Some(Program::new(stmts)), errors),
      _ => (None, errors),
    }
  } else {
    (None, errors)
  }
}

fn build_error(state_stack: &mut StateStack, ast_stack: &mut Stack<AstStackSymbol>, tokens: &Vec<Token>, token_index: usize) -> (Severity, String, Vec<Token>, usize) {
  eprintln!("ERROR ----------------------------------------------------");
  let mut ast_current_state = Vec::new();
  while true {
    match state_stack.current_state() {
      Some(AstState::StmtList) | None => break,
      Some(x) => ast_current_state.insert(0, x)
    }
    state_stack.pop();
  }
  let mut ast_current_symbols = Vec::new();
  while true {
    match ast_stack.peek() {
      Some(AstStackSymbol::StmtList(_)) | None => break,
      Some(x) => ast_current_symbols.insert(0, x)
    }
    ast_stack.pop();
  }
  if state_stack.is_empty() {
    state_stack.push(AstState::StmtList)
  }
  let message = format!("Unexpected token! {} :(", tokens[token_index].clone());
  let tkns = vec![tokens[token_index].clone()];
  let mut next_index = token_index;
  while next_index < tokens.len() {
    match tokens[next_index].token_type {
      TokenType::Newline | TokenType::EndOfInput => break,
      _ => next_index += 1
    }
  }
  (Severity::Error, message, tkns, next_index)
} 

fn is_assignment(lexeme: String) -> bool {
  lexeme == "="
    || lexeme == ">>="
    || lexeme == "<<="
    || lexeme == "^^="
    || lexeme == "||="
    || lexeme == "&&="
    || lexeme == "^="
    || lexeme == "&="
    || lexeme == "|="
    || lexeme == "+="
    || lexeme == "-="
    || lexeme == "*="
    || lexeme == "/="
    || lexeme == "%="
    || lexeme == "//="
    || lexeme == "++="
    || lexeme == "--="
    || lexeme == ">.="
    || lexeme == "??="
    || lexeme == "~="
}

fn is_equality(lexeme: String) -> bool {
  lexeme == "==" || lexeme == "!="
}

fn is_comparison(lexeme: String) -> bool {
  lexeme == "<" || lexeme == ">" || lexeme == "<=" || lexeme == ">="
}

fn is_shift(lexeme: String) -> bool {
  lexeme == "<<" || lexeme == ">>"
}

fn is_logical(lexeme: String) -> bool {
  lexeme == "||" || lexeme == "&&" || lexeme == "^^"
}

fn is_bitwise(lexeme: String) -> bool {
  lexeme == "|" || lexeme == "&" || lexeme == "^"
}

fn is_additive(lexeme: String) -> bool {
  lexeme == "+" || lexeme == "-"
}

fn is_multiplicative(lexeme: String) -> bool {
  lexeme == "*" || lexeme == "/" || lexeme == "%" || lexeme == "//"
}

fn is_array(lexeme: String) -> bool {
  lexeme == "++" || lexeme == "--" || lexeme == ">."
}

fn is_range(lexeme: String) -> bool {
  lexeme == ".." || lexeme == "..<" || lexeme == "..="
}

fn is_default(lexeme: String) -> bool {
  lexeme == "??"
}

fn is_prefix(lexeme: String) -> bool {
  lexeme == "~" || lexeme == "!" || lexeme == "-" || lexeme == "..."
}
