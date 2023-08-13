use crate::hydro::value::{Layout, Value};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LayoutTemplate {
  pub name: String,
  pub members: HashMap<String, Value>,
}

impl LayoutTemplate {
  pub fn new(name: String, members: HashMap<String, Value>) -> Self {
    Self { name, members }
  }

  pub fn build(name: &str) -> Self {
    Self {
      name: name.to_string(),
      members: HashMap::new(),
    }
  }

  pub fn member(mut self, name: &str, value: Value) -> Self {
    self.members.insert(name.to_string(), value);
    self
  }

  pub fn create_value(&self) -> Value {
    Value::Layout(Layout::new(self.members.clone()))
  }
}
