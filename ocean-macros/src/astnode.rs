use std::str::FromStr;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

pub fn ast_node_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  match &ast.data {
    Data::Struct(_) => quote! {
      impl AstNodeTrait for #name {
        fn get_token_index_range(&self) -> (TokenIndex, TokenIndex) {
          (self.metadata.start_token_index, self.metadata.end_token_index)
        }

        fn get_sibling_node(&self) -> Option<AstNodeIndex> {
          self.metadata.sibling_node
        }
      }
    },
    Data::Enum(data_enum) => {
      let get_token_index_range_variants = data_enum.variants.iter()
        .map(|v|
          proc_macro2::TokenStream::from_str(format!("  AstNode::{}(x) => x.get_token_index_range(),", v.ident.to_string()).as_str()).unwrap())
        .collect::<Vec<proc_macro2::TokenStream>>();
      let get_sibling_node_variants = data_enum.variants.iter()
        .map(|v|
          proc_macro2::TokenStream::from_str(format!("  AstNode::{}(x) => x.get_sibling_node(),", v.ident.to_string()).as_str()).unwrap())
        .collect::<Vec<proc_macro2::TokenStream>>();
      quote! {
        impl AstNodeTrait for #name {
          fn get_token_index_range(&self) -> (TokenIndex, TokenIndex) {
            match self {
              #(#get_token_index_range_variants)*
            }
          }

          fn get_sibling_node(&self) -> Option<AstNodeIndex> {
            match self {
              #(#get_sibling_node_variants)*
            }
          }
        }
      }
    },
    _ => panic!("This macro only covers structs and enums")
  }.into()
}