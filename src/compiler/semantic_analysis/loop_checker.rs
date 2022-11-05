use crate::compiler::parser::ast::Expression;
use crate::compiler::parser::ast::*;
use crate::compiler::parser::span::Spanned;
use crate::compiler::semantic_analysis::Statement::*;
use crate::util::errors::OceanError;
use crate::util::errors::Severity;

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
      "Found continue statement outside of loop".to_string(),
    )],
    PackDec(pack_dec) => loop_checker_pack(pack_dec, false),
    VarDec(var_dec) => loop_checker_var_dec(var_dec, false),
    Cast(cast_stmt) => loop_checker_cast(cast_stmt, false),
    ForLoop(loop_stmt) => loop_checker_for(loop_stmt, true),
    WhileLoop(loop_stmt) => loop_checker_while(loop_stmt, true),
    InfiniteLoop(loop_stmt) => loop_checker_inf_loop(loop_stmt, true),
    If(if_stmt) => loop_checker_if(if_stmt, in_loop),
    Expression(expr) => loop_checker_expr_stmt(expr, in_loop),
    Error(_) | Macro(_) | Return(_) | Use(_) | UnionDec(_) | _ => Vec::new(),
  }
}

fn loop_checker_pack(pack_dec: &PackDecStatement, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  for pack_dec_entry in &pack_dec.pack_declarations {
    errors.append(&mut loop_checker_pack_dec_entry(pack_dec_entry, false));
  }
  errors
}

fn loop_checker_pack_dec_entry(pack_dec_entry: &PackDeclaration, in_loop: bool) -> Vec<OceanError> {
  match &pack_dec_entry.expression {
    Some(x) => loop_checker_expression(&x, in_loop),
    None => Vec::new(),
  }
}

fn loop_checker_var_dec(var_dec: &VarDecStatement, in_loop: bool) -> Vec<OceanError> {
  match &var_dec.expression {
    Some(x) => loop_checker_expression(&x, in_loop),
    None => Vec::new(),
  }
}

fn loop_checker_cast(cast_stmt: &CastStatement, in_loop: bool) -> Vec<OceanError> {
  loop_checker_expression(&cast_stmt.function, in_loop)
}

fn loop_checker_for(for_loop: &ForLoopStatement, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  errors.append(&mut loop_checker_expression(&for_loop.iterable, in_loop));
  for stmt in &for_loop.loop_body {
    errors.append(&mut loop_checker_stmt(stmt, true));
  }
  errors
}

fn loop_checker_while(while_loop: &WhileStatement, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  errors.append(&mut loop_checker_expression(&while_loop.condition, in_loop));
  for stmt in &while_loop.loop_body {
    errors.append(&mut loop_checker_stmt(stmt, true));
  }
  errors
}

fn loop_checker_inf_loop(inf_loop: &InfiniteLoopStatement, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  for stmt in &inf_loop.loop_body {
    errors.append(&mut loop_checker_stmt(stmt, true));
  }
  errors
}

fn loop_checker_if(if_stmt: &IfStatement, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  for stmt in &if_stmt.true_body {
    errors.append(&mut loop_checker_stmt(stmt, in_loop));
  }
  for stmt in &if_stmt.else_body {
    errors.append(&mut loop_checker_stmt(stmt, in_loop));
  }
  errors
}

fn loop_checker_expr_stmt(expr: &ExpressionStatement, in_loop: bool) -> Vec<OceanError> {
  loop_checker_expression(&expr.expression, in_loop)
}

fn loop_checker_expression(expr: &Expression, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  match expr {
    Expression::Binary(x) => {
      errors.append(&mut loop_checker_expression(x.lhs.as_ref(), in_loop));
      errors.append(&mut loop_checker_expression(x.rhs.as_ref(), in_loop));
    }
    Expression::Prefix(x) => {
      errors.append(&mut loop_checker_expression(x.rhs.as_ref(), in_loop));
    }
    Expression::Postfix(x) => {
      errors.append(&mut loop_checker_expression(x.lhs.as_ref(), in_loop));
    }
    Expression::Member(x) => {
      errors.append(&mut loop_checker_expression(x.lhs.as_ref(), in_loop));
    }
    Expression::ArrayAccess(x) => {
      errors.append(&mut loop_checker_expression(x.lhs.as_ref(), in_loop));
      errors.append(&mut loop_checker_expression(x.expr.as_ref(), in_loop));
    }
    Expression::Cast(x) => {
      errors.append(&mut loop_checker_expression(x.lhs.as_ref(), in_loop));
    }
    Expression::Literal(x) => {
      errors.append(&mut loop_checker_literal(x, in_loop));
    }
    Expression::Var(_) => {}
    Expression::FunctionCall(x) => {
      errors.append(&mut loop_checker_expression(x.target.as_ref(), in_loop));
      for arg in &x.arguments {
        errors.append(&mut loop_checker_expression(arg.as_ref(), in_loop))
      }
    }
    Expression::Error(_) => {}
  };
  errors
}

fn loop_checker_literal(literal: &Literal, in_loop: bool) -> Vec<OceanError> {
  let mut errors = Vec::new();
  match literal {
    Literal::Boolean(_) | Literal::Number(_) | Literal::String(_) => {}
    Literal::Array(x) => {
      for arg in &x.args {
        errors.append(&mut loop_checker_expression(arg.as_ref(), false));
      }
    }
    Literal::Tuple(x) => {
      for arg in &x.contents {
        errors.append(&mut loop_checker_expression(&arg.expression, false));
      }
    }
    Literal::Function(x) => {
      for ret_entry in &x.return_list.returns {
        match &ret_entry.expression {
          Some(exp) => {
            errors.append(&mut loop_checker_expression(exp.as_ref(), false));
          }
          None => {}
        }
      }
      for stmt in &x.function_body {
        errors.append(&mut loop_checker_stmt(stmt, false));
      }
    }
  }
  errors
}
