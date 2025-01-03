use proc_macro::TokenStream;
use std::str::FromStr;
use quote::ToTokens;
use syn::{Attribute, Data, Expr, Fields, Lit, Meta};

pub fn new_macro(ast: &syn::DeriveInput) -> TokenStream {
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
            let default_value_attribute : Option<&Attribute> = field.attrs.iter().filter(|x| x.path().segments.len() == 1 && x.path().segments[0].ident == "default").nth(0);
            let optional_default_value = if let Some(attr) = default_value_attribute {
              match &attr.meta {
                Meta::Path(_) => None,
                Meta::List(list) => Some(list.tokens.clone()),
                Meta::NameValue(name_value) => match name_value.value.clone() {
                  Expr::Lit(lit) => match lit.lit {
                    Lit::Str(str) => match TokenStream::from_str(str.value().as_str()) {
                      Ok(stream) => Some(proc_macro2::TokenStream::from(stream)),
                      Err(_) => None, // TODO report error properly
                    },
                    _ => None,  // TODO report error properly
                  }
                  _ => None,  // TODO report error properly
                },
              }
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