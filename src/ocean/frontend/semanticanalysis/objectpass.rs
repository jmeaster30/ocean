use std::cell::RefCell;
use std::rc::Rc;
use ocean_macros::New;
use uuid::Uuid;
use crate::ocean::frontend::ast::Program;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::util::errors::Error;

#[derive(Clone, Debug, New)]
pub struct ObjectPassContext {

}

pub trait ObjectPass {
  fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<ObjectPassContext>>) -> Result<Uuid, Vec<Error>>;
}

impl ObjectPass for Program {
  fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<ObjectPassContext>>) -> Result<Uuid, Vec<Error>> {
    Ok(Uuid::new_v4())
  }
}