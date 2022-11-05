use super::{instruction::Instruction, symboltable::HydroSymbolTable};
use crate::util::errors::OceanError;

pub fn hydro_semantic_check(
  instructions: &Vec<Instruction>,
) -> (Vec<Instruction>, Option<HydroSymbolTable>, Vec<OceanError>) {
  (instructions.clone(), None, vec![])
}
