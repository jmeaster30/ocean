use super::symboltable::Symbol;

pub fn get_binary_operator_resulting_type(
  operator: String,
  lhs: &Symbol,
  rhs: &Symbol,
) -> Option<Symbol> {
  match (operator.as_str(), lhs, rhs) {
    (_, _, _) => todo!(),
  }
}
