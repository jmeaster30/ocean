use self::loop_checker::loop_checker;
use self::symboltable::SymbolTable;

use super::errors::OceanError;
use super::parser::ast::*;

pub mod loop_checker;
pub mod symboltable;

pub fn semantic_check(program: &Program) -> (Option<SymbolTable>, Vec<OceanError>) {
  let loop_errors = loop_checker(program);
  (None, loop_errors)
}
