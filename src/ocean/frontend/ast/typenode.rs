use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;

use ocean_macros::New;

#[derive(Clone, Debug)]
pub enum Type {
  Unknown,
  Base(BaseType),
  Custom(CustomType),
  Auto(AutoType),
  Lazy(LazyType),
  Ref(RefType),
  Mutable(MutType),
  Function(FunctionType),
  Array(ArrayType),
  VariableType(VariableType),
  TupleType(TupleType),
}

#[derive(Clone, Debug, New)]
pub struct BaseType {
  pub base_type: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct CustomType {
  pub identifier: Token<TokenType>,
  pub type_parameters: Option<TypeParameters>,
}

impl CustomType {
  pub fn get_name(&self) -> String {
    self.identifier.lexeme.clone()
  }

  pub fn get_type_arguments(&self) -> Vec<Type> {
    match &self.type_parameters {
      Some(parameters) => parameters.type_arguments.iter().map(|x| x.argument_type.clone()).collect::<Vec<Type>>(),
      None => Vec::new()
    }
  }
}

#[derive(Clone, Debug, New)]
pub struct TupleType {
  pub left_paren_token: Token<TokenType>,
  pub tuple_arguments: Vec<TupleArgument>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct TupleArgument {
  pub optional_name: Option<Token<TokenType>>,
  pub optional_colon: Option<Token<TokenType>>,
  pub argument_type: Type,
  pub comma_token: Option<Token<TokenType>>
}

#[derive(Clone, Debug, New)]
pub struct TypeParameters {
  pub left_paren_token: Token<TokenType>,
  pub type_arguments: Vec<TypeArgument>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct TypeArgument {
  pub argument_type: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct SubType {
  pub left_paren_token: Token<TokenType>,
  pub sub_type: Box<Type>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct AutoType {
  pub auto_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct LazyType {
  pub lazy_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct RefType {
  pub ref_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct MutType {
  pub mut_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct ArrayType {
  pub base_type: Box<Type>,
  pub left_square: Token<TokenType>,
  pub index_type: Option<Box<Type>>,
  pub right_square: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct VariableType {
  pub base_type: Box<Type>,
  pub spread_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionType {
  pub function_token: Token<TokenType>,
  pub param_left_paren: Token<TokenType>,
  pub param_types: Vec<FunctionTypeArgument>,
  pub param_right_paren: Token<TokenType>,
  pub arrow_token: Option<Token<TokenType>>,
  pub result_left_paren: Option<Token<TokenType>>,
  pub result_types: Vec<FunctionTypeArgument>,
  pub result_right_paren: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionTypeArgument {
  pub arg_type: Type,
  pub comma_token: Option<Token<TokenType>>,
}