use crate::hydro::value::{Layout, Type, Value};
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

  pub fn create_value(&self, module_name: String) -> Value {
    Value::Layout(Layout::new(module_name, self.name.clone(), self.members.clone()))
  }

  pub fn to_type(&self, module_name: String) -> Type {
    let mut member_types = HashMap::new();
    for (member_name, member_value) in &self.members {
      member_types.insert(member_name.clone(), member_value.type_of());
    }
    Type::Layout(module_name, self.name.clone(), Some(member_types))
  }
}
