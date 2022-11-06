use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum HydroType {
  Signed8,
  Signed16,
  Signed32,
  Signed64,
  Unsigned8,
  Unsigned16,
  Unsigned32,
  Unsigned64,
  Float32,
  Float64,
  String,
  Bool,
  Void,
  Unknown
}

#[derive(Debug, Clone)]
pub enum HydroSymbol {
  Base(HydroType),
  Function(Vec<u64>, u64),
  Array(u64, u64),
  Ref(u64),
  Custom(u64),
}

#[derive(Debug, Clone)]
pub struct HydroSymbolTable {
  pub symbols: HashMap<u64, HydroSymbol>,
  pub variables: HashMap<String, u64>,
  pub functions: HashMap<(String, Vec<u64>), u64>,
  pub types: HashMap<String, u64>,
  pub parent_scope: Option<Box<HydroSymbolTable>>,
}

impl HydroSymbolTable {
  pub fn new(symbol_table: Option<Box<HydroSymbolTable>>) -> Self {
    Self {
      symbols: HashMap::new(),
      variables: HashMap::new(),
      functions: HashMap::new(),
      types: HashMap::new(),
      parent_scope: symbol_table
    }
  }

  pub fn get_next_symbol_id(&self) -> u64 {
    if self.symbols.len() == 0 {
      match &self.parent_scope {
        Some(pscope) => pscope.get_next_symbol_id(),
        None => 2,
      }
    } else {
      self.symbols.keys().max().unwrap() + 1
    }
  }

  pub fn add_symbol(&mut self, symbol: HydroSymbol) -> u64 {
    let next_id = self.get_next_symbol_id();
    self.symbols.insert(next_id, symbol);
    next_id
  }

  pub fn add_variable(&mut self, variable_name: String, symbol_id: u64) -> bool {
    if self.variables.contains_key(&variable_name) {
      return false;
    }
    self.variables.insert(variable_name, symbol_id);
    return true;
  }

  pub fn find_variable(&self, variable_name: String) -> Option<u64> {
    match self.variables.get(&variable_name) {
      Some(x) => Some(*x),
      None => None
    }
  }

  pub fn find_function(&self, func_signature: (String, Vec<u64>)) -> Option<u64> {
    match self.functions.get(&func_signature) {
      Some(x) => Some(*x),
      None => None
    }
  }

  pub fn is_boolean(&self, symbol_id: u64) -> bool {
    match self.symbols.get(&symbol_id) {
      Some(HydroSymbol::Base(HydroType::Bool)) => true,
      Some(HydroSymbol::Ref(x)) => self.is_boolean(*x),
      _ => false
    }
  }

}
