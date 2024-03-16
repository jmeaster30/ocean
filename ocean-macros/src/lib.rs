extern crate proc_macro;
use proc_macro::TokenStream;
use std::cmp::min;
use std::str::FromStr;
use quote::{quote, ToTokens};
use syn::{Data, Expr, Fields, parse_macro_input, Token};
use syn::parse::Parse;

#[proc_macro_derive(Debuggable)]
pub fn debuggable_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_debuggable_macro(&ast)
}

fn impl_debuggable_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl Debuggable for #name {
      fn debug(&self, compilation_unit: &CompilationUnit, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
        let metric_name = stringify!(#name).to_lowercase();
        debug_context.metric_tracker.start(context.get_call_stack(), metric_name.clone());
        let result = self.execute(compilation_unit, context);
        debug_context.metric_tracker.stop(context.get_call_stack(), metric_name);
        return result;
      }
    }
  };
  gen.into()
}

#[proc_macro_derive(AstNode)]
pub fn ast_node_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_ast_node_macro(&ast)
}

fn impl_ast_node_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl AstNode for #name {
      fn get_as_expression(&self) -> Expression {
        Expression::AstNode(self.clone())
      }
    }
  };
  gen.into()
}

#[proc_macro_derive(New, attributes(default))]
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
  let ast = syn::parse(input).unwrap();
  impl_new_macro(&ast)
}

fn impl_new_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  match &ast.data {
    Data::Struct(struct_data) => {
      let mut token_stream = "impl ".to_string();
      token_stream += name.to_string().as_str();
      token_stream += " {";

      let mut typed_args = Vec::new();

      match &struct_data.fields {
        Fields::Named(named_fields) => {
          for field in &named_fields.named {
            let default_value_attribute = field.attrs.iter().filter(|x| x.path.segments.len() == 1 && x.path.segments[0].ident == "default").nth(0);
            let optional_default_value = if let Some(attr) = default_value_attribute {
              Some(attr.tokens.to_token_stream())
            } else {
              None
            };

            match &field.ident {
              Some(field_name) => {
                typed_args.push((field_name.to_string(), field.ty.to_token_stream(), optional_default_value))
              }
              None => {}
            }
          }
        }
        _ => {}
      }

      token_stream += "pub fn new(";
      for (idx, (arg_name, arg_type, arg_default_value)) in typed_args.iter().enumerate() {
        if arg_default_value.is_some() { continue }

        token_stream += arg_name.to_string().as_str();
        token_stream += ": ";
        token_stream += arg_type.to_string().as_str();
        if idx != typed_args.len() - 1 {
          token_stream += ", ";
        }
      }
      token_stream += ") -> Self { Self { ";
      for (idx, (arg_name, _, arg_default_value)) in typed_args.iter().enumerate() {
        token_stream += arg_name;
        if let Some(value) = arg_default_value {
          token_stream += ": ";
          token_stream += value.to_string().as_str();
        }
        if idx != typed_args.len() - 1 {
          token_stream += ", "
        }
      }
      token_stream += "} }\n";
      token_stream += "}";

      TokenStream::from_str(&token_stream).unwrap()
    }
    _ => TokenStream::new()
  }
}

struct MacroInput {
  a: Expr,
  comma: Token![,],
  b: Expr,
}

impl Parse for MacroInput {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self {
      a: input.parse()?,
      comma: input.parse()?,
      b: input.parse()?,
    })
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