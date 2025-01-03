extern crate proc_macro;
mod debuggable;
mod new;
mod astnode;

use debuggable::debuggable_macro;
use new::new_macro;

use proc_macro::TokenStream;
use std::cmp::min;
use std::str::FromStr;
use quote::quote;
use syn::{Expr, parse_macro_input, Token};
use syn::parse::Parse;
use crate::astnode::ast_node_macro;

#[proc_macro_derive(Debuggable)]
pub fn debuggable_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  debuggable_macro(&ast)
}

#[proc_macro_derive(AstNode)]
pub fn ast_node_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  ast_node_macro(&ast)
}

#[proc_macro_derive(New, attributes(default))]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  new_macro(&ast)
}

struct MacroInput {
  a: Expr,
  b: Expr,
}

impl Parse for MacroInput {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let a = input.parse::<Expr>()?;
    input.parse::<Token![,]>()?;
    let b = input.parse::<Expr>()?;
    Ok(Self { a, b })
  }
}

#[proc_macro]
pub fn borrow_and_drop(tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as MacroInput);

  let a = &input.a;
  let b = &input.b;

  quote! {
    {
      let borrow = (#a).borrow();
      #b
    }
  }.into()
}

#[proc_macro]
pub fn borrow_mut_and_drop(tokens: TokenStream) -> TokenStream {
  let input = parse_macro_input!(tokens as MacroInput);

  let a = &input.a;
  let b = &input.b;

  quote! {
    {
      let mut borrow_mut = (#a).borrow_mut();
      #b
    }
  }.into()
}

// TODO I need to think much more deeply about if I want wrapping behavior for these operations
#[proc_macro]
pub fn make_add_operations(item: TokenStream) -> TokenStream {
  // a_value and b_value will be assumed to be resolved for ease of use
  let mut token_stream = "
match (a, b) {
".to_string();

  let unsigned = vec![8, 16, 32, 64, 128];
  let signed = vec![8, 16, 32, 64, 128];
  let floated = vec![32, 64];

  for a in &unsigned {
    for b in &unsigned {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Unsigned{}(right)) => Value::Unsigned{}((std::num::Wrapping(left as {}) {} std::num::Wrapping(right as {})).0),\n",
        a, b, max, format!("u{}", max), item, format!("u{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &signed {
      let max = if a < b { b } else { a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Signed{}(right)) => Value::Signed{}((std::num::Wrapping(left as {}) {} std::num::Wrapping(right as {})).0),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str()
    }
  }

  for a in &signed {
    for b in &unsigned {
      let max = if a <= b { min(128, b * 2) } else { *a };
      token_stream += format!(
        "(Value::Signed{}(left), Value::Unsigned{}(right)) => Value::Signed{}((std::num::Wrapping(left as {}) {} std::num::Wrapping(right as {})).0),\n",
        a, b, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Signed{}(right)) => Value::Signed{}((std::num::Wrapping(left as {}) {} std::num::Wrapping(right as {})).0),\n",
        b, a, max, format!("i{}", max), item, format!("i{}", max)
      ).as_str();
    }
  }

  for a in &floated {
    for b in &floated {
      let max = if a <= b { b } else { a };
      token_stream += format!(
        "(Value::Float{}(left), Value::Float{}(right)) => Value::Float{}((left as f{}) {} (right as f{})),\n",
        a, b, max, max, item, max
      ).as_str();
    }
  }

  for a in &floated {
    for b in &unsigned {
      // TODO I think this is wrong
      token_stream += format!(
        "(Value::Float{}(left), Value::Unsigned{}(right)) => Value::Float{}((left as f{}) {} (right as f{})),\n",
        a, b, a, a, item, a
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Float{}(right)) => Value::Float{}((left as f{}) {} (right as f{})),\n",
        b, a, a, a, item, a
      ).as_str();
    }
  }

  for a in &floated {
    for b in &signed {
      // TODO I think this is wrong
      token_stream += format!(
        "(Value::Float{}(left), Value::Signed{}(right)) => Value::Float{}((left as f{}) {} (right as f{})),\n",
        a, b, a, a, item, a
      ).as_str();
      token_stream += format!(
        "(Value::Signed{}(left), Value::Float{}(right)) => Value::Float{}((left as f{}) {} (right as f{})),\n",
        b, a, a, a, item, a
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
  let floated = vec![32, 64];

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

  for a in &floated {
    for b in &floated {
      let max = if a <= b { b } else { a };
      token_stream += format!(
        "(Value::Float{}(left), Value::Float{}(right)) => Value::Boolean((left as f{}) {} (right as f{})),\n",
        a, b, max, item, max
      ).as_str();
    }
  }

  for a in &floated {
    for b in &unsigned {
      // TODO I think this is wrong
      token_stream += format!(
        "(Value::Float{}(left), Value::Unsigned{}(right)) => Value::Boolean((left as f{}) {} (right as f{})),\n",
        a, b, a, item, a
      ).as_str();
      token_stream += format!(
        "(Value::Unsigned{}(left), Value::Float{}(right)) => Value::Boolean((left as f{}) {} (right as f{})),\n",
        b, a, a, item, a
      ).as_str();
    }
  }

  for a in &floated {
    for b in &signed {
      // TODO I think this is wrong
      token_stream += format!(
        "(Value::Float{}(left), Value::Signed{}(right)) => Value::Boolean((left as f{}) {} (right as f{})),\n",
        a, b, a, item, a
      ).as_str();
      token_stream += format!(
        "(Value::Signed{}(left), Value::Float{}(right)) => Value::Boolean((left as f{}) {} (right as f{})),\n",
        b, a, a, item, a
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
