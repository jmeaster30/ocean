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

fn is_sub_type(a: HydroType, b: HydroType) -> bool {
  match a {
    HydroType::Signed8 => match b {
      HydroType::Signed8
      | HydroType::Signed16
      | HydroType::Signed32
      | HydroType::Signed64
      | HydroType::Float32
      | HydroType::Float64 => true,
      _ => false,
    },
    HydroType::Signed16 => match b {
      HydroType::Signed16
      | HydroType::Signed32
      | HydroType::Signed64
      | HydroType::Float32
      | HydroType::Float64 => true,
      _ => false,
    },
    HydroType::Signed32 => match b {
      HydroType::Signed32 | HydroType::Signed64 | HydroType::Float64 => true,
      _ => false,
    },
    HydroType::Signed64 => match b {
      HydroType::Signed64 => true,
      _ => false,
    },
    HydroType::Unsigned8 => match b {
      HydroType::Unsigned8
      | HydroType::Unsigned16
      | HydroType::Unsigned32
      | HydroType::Unsigned64
      | HydroType::Signed16
      | HydroType::Signed32
      | HydroType::Signed64
      | HydroType::Float32
      | HydroType::Float64 => true,
      _ => false,
    },
    HydroType::Unsigned16 => match b {
      HydroType::Unsigned16
      | HydroType::Unsigned32
      | HydroType::Unsigned64
      | HydroType::Signed32
      | HydroType::Signed64
      | HydroType::Float32
      | HydroType::Float64 => true,
      _ => false,
    },
    HydroType::Unsigned32 => match b {
      HydroType::Unsigned32 | HydroType::Unsigned64 | HydroType::Signed64 | HydroType::Float64 => {
        true
      }
      _ => false,
    },
    HydroType::Unsigned64 => match b {
      HydroType::Unsigned64 => true,
      _ => false,
    },
    HydroType::String => match b {
      HydroType::String => true,
      _ => false,
    },
    HydroType::Bool => match b {
      HydroType::Bool => true,
      _ => false,
    },
    HydroType::Void => match b {
      HydroType::Void => true,
      _ => false,
    },
    _ => false,
  }
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
  pub return_count: Vec<i32>,
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
      return_count: Vec::new(),
      id_to_symbol: HashMap::new(),
      symbol_to_id: HashMap::new(),
      variables: HashMap::new(),
      functions: HashMap::new(),
      type_to_id: HashMap::new(),
      id_to_member: HashMap::new(),
      parent_scope: symbol_table,
    }
  }

  pub fn get_symbol_from_type(&mut self, typevar: Type) -> Option<HydroSymbol> {
    match typevar.clone() {
      Type::ArrayType(at) => {
        let base_type = self.get_symbol_from_type(*at.base_type);
        let index_type = self.get_symbol_from_type(*at.index_type);
        match (base_type, index_type) {
          (Some(btsym), Some(itsym)) => {
            let bt_id = self.add_symbol(btsym);
            let it_id = self.add_symbol(itsym);
            Some(HydroSymbol::Array(bt_id, it_id))
          }
          _ => match &mut self.parent_scope {
            Some(pscope) => pscope.get_symbol_from_type(typevar),
            None => None,
          },
        }
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
            None => match &mut self.parent_scope {
              Some(pscope) => pscope.get_symbol_from_type(typevar),
              None => None,
            },
          },
          None => None,
        },
      },
      Type::RefType(rt) => match self.get_symbol_from_type(*rt.base_type) {
        Some(sym) => {
          let bt_id = self.add_symbol(sym);
          Some(HydroSymbol::Ref(bt_id))
        }
        None => match &mut self.parent_scope {
          Some(pscope) => pscope.get_symbol_from_type(typevar),
          None => None,
        },
      },
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

  pub fn get_symbol_by_id(&self, type_id: u64) -> Option<HydroSymbol> {
    match self.id_to_symbol.get(&type_id) {
      Some(x) => Some(x.clone()),
      None => match &self.parent_scope {
        Some(pscope) => pscope.get_symbol_by_id(type_id),
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

  // I don't know if I like this
  pub fn clean_type_name(&self, name: String) -> String {
    let mut result = name;
    if result.starts_with("@") {
      result.remove(0);
    }
    result
  }

  pub fn add_type(&mut self, typename: String, typemembers: HashMap<String, u64>) -> u64 {
    let fixed = self.clean_type_name(typename);
    let id = self.add_symbol(HydroSymbol::Custom(fixed.clone()));
    self.type_to_id.insert(fixed, id);
    self.id_to_member.insert(id, typemembers);
    id
  }

  pub fn get_member_type_id(&mut self, typeid: u64, membername: String) -> Option<u64> {
    match self.id_to_member.get(&typeid) {
      Some(members) => match members.get(&self.clean_type_name(membername)) {
        Some(result_id) => Some(*result_id),
        None => None,
      },
      None => match &mut self.parent_scope {
        Some(pscope) => pscope.get_member_type_id(typeid, membername),
        None => None,
      },
    }
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

  pub fn add_function(
    &mut self,
    func_name: String,
    argtypes: Vec<u64>,
    return_type: u64,
  ) -> Option<()> {
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

  pub fn set_variable(&mut self, var_name: String, type_id: u64) {
    self.variables.insert(var_name, type_id);
  }

  pub fn matches_bool(&self, type_id: u64) -> bool {
    match self.find_symbol(HydroSymbol::Base(HydroType::Bool)) {
      Some(bool_id) => match self.matches_type(bool_id, type_id) {
        Some(_) => true,
        None => false,
      },
      // if we've never seen a bool before we know type_id is not a bool
      None => false,
    }
  }

  pub fn is_indexable_by_type(&self, type_id: u64, actual_index_id: u64) -> Option<u64> {
    match self.get_symbol_by_id(type_id) {
      Some(HydroSymbol::Array(base_id, expected_index_id)) => {
        match self.matches_type(actual_index_id, expected_index_id) {
          Some(_) => Some(base_id),
          None => None,
        }
      }
      _ => None,
    }
  }

  pub fn matches_type(&self, a: u64, b: u64) -> Option<u64> {
    match self.get_symbol_by_id(a) {
      Some(x) => match self.get_symbol_by_id(b) {
        Some(y) => match self.match_symbol(x, y) {
          Some(value) => Some(if value { b } else { a }),
          None => None,
        },
        None => panic!("Expected to have type"),
      },
      None => panic!("Expected to have type"),
    }
  }

  fn match_symbol(&self, sub: HydroSymbol, sup: HydroSymbol) -> Option<bool> {
    match (sub.clone(), sup.clone()) {
      (HydroSymbol::Base(HydroType::Unknown), _) => None,
      (_, HydroSymbol::Base(HydroType::Unknown)) => None,
      (_, HydroSymbol::Ref(id)) => {
        let sym = self.get_symbol_by_id(id);
        match sym {
          Some(sup_sym) => self.match_symbol(sub, sup_sym),
          None => None,
        }
      }
      (HydroSymbol::Ref(id), _) => {
        let sym = self.get_symbol_by_id(id);
        match sym {
          Some(sub_sym) => self.match_symbol(sub_sym, sup),
          None => None,
        }
      }
      (HydroSymbol::Custom(sub_name), HydroSymbol::Custom(sup_name)) => {
        if sub_name == sup_name {
          Some(true)
        } else {
          None
        }
      }
      (HydroSymbol::Base(sub_type), HydroSymbol::Base(sup_type)) => {
        if is_sub_type(sub_type.clone(), sup_type.clone()) {
          Some(true)
        } else if is_sub_type(sup_type, sub_type) {
          Some(false)
        } else {
          None
        }
      }
      _ => {
        println!("{:?}", sub);
        println!("{:?}", sup);
        None
      }
    }
  }

  pub fn get_return_symbol(&self) -> Option<HydroSymbol> {
    match self.return_type_id {
      Some(x) => self.get_symbol_by_id(x),
      None => None,
    }
  }

  pub fn start_return_branch(&mut self) {
    self.return_count.push(0);
  }

  pub fn found_return(&mut self, type_id: u64) -> Option<bool> {
    match self.return_type_id {
      Some(x) => {
        let return_count_len = self.return_count.len();
        if return_count_len == 0 {
          return None;
        }

        self.return_count[return_count_len - 1] += 1;
        match self.matches_type(type_id, x) {
          Some(val) => Some(x == val),
          None => Some(false),
        }
      }
      None => None,
    }
  }

  pub fn check_return_branch(&mut self) -> Option<bool> {
    match self.return_count.pop() {
      Some(x) => Some(x >= 0),
      None => None,
    }
  }

  pub fn require_return(&mut self, both_branches_return: bool) {
    if both_branches_return {
      return;
    }
    let return_count_len = self.return_count.len();
    if return_count_len == 0 {
      return;
    }
    self.return_count[return_count_len - 1] -= 1;
  }
}
