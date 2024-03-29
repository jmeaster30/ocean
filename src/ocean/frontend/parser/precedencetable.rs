use std::collections::HashMap;
use ocean_macros::New;

#[derive(Clone, Debug, New)]
pub struct PrecedenceTable {
  #[default(HashMap::new())] prefix_op_table: HashMap<String, usize>,
  #[default(HashMap::new())] postfix_op_table: HashMap<String, usize>,
  #[default(HashMap::new())] binary_table: HashMap<String, (usize, usize)>,
}

impl PrecedenceTable {
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

  pub fn get_postfix_precedence(&self, operator: &String) -> usize {
    *self.postfix_op_table.get(operator).unwrap() // I don't think we need to handle if the operator exists or not here yet
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

  pub fn add_postfix_operator(&mut self, operator: &str, left_prec: usize) {
    self.postfix_op_table.insert(operator.to_string(), left_prec);
  }
}
