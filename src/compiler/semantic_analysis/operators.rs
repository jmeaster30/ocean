use super::symboltable::*;

pub fn get_postfix_operator_type(operator: String, input: &Symbol) -> Option<Symbol> {
  match (operator.as_str(), input) {
    _ => None,
  }
}

pub fn get_prefix_operator_type(operator: String, input: &Symbol) -> Option<Symbol> {
  match (operator.as_str(), input) {
    ("!", Symbol::Base(OceanType::Bool)) => Some(Symbol::Base(OceanType::Bool)),

    ("-", Symbol::Base(OceanType::Unsigned(x))) => Some(Symbol::Base(OceanType::Signed(*x * 2))),
    ("-", Symbol::Base(OceanType::Signed(x))) => Some(Symbol::Base(OceanType::Signed(*x))),
    ("-", Symbol::Base(OceanType::Float(x))) => Some(Symbol::Base(OceanType::Float(*x))),

    ("~", Symbol::Base(OceanType::String)) => Some(Symbol::Base(OceanType::String)),
    ("~", Symbol::Base(OceanType::Char)) => Some(Symbol::Base(OceanType::Char)),
    ("~", Symbol::Base(OceanType::Unsigned(x))) => Some(Symbol::Base(OceanType::Unsigned(*x))),
    ("~", Symbol::Base(OceanType::Signed(x))) => Some(Symbol::Base(OceanType::Signed(*x))),
    ("~", Symbol::Base(OceanType::Float(x))) => Some(Symbol::Base(OceanType::Float(*x))),
    _ => None,
  }
}

pub fn get_infix_operator_type(
  operator: String,
  lhs_symbol: &Symbol,
  rhs_symbol: &Symbol,
) -> Option<Symbol> {
  match (operator.as_str(), lhs_symbol, rhs_symbol) {
    ("+", Symbol::Base(OceanType::String), Symbol::Base(OceanType::String)) => {
      Some(Symbol::Base(OceanType::String))
    }
    ("+", Symbol::Base(OceanType::Char), Symbol::Base(OceanType::String)) => {
      Some(Symbol::Base(OceanType::String))
    }
    ("+", Symbol::Base(OceanType::String), Symbol::Base(OceanType::Char)) => {
      Some(Symbol::Base(OceanType::String))
    }
    ("+", Symbol::Base(OceanType::Char), Symbol::Base(OceanType::Char)) => {
      Some(Symbol::Base(OceanType::String))
    }

    ("+", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("-", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("*", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("/", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("//", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("%", Symbol::Base(x), Symbol::Base(y)) => get_greater_type(x, y),
    ("..", Symbol::Base(a), Symbol::Base(b)) => {
      let result = get_greater_type(a, b);
      match result {
        Some(Symbol::Base(OceanType::Signed(_)))
        | Some(Symbol::Base(OceanType::Unsigned(_)))
        | Some(Symbol::Base(OceanType::Char))
        | Some(Symbol::Base(OceanType::String)) => Some(Symbol::Array(ArraySymbol::new(
          Box::new(result.unwrap()),
          Box::new(Symbol::Base(OceanType::Unsigned(64))),
        ))),
        _ => None,
      }
    }
    ("..<", Symbol::Base(a), Symbol::Base(b)) => {
      let result = get_greater_type(a, b);
      match result {
        Some(Symbol::Base(OceanType::Signed(_)))
        | Some(Symbol::Base(OceanType::Unsigned(_)))
        | Some(Symbol::Base(OceanType::Char))
        | Some(Symbol::Base(OceanType::String)) => Some(Symbol::Array(ArraySymbol::new(
          Box::new(result.unwrap()),
          Box::new(Symbol::Base(OceanType::Unsigned(64))),
        ))),
        _ => None,
      }
    }
    ("==", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    ("!=", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    ("<", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    (">", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    ("<=", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    (">=", Symbol::Base(a), Symbol::Base(b)) => {
      if is_compat_type(a, b) {
        Some(Symbol::Base(OceanType::Bool))
      } else {
        None
      }
    }
    ("|", Symbol::Base(a), Symbol::Base(b)) => get_greater_type(a, b),
    ("&", Symbol::Base(a), Symbol::Base(b)) => get_greater_type(a, b),
    ("^", Symbol::Base(a), Symbol::Base(b)) => get_greater_type(a, b),
    ("||", Symbol::Base(OceanType::Bool), Symbol::Base(OceanType::Bool)) => Some(Symbol::Base(OceanType::Bool)),
    ("&&", Symbol::Base(OceanType::Bool), Symbol::Base(OceanType::Bool)) => Some(Symbol::Base(OceanType::Bool)),
    ("^^", Symbol::Base(OceanType::Bool), Symbol::Base(OceanType::Bool)) => Some(Symbol::Base(OceanType::Bool)),
    (">>", Symbol::Base(x), Symbol::Base(OceanType::Unsigned(y))) => Some(Symbol::Base(x.clone())),
    ("<<", Symbol::Base(x), Symbol::Base(OceanType::Unsigned(y))) => Some(Symbol::Base(x.clone())),
    _ => None,
  }
}

/*

  ++ -- >.
  | & ^
  || && ^^
  << >>
  < > <= >=
  == !=

*/
