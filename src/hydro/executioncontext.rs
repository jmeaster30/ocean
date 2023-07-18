use super::value::Value;
use super::instruction::Instruction;

use std::collections::HashMap;

pub struct ExecutionContext {
  pub parent_execution_context: Option<Box<ExecutionContext>>,
  pub stack: Vec<Value>,
  pub program_counter: usize,
  pub instructions: Vec<Instruction>,
  pub variables: HashMap<String, Value>,
  pub return_value: Option<Value>,
}