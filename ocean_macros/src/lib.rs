extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro]
pub fn make_add_numbers(_item: TokenStream) -> TokenStream {
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
        "(Value::Unsigned{}(left), Value::Unsigned{}(right)) => Value::Unsigned{}(left as {} + right as {}),\n",
        a, b, max, format!("u{}", max), format!("u{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &signed {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Signed{}(right)) => Value::Signed{}(left as {} + right as {}),\n",
        a, b, max, format!("i{}", max), format!("i{}", max)
      ).as_str()
    }
  }

  token_stream += "
  _ => panic!(\"Operator not defined on provided types :(\")
}";

  match TokenStream::from_str(&token_stream) {
    Ok(x) => x,
    Err(e) => panic!("{}", e),
  }
}