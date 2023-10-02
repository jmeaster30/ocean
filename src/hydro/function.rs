use crate::hydro::instruction::{
  Add, ArrayIndex, Call, Instruction, Load, PopValue, PushValue, Return, Store,
};
use crate::hydro::value::{Reference, Value, VariableRef};

#[derive(Debug, Clone)]
pub struct Function {
  pub name: String,
  pub parameters: Vec<String>,
  pub body: Vec<Instruction>,
}

impl Function {
  pub fn new(name: String, parameters: Vec<String>, body: Vec<Instruction>) -> Self {
    Self {
      name,
      parameters,
      body,
    }
  }

  pub fn build(name: &str) -> Self {
    Function::new(name.to_string(), Vec::new(), Vec::new())
  }

  pub fn parameter(mut self, name: &str) -> Self {
    self.parameters.push(name.to_string());
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

  // TODO better name?
  pub fn return_(mut self) -> Self {
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

  pub fn index(mut self) -> Self {
    self.body.push(Instruction::ArrayIndex(ArrayIndex {}));
    self
  }

  pub fn add(mut self) -> Self {
    self.body.push(Instruction::Add(Add {}));
    self
  }
}
