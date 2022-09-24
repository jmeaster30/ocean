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
  Cache(i32),
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

pub fn get_greater_type(a: &OceanType, b: &OceanType) -> Option<OceanType> {
  if is_type_subset(a, b) {
    Some(b.clone())
  } else if is_type_subset(b, a) {
    Some(a.clone())
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
  pub type_id: i32,
  pub span: (usize, usize),
}

impl SymbolTableVarEntry {
  pub fn new(type_id: i32, span: (usize, usize)) -> Self {
    Self { type_id, span }
  }
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  is_soft_scope: bool,
  symbols: HashMap<i32, Symbol>,
  types: HashMap<String, i32>,
  variables: HashMap<String, Vec<SymbolTableVarEntry>>,
  parent_scope: Option<Box<SymbolTable>>,
}

impl SymbolTable {
  pub fn soft_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: true,
      symbols: HashMap::new(),
      types: HashMap::new(),
      variables: HashMap::new(),
      parent_scope,
    }
  }

  pub fn hard_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: false,
      symbols: HashMap::new(),
      types: HashMap::new(),
      variables: HashMap::new(),
      parent_scope,
    }
  }

  pub fn add_var(&mut self, name: String, span: (usize, usize), type_id: i32) {
    let found = self.variables.get(&name);
    if let Some(current_value) = found {
      // TODO maybe create a warning here if we try to add a variable that exists in a higher scope
      let mut new_value = Vec::new();
      for a in current_value {
        new_value.push(a.clone());
      }
      new_value.push(SymbolTableVarEntry::new(type_id, span));
      self.variables.remove_entry(&name);
      self.variables.insert(name, new_value);
    } else {
      self
        .variables
        .insert(name, vec![SymbolTableVarEntry::new(type_id, span)]);
    }
  }

  pub fn find_variable(&self, name: &String) -> Option<&SymbolTableVarEntry> {
    None
  }

  pub fn add_symbol(&mut self, sym: Symbol) -> i32 {
    let index = self.get_new_symbol_id();
    self.symbols.insert(index, sym);
    index
  }

  pub fn get_new_symbol_id(&self) -> i32 {
    match (self.symbols.len(), self.parent_scope.as_ref()) {
      (0, None) => 0,
      (0, Some(p_scope)) => p_scope.get_new_symbol_id(),
      _ => *self.symbols.keys().max().unwrap()
    }
  }

  pub fn match_types(&mut self, a: i32, b: i32) -> Option<i32> {
    let resolved_a = self.symbols.get(&a).cloned();
    let resolved_b = self.symbols.get(&b).cloned();
    match (resolved_a, resolved_b) {
      (Some(sym_a), Some(Symbol::Unknown)) => {
        self.update_symbol(b, sym_a.clone());
        Some(a)
      }
      (Some(Symbol::Unknown), Some(sym_b)) => {
        self.update_symbol(a, sym_b.clone());
        Some(b)
      }
      (Some(Symbol::Base(type_a)), Some(Symbol::Base(type_b))) => {
        let result = get_greater_type(&type_a, &type_b);
        if result == Some(type_a.clone()) {
          Some(a)
        } else if result == Some(type_b.clone()) {
          Some(b)
        } else {
          None
        }
      }
      _ => {
        None
      }
    }
  }

  fn update_symbol(&mut self, type_id: i32, sym: Symbol) {
    if self.symbols.contains_key(&type_id) {
      self.symbols.remove(&type_id);
      self.symbols.insert(type_id, sym);
    } else if self.parent_scope.is_some() {
      self.parent_scope.as_mut().unwrap().update_symbol(type_id, sym);
    } else {
      //do nothing i guess
    }
  }
}

