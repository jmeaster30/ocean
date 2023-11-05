use crate::hydro::function::Target::{Index, Label};
use crate::hydro::instruction::*;
use crate::hydro::value::{Reference, Type, Value, VariableRef};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Target {
  Label(String),
  Index(usize),
}

#[derive(Debug, Clone)]
pub struct Function {
  pub name: String,
  pub parameters: Vec<Type>,
  pub body: Vec<Instruction>,
  pub jump_labels: HashMap<String, usize>,
}

impl Function {
  pub fn new(name: String, parameters: Vec<Type>, body: Vec<Instruction>) -> Self {
    Self { name, parameters, body, jump_labels: HashMap::new() }
  }

  pub fn get_target_pointer(&self, target: Target) -> Result<usize, String> {
    match target {
      Label(label_name) => match self.jump_labels.get(label_name.as_str()) {
        Some(result) => Ok(*result),
        None => Err(format!("Label not found '{}'", label_name)),
      },
      Index(result) => Ok(result),
    }
  }

  pub fn add_label(&mut self, name: String, target: usize) {
    self.jump_labels.insert(name, target);
  }

  pub fn build(name: &str) -> Self {
    Function::new(name.to_string(), Vec::new(), Vec::new())
  }

  pub fn parameter(mut self, param_type: Type) -> Self {
    self.parameters.insert(0, param_type);
    self
  }

  pub fn inst(mut self, instruction: Instruction) -> Self {
    self.body.push(instruction);
    self
  }

  pub fn var_ref(mut self, variable_name: &str) -> Self {
    self.body.push(Instruction::PushValue(Push { value: Value::Reference(Reference::Variable(VariableRef::new(variable_name.to_string()))) }));
    self
  }

  pub fn push(mut self, value: Value) -> Self {
    self.body.push(Instruction::PushValue(Push { value }));
    self
  }

  pub fn pop(mut self) -> Self {
    self.body.push(Instruction::PopValue(Pop {}));
    self
  }

  pub fn duplicate(mut self, offset: usize) -> Self {
    self.body.push(Instruction::Duplicate(Duplicate { offset }));
    self
  }

  pub fn swap(mut self) -> Self {
    self.body.push(Instruction::Swap(Swap {}));
    self
  }

  pub fn rotate(mut self, size: i64) -> Self {
    self.body.push(Instruction::Rotate(Rotate { size }));
    self
  }

  pub fn add(mut self) -> Self {
    self.body.push(Instruction::Add(Add {}));
    self
  }

  pub fn subtract(mut self) -> Self {
    self.body.push(Instruction::Subtract(Subtract {}));
    self
  }

  pub fn multiply(mut self) -> Self {
    self.body.push(Instruction::Multiply(Multiply {}));
    self
  }

  pub fn divide(mut self) -> Self {
    self.body.push(Instruction::Divide(Divide {}));
    self
  }

  pub fn modulo(mut self) -> Self {
    self.body.push(Instruction::Modulo(Modulo {}));
    self
  }

  pub fn leftshift(mut self) -> Self {
    self.body.push(Instruction::LeftShift(LeftShift {}));
    self
  }

  pub fn rightshift(mut self) -> Self {
    self.body.push(Instruction::RightShift(RightShift {}));
    self
  }

  pub fn ret(mut self) -> Self {
    self.body.push(Instruction::Return(Return {}));
    self
  }

  pub fn call(mut self) -> Self {
    self.body.push(Instruction::Call(Call {}));
    self
  }

  pub fn load(mut self) -> Self {
    self.body.push(Instruction::Load(Load {}));
    self
  }

  pub fn store(mut self) -> Self {
    self.body.push(Instruction::Store(Store {}));
    self
  }


}
