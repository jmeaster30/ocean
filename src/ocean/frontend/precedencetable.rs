use std::collections::HashMap;

pub struct PrecedenceTable {
  prefix_op_table: Vec<HashMap<String, u32>>,
  postfix_op_table: Vec<HashMap<String, u32>>,
  binary_table: Vec<HashMap<String, (u32, u32)>>,
  ternary_table: Vec<HashMap<(String, String), (u32, u32, u32)>>
}

impl PrecedenceTable {
  pub fn new() -> Self {
    Self {
      prefix_op_table: Vec::new(),
      postfix_op_table: Vec::new(),
      binary_table: Vec::new(),
      ternary_table: Vec::new(),
    }
  }

  pub fn add_prefix_operator(&mut self, op: String, power: Option<u32>) -> Result<(), String> {
    match self.prefix_op_table.last_mut() {
      Some(map) =>  match power {
        Some(power) => map.insert(op, power),
        None => None/*match self.prefix_binding_power(&op) {
          Some(_) => None, // do nothing so we piggy back off the already existing one
          None => map.insert(op, 5), // TODO figure out default precedence value
        },*/
      },
      None => None
    };
    Ok(())
  }

  pub fn add_postfix_operator(&mut self, op: String, power: Option<u32>) -> Result<(), String> {
    match self.postfix_op_table.last_mut() {
      Some(map) =>  match power {
        Some(power) => map.insert(op, power),
        None => None/*match self.postfix_binding_power(&op) {
          Some(_) => None, // do nothing so we piggy back off the already existing one
          None => map.insert(op, 5), // TODO figure out default precedence value
        },*/
      },
      None => None
    };
    Ok(())
  }

  pub fn add_binary_operator(&mut self, op: String, power: Option<(u32, u32)>) -> Result<(), String> {
    match self.binary_table.last_mut() {
      Some(map) =>  match power {
        Some(power) => map.insert(op, power),
        None => None/*match self.binary_binding_power(&op) {
          Some(_) => None, // do nothing so we piggy back off the already existing one
          None => map.insert(op, (5, 6)), // TODO figure out default precedence value
        },*/
      },
      None => None
    };
    Ok(())
  }

  pub fn add_ternary_operator(&mut self, op: (String, String), power: Option<(u32, u32, u32)>) -> Result<(), String> {
    match self.ternary_table.last_mut() {
      Some(map) =>  match power {
        Some(power) => map.insert(op, power),
        None => None/*match self.ternary_binding_power(&op) {
          Some(_) => None, // do nothing so we piggy back off the already existing one
          None => map.insert(op, (5, 6, 6)), // TODO figure out default precedence value
        },*/
      },
      None => None
    };
    Ok(())
  }

  pub fn add_scope(&mut self) {
    self.prefix_op_table.push(HashMap::new());
    self.postfix_op_table.push(HashMap::new());
    self.binary_table.push(HashMap::new());
    self.ternary_table.push(HashMap::new());
  }

  pub fn remove_scope(&mut self) {
    self.prefix_op_table.pop();
    self.postfix_op_table.pop();
    self.binary_table.pop();
    self.ternary_table.pop();
  }

  pub fn prefix_binding_power(&self, op: &String) -> Option<u32> {
    let mut result = None;
    for hashmap in &self.prefix_op_table {
      match hashmap.get(op) {
        Some(bp) => result = Some(*bp),
        None => {},
      }
    };
    result
  }

  pub fn postfix_binding_power(&self, op: &String) -> Option<u32> {
    let mut result = None;
    for hashmap in &self.postfix_op_table {
      match hashmap.get(op) {
        Some(bp) => result = Some(*bp),
        None => {},
      }
    };
    result
  }

  pub fn binary_binding_power(&self, op: &String) -> Option<(u32, u32)> {
    let mut result = None;
    for hashmap in &self.binary_table {
      match hashmap.get(op) {
        Some(bp) => result = Some(*bp),
        None => {},
      }
    };
    result
  }

  pub fn ternary_binding_power(&self, op: &(String, String)) -> Option<(u32, u32, u32)> {
    let mut result = None;
    for hashmap in &self.ternary_table {
      match hashmap.get(op) {
        Some(bp) => result = Some(*bp),
        None => {},
      }
    };
    result
  }
}