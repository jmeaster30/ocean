use std::collections::HashMap;

use crate::compiler::parser::ast::Type;

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
  F32,
  F64,
  U8,
  U16,
  U32,
  U64,
  I8,
  I16,
  I32,
  I64,
}

pub fn get_superset(a: &OceanType) -> Vec<OceanType> {
  match a {
    OceanType::Void => vec![],
    OceanType::Char => vec![OceanType::String, OceanType::U8],
    OceanType::String => vec![],
    OceanType::Bool => vec![],
    OceanType::F32 => vec![OceanType::F64],
    OceanType::F64 => vec![],
    OceanType::U8 => vec![OceanType::Char, OceanType::I16, OceanType::U16],
    OceanType::U16 => vec![OceanType::I32, OceanType::U32, OceanType::F32],
    OceanType::U32 => vec![OceanType::I64, OceanType::U64, OceanType::F64],
    OceanType::U64 => vec![],
    OceanType::I8 => vec![OceanType::I16],
    OceanType::I16 => vec![OceanType::I32, OceanType::F32],
    OceanType::I32 => vec![OceanType::I64, OceanType::F64],
    OceanType::I64 => vec![],
  }
}

pub fn is_type_subset(a: &OceanType, b: &OceanType) -> bool {
  if a == b {
    return true;
  }
  let direct_superset = get_superset(a);
  if direct_superset.len() == 0 {
    return false;
  }
  let mut result = false;
  for set in &direct_superset {
    if result == false && is_type_subset(set, b) {
      result = true;
    }
  }
  return result;
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
  pub members: HashMap<String, Symbol>,
}

impl TupleSymbol {
  pub fn new() -> Self {
    Self {
      members: HashMap::new(),
    }
  }

  pub fn add_named(&mut self, name: String, symbol: Symbol) {
    self.members.insert(name, symbol);
  }

  pub fn add_unnamed(&mut self, symbol: Symbol) {
    self.members.insert(self.members.len().to_string(), symbol);
  }
}

pub struct SymbolTableVarEntry {
  pub symbol: Symbol,
  pub span: (usize, usize),
}

impl SymbolTableVarEntry {
  pub fn new(symbol: Symbol, span: (usize, usize)) -> Self {
    Self { symbol, span }
  }
}

pub struct SymbolTable {
  pub types: HashMap<String, Symbol>,
  pub variables: HashMap<String, Vec<SymbolTableVarEntry>>,
  pub parent_scope: Option<Box<SymbolTable>>,
}

impl SymbolTable {
  pub fn new() -> Self {
    Self {
      types: HashMap::new(),
      variables: HashMap::new(),
      parent_scope: None,
    }
  }

  pub fn find_variable(&self, name: &String) -> Option<&SymbolTableVarEntry> {
    let found_variable_list = self.variables.get(name);
    match found_variable_list {
      Some(variable_list) => {
        let mut result = None;
        for variable in variable_list {
          match variable.symbol {
            Symbol::Function(_) => {}
            _ => result = Some(variable),
          }
        }
        result
      }
      None => None,
    }
  }

  pub fn resolve_type_ast(&self, type_ast: &Type) -> Symbol {
    Symbol::Unknown
  }

  pub fn match_types(&self, a: &Symbol, b: &Symbol) -> Option<Symbol> {
    Some(Symbol::Unknown)
  }
}
