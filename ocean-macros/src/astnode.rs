use std::str::FromStr;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum};

fn create_variant_redirections(data_enum: &DataEnum, function_name: &str, function_args: &str) -> Vec<proc_macro2::TokenStream> {
  data_enum.variants.iter()
    .map(|v| proc_macro2::TokenStream::from_str(format!("  AstNode::{}(x) => x.{}({}),", v.ident.to_string(), function_name, function_args).as_str()).unwrap())
    .collect::<Vec<proc_macro2::TokenStream>>()
}

pub fn ast_node_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  match &ast.data {
    Data::Struct(_) => quote! {
      impl AstNodeTrait for #name {
        fn get_token_index_range(&self) -> (TokenIndex, TokenIndex) {
          (self.metadata.start_token_index, self.metadata.end_token_index)
        }

        fn set_start_index(&mut self, start_index: TokenIndex) {
          self.metadata.start_token_index = start_index;
        }

        fn set_end_index(&mut self, end_index: TokenIndex) {
          self.metadata.end_token_index = end_index;
        }

        fn get_parent_node(&self) -> Option<AstNodeIndex> {
          self.metadata.parent_node
        }

        fn set_parent_node(&mut self, node_index: AstNodeIndex) {
          self.metadata.parent_node = Some(node_index);
        }

        fn get_sibling_node(&self) -> Option<AstNodeIndex> {
          self.metadata.sibling_node
        }

        fn set_sibling_node(&mut self, node_index: AstNodeIndex) {
          self.metadata.sibling_node = Some(node_index);
        }
      }
    },
    Data::Enum(data_enum) => {
      let get_token_index_range_variants = create_variant_redirections(data_enum, "get_token_index_range", "");
      let set_start_index_variants = create_variant_redirections(data_enum, "set_start_index", "start_index");
      let set_end_index_variants = create_variant_redirections(data_enum, "set_end_index", "end_index");
      let get_parent_node_variants = create_variant_redirections(data_enum, "get_parent_node", "");
      let set_parent_node_variants = create_variant_redirections(data_enum, "set_parent_node", "node_index");
      let get_sibling_node_variants = create_variant_redirections(data_enum, "get_sibling_node", "");
      let set_sibling_node_variants = create_variant_redirections(data_enum, "set_sibling_node", "node_index");
      quote! {
        impl AstNodeTrait for #name {
          fn get_token_index_range(&self) -> (TokenIndex, TokenIndex) {
            match self {
              #(#get_token_index_range_variants)*
            }
          }

          fn set_start_index(&mut self, start_index: TokenIndex) {
            match self {
              #(#set_start_index_variants)*
            }
          }

          fn set_end_index(&mut self, end_index: TokenIndex) {
            match self {
              #(#set_end_index_variants)*
            }
          }

          fn get_parent_node(&self) -> Option<AstNodeIndex> {
            match self {
              #(#get_parent_node_variants)*
            }
          }

          fn set_parent_node(&mut self, node_index: AstNodeIndex) {
            match self {
              #(#set_parent_node_variants)*
            }
          }

          fn get_sibling_node(&self) -> Option<AstNodeIndex> {
            match self {
              #(#get_sibling_node_variants)*
            }
          }

          fn set_sibling_node(&mut self, node_index: AstNodeIndex) {
            match self {
              #(#set_sibling_node_variants)*
            }
          }
        }
      }
    },
    _ => panic!("This macro only covers structs and enums")
  }.into()
}