#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  ArraySymbol, OceanType, Symbol, SymbolTable, SymbolTableVarEntry,
  TupleSymbol,
};

pub fn type_checker(program: &Program) -> (SymbolTable, Vec<OceanError>) {
  let mut symbol_table = SymbolTable::hard_scope(None);
  let mut errors = Vec::new();

  for statement in &program.statements {
    type_checker_stmt(statement, &mut symbol_table, &mut errors)
  }

  (symbol_table, errors)
}

pub fn type_checker_stmt(
  statement: &Statement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  match statement {
    Statement::Error(x) => { /* I don't think I need to do anything here */ }
    Statement::Macro(x) => todo!(),
    Statement::Continue(x) => {}
    Statement::Break(x) => {}
    Statement::Return(x) => {}
    Statement::PackDec(x) => todo!(),
    Statement::UnionDec(x) => todo!(),
    Statement::VarDec(var_dec) => type_checker_var_dec(var_dec, symbol_table, errors),
    Statement::Cast(x) => todo!(),
    Statement::Match(x) => panic!(),
    Statement::Use(x) => todo!(),
    Statement::If(x) => todo!(),
    Statement::ForLoop(x) => todo!(),
    Statement::WhileLoop(x) => todo!(),
    Statement::InfiniteLoop(x) => todo!(),
    Statement::Expression(x) => {
      get_expression_type(&x.expression, symbol_table, errors);
    }
  }
}

pub fn get_var_type(
  var: &Var,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  todo!()
}

pub fn get_untyped_var_type(
  var: &UntypedVar,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  todo!()
}

pub fn type_checker_var_dec(
  var_dec: &VarDecStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  todo!()
}

pub fn get_expression_type(
  expr: &Expression,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  match expr {
    Expression::Binary(binary) => todo!(),
    Expression::Prefix(prefix) => todo!(),
    Expression::Postfix(postfix) => todo!(),
    Expression::Member(_) => {
      todo!("should I think about doing the functions as member variables at this point?")
    }
    Expression::ArrayAccess(access) => {
      let target_sym = get_expression_type(access.lhs.as_ref(), symbol_table, errors);
      let index_sym = get_expression_type(access.expr.as_ref(), symbol_table, errors);
      match (target_sym, index_sym) {
        (Some(target), Some(index)) => todo!(),
        _ => None,
      }
    }
    Expression::Cast(_) => todo!(),
    Expression::Literal(x) => {
      let t = get_literal_type(x, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::Var(var) => {
      let t = get_untyped_var_type(var, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::FunctionCall(x) => {
      let t = get_function_call_type(x, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::Error(_) => todo!(),
  }
}

pub fn get_literal_type(
  literal: &Literal,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  match literal {
    Literal::Boolean(_) => Some(Symbol::Base(OceanType::Bool)),
    Literal::Number(x) => todo!(),
    Literal::String(x) => todo!(),
    Literal::Array(x) => todo!(),
    Literal::Tuple(x) => todo!(),
    Literal::Function(x) => todo!(),
  }
}

pub fn get_function_call_type(
  call: &FunctionCall,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  let call_target = get_expression_type(call.target.as_ref(), symbol_table, errors);
  match call_target {
    Some(Symbol::Function(x)) => todo!(),
    Some(_) | None => todo!()
  }
}
