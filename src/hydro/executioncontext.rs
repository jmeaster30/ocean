use super::value::Value;
use super::instruction::Instruction;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ExecutionContext {
  pub parent_execution_context: Option<Rc<RefCell<ExecutionContext>>>,
  pub stack: Vec<Value>,
  pub program_counter: usize,
  pub instructions: Vec<Instruction>,
  pub variables: HashMap<String, Value>,
  pub return_value: Option<Value>,
}