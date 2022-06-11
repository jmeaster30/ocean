pub mod ast;
pub mod span;

use crate::compiler::OceanError;
use crate::compiler::TokenStack;
use ast::*;

pub fn parse(tokens: TokenStack) -> (Program, Vec<OceanError>) {
  (None, Vec::new())
}
