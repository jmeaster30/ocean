use std::collections::HashMap;
use crate::hydro::function::Target::{Index, Label};
use crate::hydro::instruction::*;
use crate::hydro::value::{Reference, Type, Value, VariableRef};

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
    Self {
      name,
      parameters,
      body,
      jump_labels: HashMap::new(),
    }
  }

  pub fn get_target_pointer(&self, target: Target) -> Result<usize, String>
  {
    match target {
      Label(label_name) => match self.jump_labels.get(label_name.as_str()) {
        Some(result) => Ok(*result),
        None => Err(format!("Label not found '{}'", label_name))
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
    self.body.push(Instruction::PushValue(PushValue {
      value: Value::Reference(Reference::Variable(VariableRef::new(
        variable_name.to_string(),
      ))),
    }));
    self
  }

  pub fn push_value(mut self, value: Value) -> Self {
    self.body.push(Instruction::PushValue(PushValue { value }));
    self
  }

  pub fn pop_value(mut self) -> Self {
    self.body.push(Instruction::PopValue(PopValue {}));
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

  pub fn add(mut self) -> Self {
    self.body.push(Instruction::Add(Add {}));
    self
  }
}
