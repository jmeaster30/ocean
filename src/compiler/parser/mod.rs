pub mod ast;
pub mod display;
pub mod span;

use crate::compiler::OceanError;
use crate::compiler::TokenStack;
use ast::*;

pub fn parse(tokens: &mut TokenStack) -> (Option<Program>, Vec<OceanError>) {
  (None, Vec::new())
}
