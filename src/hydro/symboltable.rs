use std::collections::HashMap;

use crate::hydro::instruction::Type;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
  Unknown,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum HydroSymbol {
  Base(HydroType),
  Function(Vec<u64>, u64),
  Array(u64, u64),
  Ref(u64),
  Custom(String),
}

#[derive(Debug, Clone)]
pub struct HydroSymbolTable {
  pub return_type_id: Option<u64>,
  pub id_to_symbol: HashMap<u64, HydroSymbol>,
  pub symbol_to_id: HashMap<HydroSymbol, u64>,
  pub variables: HashMap<String, u64>,
  pub functions: HashMap<(String, Vec<u64>), u64>,
  pub type_to_id: HashMap<String, u64>,
  pub id_to_member: HashMap<u64, HashMap<String, u64>>,
  pub parent_scope: Option<Box<HydroSymbolTable>>,
}

impl HydroSymbolTable {
  pub fn new(symbol_table: Option<Box<HydroSymbolTable>>) -> Self {
    Self {
      return_type_id: None,
      id_to_symbol: HashMap::new(),
      symbol_to_id: HashMap::new(),
      variables: HashMap::new(),
      functions: HashMap::new(),
      type_to_id: HashMap::new(),
      id_to_member: HashMap::new(),
      parent_scope: symbol_table,
    }
  }

  pub fn get_symbol_from_type(&self, typevar: Type) -> Option<HydroSymbol> {
    match typevar {
      Type::ArrayType(at) => {
        todo!()
      }
      Type::BaseType(bt) => match bt.token.lexeme.as_str() {
        "i8" => Some(HydroSymbol::Base(HydroType::Signed8)),
        "i16" => Some(HydroSymbol::Base(HydroType::Signed16)),
        "i32" => Some(HydroSymbol::Base(HydroType::Signed32)),
        "i64" => Some(HydroSymbol::Base(HydroType::Signed64)),
        "f32" => Some(HydroSymbol::Base(HydroType::Float32)),
        "f64" => Some(HydroSymbol::Base(HydroType::Float64)),
        "u8" => Some(HydroSymbol::Base(HydroType::Unsigned8)),
        "u16" => Some(HydroSymbol::Base(HydroType::Unsigned16)),
        "u32" => Some(HydroSymbol::Base(HydroType::Unsigned32)),
        "u64" => Some(HydroSymbol::Base(HydroType::Unsigned64)),
        "string" => Some(HydroSymbol::Base(HydroType::String)),
        "bool" => Some(HydroSymbol::Base(HydroType::Bool)),
        "void" => Some(HydroSymbol::Base(HydroType::Void)),
        custom_type_name => match self.get_type_id(custom_type_name.to_string()) {
          Some(type_id) => match self.id_to_symbol.get(&type_id) {
            Some(x) => Some(x.clone()),
            None => None,
          },
          None => None,
        },
      },
      Type::RefType(rt) => {
        todo!()
      }
    }
  }

  fn get_next_symbol_id(&self) -> u64 {
    if self.id_to_symbol.len() == 0 {
      match &self.parent_scope {
        Some(pscope) => pscope.get_next_symbol_id(),
        None => 2,
      }
    } else {
      self.id_to_symbol.keys().max().unwrap() + 1
    }
  }

  pub fn find_symbol(&self, symbol: HydroSymbol) -> Option<u64> {
    match self.symbol_to_id.get(&symbol) {
      Some(x) => Some(*x),
      None => match &self.parent_scope {
        Some(pscope) => pscope.find_symbol(symbol),
        None => None,
      },
    }
  }

  pub fn add_symbol(&mut self, symbol: HydroSymbol) -> u64 {
    match self.find_symbol(symbol.clone()) {
      Some(x) => x,
      None => {
        let next_id = self.get_next_symbol_id();
        self.id_to_symbol.insert(next_id, symbol.clone());
        self.symbol_to_id.insert(symbol, next_id);
        next_id
      }
    }
  }

  pub fn add_type(&mut self, typename: String, typemembers: HashMap<String, u64>) -> u64 {
    let id = self.add_symbol(HydroSymbol::Custom(typename.clone()));
    self.type_to_id.insert(typename, id);
    self.id_to_member.insert(id, typemembers);
    id
  }

  pub fn get_type_id(&self, typename: String) -> Option<u64> {
    match self.type_to_id.get(&typename) {
      Some(x) => Some(*x),
      None => match &self.parent_scope {
        Some(pscope) => pscope.get_type_id(typename),
        None => None,
      },
    }
  }

  pub fn add_function(&mut self, func_name: String, argtypes: Vec<u64>, return_type: u64) -> Option<()> {
    // TODO this may be wrong actually 
    match self.get_function_return_type_id(func_name.clone(), argtypes.clone()) {
      Some(x) => Some(()), // TODO send declaration info for error messages 
      None => {
        self.functions.insert((func_name, argtypes), return_type);
        None
      }
    }
  }

  pub fn get_function_return_type_id(&self, func_name: String, argtypes: Vec<u64>) -> Option<u64> {
    match self.functions.get(&(func_name.clone(), argtypes.clone())) {
      Some(x) => Some(*x),
      None => match &self.parent_scope {
        Some(pscope) => pscope.get_function_return_type_id(func_name, argtypes),
        None => None,
      },
    }
  }

  pub fn get_variable_type_id(&self, var_name: String) -> Option<u64> {
    match self.variables.get(&var_name) {
      Some(x) => Some(*x),
      None => match &self.parent_scope {
        Some(pscope) => pscope.get_variable_type_id(var_name),
        None => None,
      },
    }
  }

  pub fn add_variable(&mut self, var_name: String, type_id: u64) {
    self.variables.insert(var_name, type_id);
  }

  pub fn matches_bool(&self, type_id: u64) -> bool {
    match self.id_to_symbol.get(&type_id) {
      Some(HydroSymbol::Base(HydroType::Bool)) => true,
      Some(HydroSymbol::Ref(x)) => self.matches_bool(*x),
      _ => false,
    }
  }
}
