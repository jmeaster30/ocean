#![allow(
  unused_variables,
  unused_imports,
  unreachable_patterns,
  unused_assignments,
  dead_code
)]

use self::loop_checker::loop_checker;
use self::symboltable::SymbolTable;
use self::type_checker::type_checker;

use super::errors::OceanError;
use super::parser::ast::*;

pub mod loop_checker;
pub mod operators;
pub mod symboltable;
pub mod type_checker;

pub fn semantic_check(program: &Program) -> (Option<SymbolTable>, Vec<OceanError>) {
  let mut errors = Vec::new();
  errors.append(&mut loop_checker(program));

  let (symbol_table, mut type_errors) = type_checker(program);

  errors.append(&mut type_errors);

  (Some(symbol_table), errors)
}
