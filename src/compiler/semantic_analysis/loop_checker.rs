use crate::compiler::errors::OceanError;
use crate::compiler::errors::Severity;
use crate::compiler::parser::ast::*;
use crate::compiler::parser::span::Spanned;
use crate::compiler::semantic_analysis::Statement::*;

pub fn loop_checker(program: &Program) -> Vec<OceanError> {
  let mut errors = Vec::new();
  for stmt in &program.statements {
    errors.append(&mut loop_checker_stmt(stmt, false));
  }
  errors
}

fn loop_checker_stmt(stmt: &Statement, in_loop: bool) -> Vec<OceanError> {
  match stmt {
    Match(_) => todo!(),
    Break(break_stmt) if !in_loop => vec![OceanError::SemanticError(
      Severity::Error,
      break_stmt.get_span(),
      "Found break statement outside of loop".to_string(),
    )],
    Continue(cont_stmt) if !in_loop => vec![OceanError::SemanticError(
      Severity::Error,
      cont_stmt.get_span(),
      "Found break statement outside of loop".to_string(),
    )],
    PackDec(pack_dec) => loop_checker_pack(pack_dec, false),
    VarDec(var_dec) => loop_checker_var_dec(var_dec, false),
    Cast(cast_stmt) => loop_checker_cast(cast_stmt, false),
    ForLoop(loop_stmt) => loop_checker_for(loop_stmt, true),
    WhileLoop(loop_stmt) => loop_checker_while(loop_stmt, true),
    InfiniteLoop(loop_stmt) => loop_checker_inf_loop(loop_stmt, true),
    If(if_stmt) => loop_checker_if(if_stmt, in_loop),
    Expression(expr) => loop_checker_expr(expr, in_loop),
    Error(_) | Macro(_) | Return(_) | Use(_) | UnionDec(_) | _ => Vec::new(),
  }
}

fn loop_checker_pack(pack_dec: &PackDecStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_var_dec(var_dec: &VarDecStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_cast(cast_stmt: &CastStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_for(for_loop: &ForLoopStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_while(while_loop: &WhileStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_inf_loop(inf_loop: &InfiniteLoopStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_if(if_stmt: &IfStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}

fn loop_checker_expr(expr: &ExpressionStatement, in_loop: bool) -> Vec<OceanError> {
  Vec::new()
}
