use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ocean_macros::New;
use uuid::Uuid;
use crate::ocean::frontend::ast::{ArrayType, AutoType, BaseType, CustomType, FunctionType, LazyType, MutType, Type, VariableType};
use crate::util::doublemap::DoubleMap;
use crate::util::errors::{Error, ErrorMetadata, Severity};
use crate::util::hashablemap::HashableMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SymbolType {
  BaseType(BaseSymbolType),
  CustomType(String),
  Function(Function),
  Auto(String),
  Mutable(Uuid),
  Reference(Uuid),
  Lazy(Uuid),
  CompoundType(Vec<(String, Uuid)>),
  Pack(Pack),
  Union(Union),
  Interface(Interface),
  Variable(Variable),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum BaseSymbolType {
  I8,
  I16,
  I32,
  I64,
  I128,
  F32,
  F64,
  U8,
  U16,
  U32,
  U64,
  U128,
  String,
  Bool,
  Char
}

#[derive(Clone, Debug, New)]
pub struct Symbol {
  constant: Option<bool>,
  assignable: bool,
  //all_types_possible: bool, // if this is true then an empty list in possible_types means "Any types" but if it is false then an empty list in possible_types means "No possible types"
  //possible_types: Vec<SymbolType>,
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Variable {
  declaration_span: (usize, usize),
  symbol: Uuid
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Pack {
  name: String,
  type_args: Vec<Uuid>,
  interfaces: Vec<Uuid>,
  members: HashableMap<String, Uuid>
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Union {
  name: String,
  members: HashableMap<String, Vec<Uuid>>
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Interface {
  name: String,
  functions: HashableMap<String, Function>,
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Function {
  name: String,
  arguments: Vec<(String, Uuid)>,
  returns: Vec<(String, Uuid)>,
}

#[derive(Clone, Debug)]
enum SymbolTableEntryType {
  Base(Symbol),
  Variable(String),
  Pack(String),
  Union(String),
  Function(String),
  Interface(String),
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  path_name: Option<String>,
  parent: Option<Rc<RefCell<SymbolTable>>>,
  hard_scope: bool,
  usings: Vec<Rc<RefCell<SymbolTable>>>,
  uuid_map: DoubleMap<Uuid, SymbolType>,
  symbols: HashMap<Uuid, Symbol>,
  variables: HashMap<String, Uuid>,
  packs: HashMap<String, Uuid>,
  unions: HashMap<String, Uuid>,
  functions: HashMap<String, Uuid>,
  interfaces: HashMap<String, Uuid>,
}

impl SymbolTable {
  pub fn global_scope(path_name: String) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      path_name: Some(path_name),
      parent: None,
      hard_scope: true,
      usings: Vec::new(),
      uuid_map: DoubleMap::new(),
      symbols: HashMap::new(),
      variables: HashMap::new(),
      packs: HashMap::new(),
      unions: HashMap::new(),
      functions: HashMap::new(),
      interfaces: HashMap::new(),
    }))
  }

  pub fn soft_scope(parent_scope: Rc<RefCell<SymbolTable>>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      path_name: None,
      parent: Some(parent_scope),
      hard_scope: false,
      usings: Vec::new(),
      uuid_map: DoubleMap::new(),
      symbols: HashMap::new(),
      variables: HashMap::new(),
      packs: HashMap::new(),
      unions: HashMap::new(),
      functions: HashMap::new(),
      interfaces: HashMap::new(),
    }))
  }

  pub fn hard_scope(parent_scope: Option<Rc<RefCell<SymbolTable>>>) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      path_name: None,
      parent: parent_scope,
      hard_scope: true,
      usings: Vec::new(),
      uuid_map: DoubleMap::new(),
      symbols: HashMap::new(),
      variables: HashMap::new(),
      packs: HashMap::new(),
      unions: HashMap::new(),
      functions: HashMap::new(),
      interfaces: HashMap::new(),
    }))
  }

  pub fn add_using_table(&mut self, symbol_table: Rc<RefCell<SymbolTable>>) {
    self.usings.push(symbol_table);
  }

  pub fn find_interface(&self, interface_name: &String) -> Option<Uuid> {
    self.find_internal(interface_name, &(|s: &SymbolTable, n: &String| {
      match s.interfaces.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true)
  }

  pub fn find_variable(&self, variable_name: &String) -> Option<Uuid> {
    self.find_internal(variable_name, &(|s: &SymbolTable, n: &String| {
      match s.variables.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true)
  }

  pub fn find_pack(&self, pack_name: &String) -> Option<Uuid> {
    self.find_internal(pack_name, &(|s: &SymbolTable, n: &String| {
      match s.packs.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true)
  }

  pub fn find_union(&self, union_name: &String) -> Option<Uuid> {
    self.find_internal(union_name, &(|s: &SymbolTable, n: &String| {
      match s.unions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true)
  }

  // TODO this may not be correct cause of parameters and junk
  pub fn find_functions(&self, function_name: &String) -> Option<Uuid> {
    self.find_internal(function_name, &(|s: &SymbolTable, n: &String| {
      match s.functions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true)
  }

  fn find_internal<F>(&self, name: &String, selector: &F, check_usings: bool) -> Option<Uuid>
    where F: Fn(&SymbolTable, &String) -> Option<Uuid> {
    match selector(&self, &name) {
      Some(x) => Some(x),
      None => {
        if !check_usings { return None }

        for using in &self.usings {
          match using.borrow().find_internal(name, selector, false) {
            Some(uuid) => return Some(uuid),
            None => {}
          }
        }

        match self.parent.clone() {
          Some(parent) => parent.borrow().find_internal(name, selector, check_usings),
          None => None,
        }
      }
    }
  }

  pub fn add_type(&mut self, new_type: Type) -> Uuid {
    match new_type {
      Type::Base(base) => self.add_base_type(base),
      Type::Custom(custom) => self.add_custom_type(custom),
      Type::Auto(auto) => self.add_auto_type(auto),
      Type::Lazy(lazy) => self.add_lazy_type(lazy),
      Type::Ref(_) => todo!(),
      Type::Mutable(mutable) => self.add_mutable_type(mutable),
      Type::Function(func) => self.add_function_type(func),
      Type::Array(array) => self.add_array_type(array),
      Type::VariableType(var) => self.add_variable_type(var),
      Type::TupleType(_) => todo!()
    }
  }

  fn add_base_type(&mut self, new_type: BaseType) -> Uuid {
    let symbol_type = SymbolType::BaseType(match new_type.base_type.lexeme.as_str() {
      "i8" => BaseSymbolType::I8,
      "i16" => BaseSymbolType::I16,
      "i32" => BaseSymbolType::I32,
      "i64" => BaseSymbolType::I64,
      "i128" => BaseSymbolType::I128,
      "f32" => BaseSymbolType::F32,
      "f64" => BaseSymbolType::F64,
      "u8" => BaseSymbolType::U8,
      "u16" => BaseSymbolType::U16,
      "u32" => BaseSymbolType::U32,
      "u64" => BaseSymbolType::U64,
      "u128" => BaseSymbolType::U128,
      "string" => BaseSymbolType::String,
      "bool" => BaseSymbolType::Bool,
      "char" => BaseSymbolType::Char,
      _ => panic!()
    });
    //let symbol = Symbol::new(symbol_type, None, false, false, vec![symbol_type]);
    let uuid = Uuid::new_v4();
    //self.uuid_map.insert(uuid, SymbolTableEntryType::Base(symbol));
    uuid
  }

  fn add_custom_type(&mut self, new_type: CustomType) -> Uuid {
    let symbol_type = SymbolType::CustomType(new_type.identifier.lexeme.clone());
    //let symbol = Symbol::new(symbol_type, None, false, false, vec![symbol_type]);
    let uuid = Uuid::new_v4();
    //self.uuid_map.insert(uuid, SymbolTableEntryType::Base(symbol));
    uuid
  }

  fn add_auto_type(&mut self, new_type: AutoType) -> Uuid {
    Uuid::new_v4()
  }

  fn add_lazy_type(&mut self, new_type: LazyType) -> Uuid {
    Uuid::new_v4()
  }

  fn add_mutable_type(&mut self, new_type: MutType) -> Uuid {
    Uuid::new_v4()
  }

  fn add_function_type(&mut self, new_type: FunctionType) -> Uuid {
    Uuid::new_v4()
  }

  fn add_array_type(&mut self, new_type: ArrayType) -> Uuid {
    Uuid::new_v4()
  }

  fn add_variable_type(&mut self, new_type: VariableType) -> Uuid {
    Uuid::new_v4()
  }

  /*pub fn check_for_variable(&self, variable_name: String, only_check_current_scope: bool) -> Option<Variable> {
    match self.variables.get(&*variable_name) {
      Some(x) => Some(x.clone()),
      None if self.hard_scope || only_check_current_scope => None,
      None if !self.hard_scope => match self.parent.clone() {
        Some(parent ) => parent.borrow().check_for_variable(variable_name, only_check_current_scope),
        None => None,
      },
      _ => panic!("shouldn't get here")
    }
  }

  pub fn add_variable(&mut self, variable_name: String, variable_decl_span: (usize, usize), variable_type: Uuid) -> Result<(), Error> {
    if let Some(variable_data) = self.check_for_variable(variable_name.clone(), false) {
      return Err(Error::new_with_metadata(
        Severity::Error,
        variable_decl_span,
        format!("A variable of the name '{}' has already been declared in this scope.", variable_name.clone()),
        ErrorMetadata::new()
          .extra_highlighted_info(variable_data.decl_span, format!("Variable '{}' already declared here", variable_name))
      ));
    }
    self.variables.insert(variable_name.clone(), Variable::new(variable_decl_span, variable_type));
    Ok(())
  }*/
}

#[derive(Copy, Clone, Debug, New)]
pub struct AnalyzerContext {
  pub in_loop: bool,
  pub assignment_target: bool,
  pub right_hand_side_type: Option<Uuid>,
  // add pattern expression for match arms
}

impl AnalyzerContext {
  pub fn default() -> Self {
    Self {
      in_loop: false,
      assignment_target: false,
      right_hand_side_type: None,
    }
  }

  pub fn create_in_loop(self) -> Self {
    Self {
      in_loop: true,
      assignment_target: self.assignment_target,
      right_hand_side_type: self.right_hand_side_type,
    }
  }

  pub fn create_assignment_target(self, right_hand_side_type: Option<Uuid>) -> Self {
    Self {
      in_loop: self.in_loop,
      assignment_target: true,
      right_hand_side_type,
    }
  }
}
