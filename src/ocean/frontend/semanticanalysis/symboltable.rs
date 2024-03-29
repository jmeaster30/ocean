use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use ocean_macros::New;
use uuid::Uuid;
use crate::util::errors::{Error, ErrorMetadata, Severity};

#[derive(Clone, Debug)]
pub enum SymbolType {
  BaseType(BaseSymbolType),
  CustomType(Uuid),
  Function(Uuid),
  Auto(Uuid),
  Mutable(Uuid),
  Reference(Uuid),
  Lazy(Uuid),
  CompoundType(Vec<(String, Uuid)>),
  Union(Uuid),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Symbol {
  symbol_type: SymbolType,
  constant: Option<bool>,
  assignable: bool,
  all_types_possible: bool, // if this is true then an empty list in possible_types means "Any types" but if it is false then an empty list in possible_types means "No possible types"
  possible_types: Vec<SymbolType>,
}

#[derive(Clone, Debug, New)]
pub struct Variable {
  decl_span: (usize, usize),
  symbol: Uuid
}

#[derive(Clone, Debug, New)]
pub struct Pack {
  name: String,
  interfaces: Vec<Uuid>,
  members: HashMap<String, Uuid>
}

#[derive(Clone, Debug, New)]
pub struct Union {
  name: String,
  members: HashMap<String, Vec<Uuid>>
}

#[derive(Clone, Debug, New)]
pub struct Interface {
  name: String,
  functions: HashMap<String, Function>,
}

#[derive(Clone, Debug, New)]
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
  uuid_map: HashMap<Uuid, SymbolTableEntryType>,
  variables: HashMap<String, Variable>,
  packs: HashMap<String, Pack>,
  unions: HashMap<String, Union>,
  functions: HashMap<String, Function>,
  interfaces: HashMap<String, Interface>,
}

impl SymbolTable {
  pub fn global_scope(path_name: String) -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self {
      path_name: Some(path_name),
      parent: None,
      hard_scope: true,
      usings: Vec::new(),
      uuid_map: HashMap::new(),
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
      uuid_map: HashMap::new(),
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
      uuid_map: HashMap::new(),
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

  pub fn check_for_variable(&self, variable_name: String, only_check_current_scope: bool) -> Option<Variable> {
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
  }
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
