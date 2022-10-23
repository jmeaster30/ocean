use super::instruction::Instruction;
use crate::compiler::{errors::OceanError, semantic_analysis::symboltable::*};

pub fn hydro_semantic_check(
  instructions: &Vec<Instruction>,
) -> (Vec<Instruction>, Option<SymbolTable>, Vec<OceanError>) {
  (instructions.clone(), None, vec![])
}
