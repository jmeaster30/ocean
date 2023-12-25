use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PrecedenceTable {
  prefix_op_table: HashMap<String, usize>,
  postfix_op_table: HashMap<String, usize>,
  binary_table: HashMap<String, (usize, usize)>,
}

impl PrecedenceTable {
  pub fn new() -> Self {
    Self {
      prefix_op_table: HashMap::new(),
      postfix_op_table: HashMap::new(),
      binary_table: HashMap::new(),
    }
  }

  pub fn is_prefix_operator(&self, operator: &String) -> bool {
    self.prefix_op_table.contains_key(operator)
  }

  pub fn is_postfix_operator(&self, operator: &String) -> bool {
    self.postfix_op_table.contains_key(operator)
  }

  pub fn is_binary_operator(&self, operator: &String) -> bool {
    self.binary_table.contains_key(operator)
  }

  pub fn get_prefix_precedence(&self, operator: &String) -> usize {
    *self.prefix_op_table.get(operator).unwrap() // I don't think we need to handle if the operator exists or not here yet
  }

  pub fn get_binary_precedence(&self, operator: &String) -> (usize, usize) {
    *self.binary_table.get(operator).unwrap() // TODO handle errors
  }

  pub fn add_binary_operator(&mut self, operator: &str, left_prec: usize, right_prec: usize) {
    self.binary_table.insert(operator.to_string(), (left_prec, right_prec));
  }

  pub fn add_prefix_operator(&mut self, operator: &str, right_prec: usize) {
    self.prefix_op_table.insert(operator.to_string(), right_prec);
  }
}
