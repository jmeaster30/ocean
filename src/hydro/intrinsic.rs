use std::collections::HashMap;
use crate::hydro::value::Type;

pub mod intrinsicmanager;

#[derive(Debug, Clone)]
pub struct Intrinsic {
  pub name: String,
  pub parameters: Vec<Type>,
  pub target_map: HashMap<String, String>,
}

impl Intrinsic {
  pub fn new(name: String, parameters: Vec<Type>, target_map: HashMap<String, String>) -> Self {
    Self {
      name,
      parameters,
      target_map
    }
  }

  pub fn get_intrinsic_code(&self, target: String) -> Result<String, String> {
    match self.target_map.get(target.as_str()) {
      None => Err(format!("Could not find code for target {}", target)),
      Some(code) => Ok(code.clone())
    }
  }
}