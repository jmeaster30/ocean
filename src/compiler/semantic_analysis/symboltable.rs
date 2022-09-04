use std::collections::HashMap;

use super::operators::*;
use crate::compiler::parser::ast::Type;

#[allow(unused_variables)]
#[derive(Clone, Debug)]
pub enum Symbol {
  Function(FunctionSymbol),
  Auto(AutoSymbol),
  Custom(CustomSymbol),
  Tuple(TupleSymbol),
  Modified(ModifiedSymbol),
  Array(ArraySymbol),
  Base(OceanType),
  Unknown,
}

#[derive(Clone, PartialEq, Debug)]
pub enum OceanType {
  Void,
  Char,
  String,
  Bool,
  Float(u8),
  Unsigned(u8),
  Signed(u8),
}

pub fn get_superset(a: &OceanType) -> Vec<OceanType> {
  match a {
    OceanType::Void => vec![],
    OceanType::Char => vec![OceanType::String],
    OceanType::String => vec![],
    OceanType::Bool => vec![],
    OceanType::Float(64) => vec![],
    OceanType::Float(x) => vec![OceanType::Float(*x * 2)],
    OceanType::Unsigned(8) => vec![OceanType::Signed(16), OceanType::Unsigned(16)],
    OceanType::Unsigned(64) => vec![],
    OceanType::Unsigned(x) => vec![
      OceanType::Unsigned(*x * 2),
      OceanType::Signed(*x * 2),
      OceanType::Float(*x * 2),
    ],
    OceanType::Signed(8) => vec![OceanType::Signed(16)],
    OceanType::Signed(64) => vec![],
    OceanType::Signed(x) => vec![OceanType::Signed(*x * 2), OceanType::Float(*x * 2)],
  }
}

pub fn get_index_type(a: &Symbol) -> Option<Symbol> {
  match a {
    Symbol::Base(ocean_type) if *ocean_type == OceanType::String => {
      Some(Symbol::Base(OceanType::Unsigned(64)))
    }
    Symbol::Array(array_type) => Some(array_type.index.as_ref().clone()),
    _ => None,
  }
}

pub fn get_iterator_type(a: &Symbol) -> Option<Symbol> {
  match a {
    Symbol::Base(ocean_type) if *ocean_type == OceanType::String => {
      Some(Symbol::Base(OceanType::Char))
    }
    Symbol::Array(array_type) => Some(array_type.storage.as_ref().clone()),
    _ => None,
  }
}

pub fn build_full_superset(a: &OceanType, current_results: &Vec<OceanType>) -> Vec<OceanType> {
  let superset = get_superset(a);
  let filtered_set = superset
    .into_iter()
    .filter(|i| i != a && !current_results.contains(i))
    .collect::<Vec<_>>();

  let mut result = filtered_set.to_vec();
  result.append(&mut current_results.to_vec());

  if filtered_set.is_empty() {
    return result;
  }

  result.push(a.clone());

  for set in filtered_set {
    let additions = build_full_superset(&set, &result);
    for add_set in additions {
      if !result.contains(&add_set) {
        result.push(add_set);
      }
    }
  }

  return result;
}

pub fn is_type_subset(a: &OceanType, b: &OceanType) -> bool {
  if a == b {
    return true;
  }
  let direct_superset = build_full_superset(a, &vec![]);
  return direct_superset.len() != 0 && direct_superset.contains(b);
}

pub fn is_compat_type(a: &OceanType, b: &OceanType) -> bool {
  return is_type_subset(a, b) || is_type_subset(b, a);
}

pub fn get_greater_type(a: &OceanType, b: &OceanType) -> Option<Symbol> {
  if is_type_subset(a, b) {
    Some(Symbol::Base(b.clone()))
  } else if is_type_subset(b, a) {
    Some(Symbol::Base(a.clone()))
  } else {
    None
  }
}

#[derive(Clone, Debug)]
pub struct ArraySymbol {
  pub storage: Box<Symbol>,
  pub index: Box<Symbol>,
}

impl ArraySymbol {
  pub fn new(storage: Box<Symbol>, index: Box<Symbol>) -> Self {
    Self { storage, index }
  }
}

#[derive(Clone, Debug)]
pub struct AutoSymbol {
  pub name: String,
  pub constraints: Option<Vec<Symbol>>, // Some(Vec::new) <- any.... None <- none
  pub members: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct FunctionSymbol {
  pub parameters: Vec<(String, Symbol)>,
  pub returns: Vec<(String, Symbol)>,
}

#[derive(Clone, Debug)]
pub struct ModifiedSymbol {
  pub reference: bool,
  pub mutable: bool,
  pub comp: bool,
  pub base_type: Box<Symbol>,
}

#[derive(Clone, Debug)]
pub struct CustomSymbol {
  pub name: String,
  pub members: HashMap<String, Symbol>,
}

#[derive(Clone, Debug)]
pub struct TupleSymbol {
  pub members: Vec<(String, Symbol)>,
}

impl TupleSymbol {
  pub fn new() -> Self {
    Self {
      members: Vec::new(),
    }
  }

