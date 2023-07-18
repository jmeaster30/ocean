extern crate proc_macro;
use proc_macro::TokenStream;
use std::str::FromStr;

#[proc_macro]
pub fn make_add_numbers(_item: TokenStream) -> TokenStream {
  // a_value and b_value will be assumed to be resolved for ease of use
  let mut token_stream = TokenStream::from_str("
match (a, b) {
  _ => panic!(\"Operator not defined on provided types :(\")
}
").unwrap();

  token_stream
}