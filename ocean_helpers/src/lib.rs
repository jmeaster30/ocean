extern crate proc_macro;
use proc_macro::TokenStream;
use std::cmp::min;
use std::str::FromStr;

#[proc_macro]
pub fn make_add_operations(item: TokenStream) -> TokenStream {
  // a_value and b_value will be assumed to be resolved for ease of use
  let mut token_stream = "
match (a, b) {
".to_string();

  let unsigned = vec![8, 16, 32, 64, 128];
  let signed = vec![8, 16, 32, 64, 128];

  for a in &unsigned {
    for b in &unsigned {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Unsigned{}(right)) => Value::Unsigned{}(left as {} {} right as {}),\n",
        a, b, max, format!("u{}", max), item, format!("u{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &signed {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Signed{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &unsigned {
      let max = if a <= b { min(128, b * 2) } else { *a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Unsigned{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Signed{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        b, a, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
    }
  }

  token_stream += format!("
  (left, right) => panic!(\"Operator not defined on provided types :( '{{:?}}' {} '{{:?}}'\", left, right)\
}}", item).as_str();

  match TokenStream::from_str(&token_stream) {
    Ok(x) => x,
    Err(e) => panic!("{}", e),
  }
}

#[proc_macro]
pub fn make_bit_operations(item: TokenStream) -> TokenStream {
  // a_value and b_value will be assumed to be resolved for ease of use
  let mut token_stream = "
match (a, b) {
".to_string();

  let unsigned = vec![8, 16, 32, 64, 128];
  let signed = vec![8, 16, 32, 64, 128];

  for a in &unsigned {
    for b in &unsigned {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Unsigned{}(right)) => Value::Unsigned{}(left as {} {} right as {}),\n",
        a, b, max, format!("u{}", max), item, format!("u{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &signed {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Signed{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &unsigned {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Unsigned{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Signed{}(right)) => Value::Signed{}(left as {} {} right as {}),\n",
        b, a, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
    }
  }

  token_stream += format!("
  (left, right) => panic!(\"Operator not defined on provided types :( '{{:?}}' {} '{{:?}}'\", left, right)\
}}", item).as_str();

  match TokenStream::from_str(&token_stream) {
    Ok(x) => x,
    Err(e) => panic!("{}", e),
  }
}

#[proc_macro]
pub fn make_comparison_operations(item: TokenStream) -> TokenStream {
  // a_value and b_value will be assumed to be resolved for ease of use
  let mut token_stream = "
match (a, b) {
".to_string();

  let unsigned = vec![8, 16, 32, 64, 128];
  let signed = vec![8, 16, 32, 64, 128];

  for a in &unsigned {
    for b in &unsigned {
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Unsigned{}(right)) => Value::Boolean((left as u128) {} (right as u128)),\n",
        a, b, item,
      ).as_str()
    }
  }

  for a in &signed {
    for b in &signed {
      token_stream += format!(
        "(Value::Signed{}(left), Value::Signed{}(right)) => Value::Boolean((left as i128) {} (right as i128)),\n",
        a, b, item,
      ).as_str()
    }
  }

  for a in &signed {
    for b in &unsigned {
      token_stream += format!(
        "(Value::Signed{}(left), Value::Unsigned{}(right)) => Value::Boolean((left as i128) {} (right as i128)),\n",
        a, b, item
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Signed{}(right)) => Value::Boolean((left as i128) {} (right as i128)),\n",
        b, a, item
      ).as_str();
    }
  }

  token_stream += format!("
  (left, right) => panic!(\"Operator not defined on provided types :( '{{:?}}' {} '{{:?}}'\", left, right)\
}}", item).as_str();

  match TokenStream::from_str(&token_stream) {
    Ok(x) => x,
    Err(e) => panic!("{}", e),
  }
}