  pub fn add_named(&mut self, name: String, symbol: Symbol) {
    self.members.push((name, symbol));
  }

  pub fn add_unnamed(&mut self, symbol: Symbol) {
    self.members.push((self.members.len().to_string(), symbol));
  }
}

#[derive(Clone, Debug)]
pub struct SymbolTableVarEntry {
  pub symbol: Symbol,
  pub span: (usize, usize),
}

impl SymbolTableVarEntry {
  pub fn new(symbol: Symbol, span: (usize, usize)) -> Self {
    Self { symbol, span }
  }
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  is_soft_scope: bool,
  types: HashMap<String, Symbol>,
  variables: HashMap<String, Vec<SymbolTableVarEntry>>,
  parent_scope: Option<Box<SymbolTable>>,
}

impl SymbolTable {
  pub fn soft_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: true,
      types: HashMap::new(),
      variables: HashMap::new(),
      parent_scope,
    }
  }

  pub fn hard_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: false,
      types: HashMap::new(),
      variables: HashMap::new(),
      parent_scope,
    }
  }

  pub fn add_var(&mut self, name: String, span: (usize, usize), symbol: Symbol) {
    let found = self.variables.get(&name);
    if let Some(current_value) = found {
      // TODO maybe create a warning here if we try to add a variable that exists in a higher scope
      let mut new_value = Vec::new();
      for a in current_value {
        new_value.push(a.clone());
      }
      new_value.push(SymbolTableVarEntry::new(symbol, span));
      self.variables.remove_entry(&name);
      self.variables.insert(name, new_value);
    } else {
      self
        .variables
        .insert(name, vec![SymbolTableVarEntry::new(symbol, span)]);
    }
  }

  pub fn find_variable(&self, name: &String) -> Option<&SymbolTableVarEntry> {
    let found_variable_list = self.variables.get(name);
    let mut final_result = None;
    match found_variable_list {
      Some(variable_list) => {
        let mut result = None;
        for variable in variable_list {
          match variable.symbol {
            Symbol::Function(_) => {}
            _ => result = Some(variable),
          }
        }
        match result {
          Some(res) => final_result = result,
          None => {
            if self.is_soft_scope && self.parent_scope.is_some() {
              final_result = self.parent_scope.as_ref().unwrap().find_variable(name);
            } else {
              final_result = None
            }
          }
        }
      }
      None => {
        if self.is_soft_scope && self.parent_scope.is_some() {
          final_result = self.parent_scope.as_ref().unwrap().find_variable(name);
        } else {
          final_result = None
        }
      }
    }
    final_result
  }

  pub fn resolve_type_ast(&self, type_ast: &Type) -> Symbol {
    Symbol::Unknown
  }

  pub fn match_types(&self, a: &Symbol, b: &Symbol) -> Option<Symbol> {
    match (a, b) {
      (Symbol::Unknown, x) => {
        eprintln!("TODO: UPDATE UNKNOWN VARIABLE");
        Some(x.clone())
      }
      (x, Symbol::Unknown) => {
        eprintln!("TODO: UPDATE UNKNOWN VARIABLE");
        Some(x.clone())
      }
      (Symbol::Base(x), Symbol::Base(y)) => {
        if is_type_subset(x, y) {
          Some(Symbol::Base(y.clone()))
        } else if is_type_subset(y, x) {
          Some(Symbol::Base(x.clone()))
        } else {
          None
        }
      }
      (Symbol::Array(x), Symbol::Array(y)) => {
        let storage_match = self.match_types(x.storage.as_ref(), y.storage.as_ref());
        let index_match = self.match_types(x.index.as_ref(), y.index.as_ref());
        match (storage_match, index_match) {
          (Some(storage), Some(index)) => Some(Symbol::Array(ArraySymbol::new(
            Box::new(storage),
            Box::new(index),
          ))),
          _ => None,
        }
      }
      _ => {
        eprintln!("ERROR UNHANDLED TYPE MATCH CASE!!!!!!!!");
        None
      }
    }
  }
}

pub fn resolve_postfix_operator(
  operator: String,
  expr_symbol: &Symbol,
  symbol_table: &SymbolTable,
) -> Option<Symbol> {
  let option = get_postfix_operator_type(operator, expr_symbol);
  option
}

pub fn resolve_prefix_operator(
  operator: String,
  expr_symbol: &Symbol,
  symbol_table: &SymbolTable,
) -> Option<Symbol> {
  let option = get_prefix_operator_type(operator, expr_symbol);
  option
}

pub fn resolve_binary_operator(
  operator: String,
  lhs_symbol: &Symbol,
  rhs_symbol: &Symbol,
  symbol_table: &SymbolTable,
) -> Option<Symbol> {
  let option = get_infix_operator_type(operator, lhs_symbol, rhs_symbol);
  option
}
