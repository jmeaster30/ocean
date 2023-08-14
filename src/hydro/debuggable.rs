use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::instruction::Instruction;
use crate::hydro::module::Module;

pub trait Debuggable {
  fn debug(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception>;
}

impl Instruction {
  pub fn debug(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    Ok(false)
  }
}
