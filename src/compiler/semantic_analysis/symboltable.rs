use std::collections::HashMap;

use crate::compiler::parser::ast::Type;

#[allow(unused_variables)]
#[derive(Clone, Debug)]
pub enum Symbol {
  Function(FunctionSymbol),
  Auto(AutoSymbol),
  Custom(CustomSymbol),
  Tuple(TupleSymbol),
  Assignable(AssignableSymbol),
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

pub fn get_base_type_symbol_from_lexeme(lexeme: &String) -> Symbol {
  match lexeme.as_str() {
    "i8" => Symbol::Base(OceanType::Signed(8)),
    "i16" => Symbol::Base(OceanType::Signed(16)),
    "i32" => Symbol::Base(OceanType::Signed(32)),
    "i64" => Symbol::Base(OceanType::Signed(64)),
    "u8" => Symbol::Base(OceanType::Unsigned(8)),
    "u16" => Symbol::Base(OceanType::Unsigned(16)),
    "u32" => Symbol::Base(OceanType::Unsigned(32)),
    "u64" => Symbol::Base(OceanType::Unsigned(64)),
    "f32" => Symbol::Base(OceanType::Float(32)),
    "f64" => Symbol::Base(OceanType::Float(64)),
    "string" => Symbol::Base(OceanType::String),
    "char" => Symbol::Base(OceanType::Char),
    "bool" => Symbol::Base(OceanType::Bool),
    "void" => Symbol::Base(OceanType::Void),
    _ => panic!("Unexpected type lexeme '{}'", lexeme),
  }
}

pub fn get_superset(a: &OceanType) -> Vec<OceanType> {
  match a {
    OceanType::Void => vec![],
    OceanType::Char => vec![OceanType::Unsigned(8), OceanType::String],
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

fn build_full_superset(a: &OceanType, current_results: &Vec<OceanType>) -> Vec<OceanType> {
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

fn is_type_subset(a: &OceanType, b: &OceanType) -> bool {
  if a == b {
    return true;
  }
  let direct_superset = build_full_superset(a, &vec![]);
  return direct_superset.len() != 0 && direct_superset.contains(b);
}

fn is_compat_type(a: &OceanType, b: &OceanType) -> bool {
  return is_type_subset(a, b) || is_type_subset(b, a);
}

fn get_greater_type(a: &OceanType, b: &OceanType) -> Option<OceanType> {
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
  pub storage: i32,
  pub index: i32,
}

impl ArraySymbol {
  pub fn new(storage: i32, index: i32) -> Self {
    Self { storage, index }
  }
}

#[derive(Clone, Debug)]
pub struct AutoSymbol {
  pub name: String,
  //pub constraints: Option<Vec<Symbol>>, // Some(Vec::new) <- any.... None <- none
  pub members: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct FunctionSymbol {
  pub parameters: Vec<(String, i32)>,
  pub returns: Vec<(String, i32)>,
}

impl FunctionSymbol {
  pub fn new() -> Self {
    Self {
      parameters: Vec::new(),
      returns: Vec::new(),
    }
  }

  pub fn add_parameter(&mut self, name: String, symbol: i32) {
    self.parameters.push((name, symbol));
  }

  pub fn add_return(&mut self, name: String, symbol: i32) {
    self.returns.push((name, symbol));
  }
}

#[derive(Clone, Debug)]
pub struct ModifiedSymbol {
  pub reference: bool,
  pub mutable: bool,
  pub comp: bool,
  pub base_type: i32,
}

#[derive(Clone, Debug)]
pub struct AssignableSymbol {
  pub base_type: i32,
}

impl AssignableSymbol {
  pub fn new(base_type: i32) -> Self {
    Self { base_type }
  }
}

#[derive(Clone, Debug)]
pub struct CustomSymbol {
  pub name: String,
  pub members: HashMap<String, i32>,
}

impl CustomSymbol {
  pub fn new(name: String) -> Self {
    Self {
      name,
      members: HashMap::new(),
    }
  }

  pub fn add_member(&mut self, name: String, type_id: i32) {
    self.members.insert(name, type_id);
  }
}

#[derive(Clone, Debug)]
pub struct TupleSymbol {
  pub members: Vec<(String, i32)>,
}

impl TupleSymbol {
  pub fn new() -> Self {
    Self {
      members: Vec::new(),
    }
  }

  pub fn add_named(&mut self, name: String, symbol: i32) {
    self.members.push((name, symbol));
  }

  pub fn add_unnamed(&mut self, symbol: i32) {
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

pub fn get_base_type_id(base_type: Symbol) -> i32 {
  match base_type {
    Symbol::Base(OceanType::Bool) => 0,
    Symbol::Base(OceanType::Char) => 1,
    Symbol::Base(OceanType::String) => 2,
    Symbol::Base(OceanType::Void) => 3,
    Symbol::Unknown => 4,
    Symbol::Base(OceanType::Signed(x)) => x as i32,
    Symbol::Base(OceanType::Unsigned(x)) => (x + 1) as i32,
    Symbol::Base(OceanType::Float(x)) => (x + 2) as i32,
    _ => panic!(),
  }
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  is_soft_scope: bool,
  symbols: HashMap<i32, Symbol>,
  types: HashMap<String, i32>, // TODO I want to store the location the type was defined
  variables: HashMap<String, Vec<SymbolTableVarEntry>>,
  casts: Vec<(i32, i32)>,
  parent_scope: Option<Box<SymbolTable>>,
}

impl SymbolTable {
  pub fn init() -> Self {
    let mut base_symbols = HashMap::new();
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Bool)),
      Symbol::Base(OceanType::Bool),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Char)),
      Symbol::Base(OceanType::Char),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::String)),
      Symbol::Base(OceanType::String),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Void)),
      Symbol::Base(OceanType::Void),
    );
    base_symbols.insert(get_base_type_id(Symbol::Unknown), Symbol::Unknown);
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Signed(8))),
      Symbol::Base(OceanType::Signed(8)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Signed(16))),
      Symbol::Base(OceanType::Signed(16)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Signed(32))),
      Symbol::Base(OceanType::Signed(32)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Signed(64))),
      Symbol::Base(OceanType::Signed(64)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Unsigned(8))),
      Symbol::Base(OceanType::Unsigned(8)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Unsigned(16))),
      Symbol::Base(OceanType::Unsigned(16)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Unsigned(32))),
      Symbol::Base(OceanType::Unsigned(32)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
      Symbol::Base(OceanType::Unsigned(64)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Float(32))),
      Symbol::Base(OceanType::Float(32)),
    );
    base_symbols.insert(
      get_base_type_id(Symbol::Base(OceanType::Float(64))),
      Symbol::Base(OceanType::Float(64)),
    );
    Self {
      is_soft_scope: false,
      symbols: base_symbols,
      types: HashMap::new(),
      variables: HashMap::new(),
      casts: Vec::new(),
      parent_scope: None,
    }
  }

  pub fn soft_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: true,
      symbols: HashMap::new(),
      types: HashMap::new(),
      variables: HashMap::new(),
      casts: Vec::new(),
      parent_scope,
    }
  }

  pub fn hard_scope(parent_scope: Option<Box<SymbolTable>>) -> Self {
    Self {
      is_soft_scope: false,
      symbols: HashMap::new(),
      types: HashMap::new(),
      variables: HashMap::new(),
      casts: Vec::new(),
      parent_scope,
    }
  }

  pub fn find_type(&self, name: &String) -> Option<i32> {
    match self.types.get(name) {
      Some(x) => Some(x.clone()),
      None => match &self.parent_scope {
        Some(p_scope) => p_scope.find_type(name),
        None => None,
      },
    }
  }

  pub fn add_type(&mut self, name: String, type_id: i32) {
    self.types.insert(name, type_id);
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
    match self.variables.get(name) {
      Some(variable_list) => {
        if variable_list.is_empty() {
          None // we shouldn't get here I think
        } else {
          Some(&variable_list[0]) // improve this...
        }
      }
      None => match &self.parent_scope {
        Some(p_scope) if self.is_soft_scope => p_scope.find_variable(name),
        _ => None,
      },
    }
  }

  pub fn find_variable_in_scope(&self, name: &String) -> Option<&SymbolTableVarEntry> {
    match self.variables.get(name) {
      Some(variable_list) => {
        if variable_list.is_empty() {
          None // we shouldn't get here I think
        } else {
          Some(&variable_list[0]) // improve this...
        }
      }
      None => None,
    }
  }

  pub fn add_symbol(&mut self, sym: Symbol) -> i32 {
    let index = self.get_new_symbol_id();
    self.symbols.insert(index, sym);
    index
  }

  pub fn get_symbol(&self, index: i32) -> Option<Symbol> {
    match self.symbols.get(&index) {
      Some(x) => Some(x.clone()),
      None => match &self.parent_scope {
        Some(p_scope) => p_scope.get_symbol(index),
        None => None,
      },
    }
  }

  pub fn get_new_symbol_id(&self) -> i32 {
    match (self.symbols.len(), self.parent_scope.as_ref()) {
      (0, None) => 0,
      (0, Some(p_scope)) => p_scope.get_new_symbol_id(),
      _ => *self.symbols.keys().max().unwrap() + 1,
    }
  }

  pub fn match_types(&mut self, a: i32, b: i32) -> Option<i32> {
    // Check if the ids are exactly the same
    if a == b {
      return Some(a);
    }

    let resolved_a = self.get_symbol(a);
    let resolved_b = self.get_symbol(b);
    match (resolved_a, resolved_b) {
      (Some(sym_a), Some(Symbol::Unknown)) => {
        self.update_symbol(b, sym_a.clone());
        Some(b)
      }
      (Some(Symbol::Unknown), Some(sym_b)) => {
        self.update_symbol(a, sym_b.clone());
        Some(b)
      }
      (Some(Symbol::Assignable(assignable_symbol)), Some(_)) => {
        self.match_types(assignable_symbol.base_type, b)
      }
      (Some(_), Some(Symbol::Assignable(assignable_symbol))) => {
        self.match_types(a, assignable_symbol.base_type)
      }
      (Some(Symbol::Array(type_a)), Some(Symbol::Array(type_b))) => {
        // TODO I think this needs to be done a different way perhaps
        let matched_storage_id = self.match_types(type_a.storage, type_b.storage);
        let matched_index_id = self.match_types(type_a.index, type_b.index);
        match (matched_storage_id, matched_index_id) {
          (Some(s_id), Some(i_id)) => {
            let result_id = self.add_symbol(Symbol::Array(ArraySymbol::new(s_id, i_id)));
            Some(result_id)
          }
          _ => None,
        }
      }
      (Some(Symbol::Base(type_a)), Some(Symbol::Base(type_b))) => {
        let result = get_greater_type(&type_a, &type_b);
        match result {
          Some(result_type) => Some(get_base_type_id(Symbol::Base(result_type))),
          _ => None,
        }
      }
      _ => None,
    }
  }

  fn update_symbol(&mut self, type_id: i32, sym: Symbol) {
    if self.symbols.contains_key(&type_id) {
      self.symbols.remove(&type_id);
      self.symbols.insert(type_id, sym);
    } else if self.parent_scope.is_some() {
      self
        .parent_scope
        .as_mut()
        .unwrap()
        .update_symbol(type_id, sym);
    } else {
      //do nothing i guess
    }
  }

  fn get_resolved_symbol(&self, target_type_id: i32) -> Option<Symbol> {
    let resolved_symbol = self.symbols.get(&target_type_id);
    match resolved_symbol {
      Some(Symbol::Cache(cache_type_id)) => self.get_resolved_symbol(*cache_type_id),
      Some(symbol) => Some(symbol.clone()),
      None => match self.parent_scope.as_ref() {
        Some(p_scope) => p_scope.get_resolved_symbol(target_type_id),
        None => panic!("Couldn't find type id {}", target_type_id),
      },
    }
  }

  pub fn is_indexable(&self, type_id: i32) -> bool {
    let resolved_symbol = self.get_resolved_symbol(type_id);
    match resolved_symbol {
      Some(Symbol::Array(array_symbol)) => true,
      Some(Symbol::Base(OceanType::String)) => true,
      None => panic!("Couldn't find type id {}", type_id),
      _ => false,
    }
  }

  pub fn get_storage_type_from_indexable(
    &mut self,
    target_type_id: i32,
    index_id: i32,
  ) -> Result<i32, ()> {
    match self.get_resolved_symbol(target_type_id) {
      Some(Symbol::Array(array_symbol)) => match self.match_types(index_id, array_symbol.index) {
        Some(_) => Ok(array_symbol.storage),
        None => Err(()),
      },
      Some(Symbol::Base(OceanType::String)) => match self.match_types(
        index_id,
        get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
      ) {
        Some(_) => Ok(get_base_type_id(Symbol::Base(OceanType::Char))),
        None => Err(()),
      },
      _ => panic!("Could not find target type {}", target_type_id),
    }
  }

  pub fn get_iterator_type_from_indexable(&mut self, target_type_id: i32) -> Result<i32, ()> {
    match self.get_resolved_symbol(target_type_id) {
      Some(Symbol::Array(array_symbol)) => match self.match_types(
        get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
        array_symbol.index,
      ) {
        Some(_) => Ok(array_symbol.storage),
        None => {
          let mut iterator_tuple = TupleSymbol::new();
          iterator_tuple.add_named("value".to_string(), array_symbol.storage);
          iterator_tuple.add_named("index".to_string(), array_symbol.index);
          let iterator_id = self.add_symbol(Symbol::Tuple(iterator_tuple));
          Ok(iterator_id)
        }
      },
      Some(Symbol::Base(OceanType::String)) => Ok(get_base_type_id(Symbol::Base(OceanType::Char))),
      _ => Err(()),
    }
  }

  pub fn get_member_type(&self, target_type_id: i32, member_name: String) -> Result<i32, ()> {
    match self.get_resolved_symbol(target_type_id) {
      Some(Symbol::Tuple(tuple_symbol)) => {
        let mut member_iter = tuple_symbol.members.into_iter();
        match member_iter.find(|x| x.0 == member_name) {
          Some(found_member) => Ok(found_member.1),
          None => Err(()),
        }
      }
      Some(Symbol::Assignable(assignable_symbol)) => {
        self.get_member_type(assignable_symbol.base_type, member_name)
      }
      Some(Symbol::Custom(custom_symbol)) => {
        match custom_symbol.members.get(&member_name) {
          Some(member) => Ok(*member),
          None => Err(())
        }
      }
      _ => Err(()),
    }
  }

  pub fn check_function_parameter_lengths(
    &self,
    target_type_id: i32,
    param_length: usize,
  ) -> Result<(), (bool, usize)> {
    let resolved_symbol = self.get_resolved_symbol(target_type_id);
    match resolved_symbol {
      Some(Symbol::Function(function_symbol)) => {
        if function_symbol.parameters.len() == param_length {
          Ok(())
        } else {
          Err((true, function_symbol.parameters.len()))
        }
      }
      _ => Err((false, 0)),
    }
  }

  pub fn get_function_return_types(
    &mut self,
    target_type_id: i32,
    arguments: &Vec<i32>,
  ) -> Result<i32, Vec<(usize, String, i32)>> {
    let resolved_symbol = self.get_resolved_symbol(target_type_id);
    match resolved_symbol {
      Some(Symbol::Function(function_symbol)) => {
        let mut errors = Vec::new();
        for i in 0..function_symbol.parameters.len() {
          let arg_type_id = arguments[i];
          let (param_name, param_type_id) = function_symbol.parameters[i].clone();
          match self.match_types(arg_type_id, param_type_id) {
            Some(mid) => {}
            None => {
              errors.push((i, param_name, param_type_id));
            }
          };
        }

        if errors.len() != 0 {
          return Err(errors);
        }

        if function_symbol.returns.len() == 0 {
          return Ok(get_base_type_id(Symbol::Base(OceanType::Void)));
        }

        if function_symbol.returns.len() == 1 {
          return Ok(function_symbol.returns[0].1);
        }

        let mut return_symbol = TupleSymbol::new();
        for (return_name, return_type_id) in function_symbol.returns {
          return_symbol.add_named(return_name, return_type_id);
        }

        let return_symbol_id = self.add_symbol(Symbol::Tuple(return_symbol));
        Ok(return_symbol_id)
      }
      _ => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    }
  }

  pub fn get_function_params(&self, target_type_id: i32) -> Vec<(String, i32)> {
    let function_symbol = self.get_resolved_symbol(target_type_id);
    match function_symbol {
      Some(Symbol::Function(function)) => function.parameters,
      _ => Vec::new(),
    }
  }

  pub fn get_function_returns(&self, target_type_id: i32) -> Vec<(String, i32)> {
    let function_symbol = self.get_resolved_symbol(target_type_id);
    match function_symbol {
      Some(Symbol::Function(function)) => function.returns,
      _ => Vec::new(),
    }
  }

  pub fn add_cast(&mut self, from_type: i32, to_type: i32) {
    self.casts.push((from_type, to_type));
  }

  pub fn find_cast(&mut self, from_type: i32, to_type: i32) -> bool {
    for cast in self.casts.clone() {
      let from_match = self.match_types(cast.0, from_type);
      let to_match = self.match_types(cast.1, to_type);
      if from_match.is_some() && to_match.is_some() {
        return true;
      }
    }

    match &mut self.parent_scope {
      Some(p_scope) => p_scope.find_cast(from_type, to_type),
      None => false,
    }
  }

  pub fn get_postfix_operator_type(
    &mut self,
    operator: String,
    target_type_id: i32,
  ) -> Option<i32> {
    None
  }

  pub fn get_prefix_operator_type(&mut self, operator: String, target_type_id: i32) -> Option<i32> {
    let target_symbol = self.get_resolved_symbol(target_type_id);
    match operator.as_str() {
      "!" => match self.match_types(
        target_type_id,
        get_base_type_id(Symbol::Base(OceanType::Bool)),
      ) {
        Some(_) => Some(get_base_type_id(Symbol::Base(OceanType::Bool))),
        None => None,
      },
      "-" => {
        let target_symbol = self.get_resolved_symbol(target_type_id);
        match target_symbol {
          Some(Symbol::Base(OceanType::Unsigned(64))) => {
            Some(get_base_type_id(Symbol::Base(OceanType::Unsigned(64))))
          }
          Some(Symbol::Base(OceanType::Unsigned(x))) => {
            Some(get_base_type_id(Symbol::Base(OceanType::Signed(x * 2))))
          }
          Some(Symbol::Base(OceanType::Signed(_))) => Some(target_type_id),
          Some(Symbol::Base(OceanType::Float(_))) => Some(target_type_id),
          _ => None,
        }
      }
      "~" => {
        let target_symbol = self.get_resolved_symbol(target_type_id);
        match target_symbol {
          Some(Symbol::Base(OceanType::Char))
          | Some(Symbol::Base(OceanType::String))
          | Some(Symbol::Base(OceanType::Unsigned(_)))
          | Some(Symbol::Base(OceanType::Signed(_)))
          | Some(Symbol::Base(OceanType::Float(_))) => Some(target_type_id),
          _ => None,
        }
      }
      _ => None,
    }
  }

  pub fn get_infix_operator_type(
    &mut self,
    operator: String,
    left_type_id: i32,
    right_type_id: i32,
  ) -> Option<i32> {
    match operator.as_str() {
      "+" | "-" | "*" | "/" | "//" | "%" | "|" | "&" | "^" => {
        self.match_types(left_type_id, right_type_id)
      }
      ".." | "..<" => {
        let greater_type_id = self.match_types(left_type_id, right_type_id);
        match greater_type_id {
          Some(storage_type_id) => {
            let index_type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(64)));
            let added = self.add_symbol(Symbol::Array(ArraySymbol::new(
              storage_type_id,
              index_type_id,
            )));
            Some(added)
          }
          None => None,
        }
      }
      "==" | "!=" | "<" | ">" | "<=" | ">=" => {
        let greater_type_id = self.match_types(left_type_id, right_type_id);
        match greater_type_id {
          Some(_) => Some(get_base_type_id(Symbol::Base(OceanType::Bool))),
          None => None,
        }
      }
      "||" | "&&" | "^^" => {
        let a_type_id = get_base_type_id(Symbol::Base(OceanType::Bool));
        let b_type_id = get_base_type_id(Symbol::Base(OceanType::Bool));
        let left_match_type = self.match_types(left_type_id, a_type_id);
        let right_match_type = self.match_types(right_type_id, b_type_id);
        match (left_match_type, right_match_type) {
          (Some(a_type_id), Some(b_type_id)) => Some(a_type_id),
          _ => None,
        }
      }
      ">>" | "<<" => {
        let a_type_id = get_base_type_id(Symbol::Base(OceanType::Signed(64)));
        let right_match_type = self.match_types(right_type_id, a_type_id);
        match right_match_type {
          Some(a_type_id) => Some(left_type_id),
          _ => None,
        }
      }
      "=" => {
        let left_resolved_symbol = self.get_resolved_symbol(left_type_id);
        match left_resolved_symbol {
          Some(Symbol::Assignable(left_symbol)) => {
            match self.match_types(left_symbol.base_type, right_type_id) {
              Some(_) => Some(left_symbol.base_type),
              None => None,
            }
          }
          _ => None,
        }
      }
      _ => None,
    }
  }
}
