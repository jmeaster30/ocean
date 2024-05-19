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
pub enum QuerySymbolType {
  BaseType(BaseSymbolType),
  CustomType(String),
  Function(Vec<Uuid>, Vec<Uuid>),
  Auto(String),
  Mutable(Uuid),
  Reference(Uuid),
  Lazy(Uuid),
  CompoundType(Vec<(String, Uuid)>),
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
  lazy: bool,
  reference: bool,
  assignable: bool,
  symbol_type: SymbolType, // this may need to be possible types
  //all_types_possible: bool, // if this is true then an empty list in possible_types means "Any types" but if it is false then an empty list in possible_types means "No possible types"
  //possible_types: Vec<SymbolType>,
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Variable {
  declaration_span: (usize, usize),
  name: String,
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Pack {
  declaration_span: (usize, usize),
  name: String,
  type_args: Vec<Uuid>,
  interfaces: Vec<Uuid>,
  members: HashableMap<String, Uuid>
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Union {
  declaration_span: (usize, usize),
  name: String,
  members: HashableMap<String, Vec<Uuid>>
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Interface {
  declaration_span: (usize, usize),
  name: String,
  functions: HashableMap<String, Function>,
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Function {
  declaration_span: (usize, usize),
  name: String,
  arguments: Vec<(String, Uuid)>,
  returns: Vec<(String, Uuid)>,
}

#[derive(Clone, Debug)]
pub enum SymbolType {
  Base(BaseSymbolType),
  Variable(Variable),
  Pack(Pack),
  Union(Union),
  Function(Function),
  Interface(Interface),
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  path_name: Option<String>,
  parent: Option<Rc<RefCell<SymbolTable>>>,
  hard_scope: bool,
  usings: Vec<Rc<RefCell<SymbolTable>>>,
  uuid_map: DoubleMap<Uuid, QuerySymbolType>,
  symbols: HashMap<Uuid, Symbol>,
  autos: HashMap<String, Uuid>,
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
      autos: HashMap::new(),
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
      autos: HashMap::new(),
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
      autos: HashMap::new(),
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

  pub fn add_pack_declaration(&mut self, pack_name: &String, pack_span: (usize, usize)) -> Result<(), Error> {
    if let Some(pack_uuid) = self.find_pack(pack_name) {
      let declared_pack = match self.find_symbol_by_uuid(&pack_uuid).symbol_type {
        SymbolType::Pack(p) => p,
        _ => panic!("Should not happen :("),
      };
      Err(Error::new_with_metadata(
        Severity::Error,
        pack_span,
        "Pack with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_pack.declaration_span, "Pack already defined here".to_string())
      ))
    } else {
      let uuid = Uuid::new_v4();
      let pack = SymbolType::Pack(Pack::new(pack_span, pack_name.clone(), Vec::new(), Vec::new(), HashableMap::new()));
      let symbol = Symbol::new(false, false, false, pack);
      self.packs.insert(pack_name.clone(), uuid);
      self.symbols.insert(uuid, symbol);
      self.uuid_map.insert(uuid, QuerySymbolType::CustomType(pack_name.clone()));
      Ok(())
    }
  }

  pub fn add_union_declaration(&mut self, union_name: &String, union_span: (usize, usize)) -> Result<(), Error> {
    if let Some(union_uuid) = self.find_union(union_name) {
      let declared_union = match self.find_symbol_by_uuid(&union_uuid).symbol_type {
        SymbolType::Union(u) => u,
        _ => panic!("Should not happen :("),
      };
      Err(Error::new_with_metadata(
        Severity::Error,
        union_span,
        "Union with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_union.declaration_span, "Union already declared here.".to_string())
      ))
    } else {
      Ok(())
    }
  }

  pub fn add_interface_declaration(&mut self, interface_name: &String, interface_span: (usize, usize)) -> Result<(), Error> {
    if let Some(interface_uuid) = self.find_interface(interface_name) {
      let declared_interface = match self.find_symbol_by_uuid(&interface_uuid).symbol_type {
        SymbolType::Interface(i) => i,
        _ => panic!("Should not happen :("),
      };
      Err(Error::new_with_metadata(
        Severity::Error,
        interface_span,
        "Interface with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_interface.declaration_span, "Interface already declared here.".to_string())
      ))
    } else {
      Ok(())
    }
  }

  pub fn find_interface(&self, interface_name: &String) -> Option<Uuid> {
    self.find_internal(interface_name, &(|s: &SymbolTable, n: &String| {
      match s.interfaces.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false)
  }

  pub fn find_variable(&self, variable_name: &String) -> Option<Uuid> {
    self.find_internal(variable_name, &(|s: &SymbolTable, n: &String| {
      match s.variables.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false)
  }

  pub fn find_pack(&self, pack_name: &String) -> Option<Uuid> {
    self.find_internal(pack_name, &(|s: &SymbolTable, n: &String| {
      match s.packs.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false)
  }

  pub fn find_union(&self, union_name: &String) -> Option<Uuid> {
    self.find_internal(union_name, &(|s: &SymbolTable, n: &String| {
      match s.unions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false)
  }

  // TODO this may not be correct cause of parameters and junk
  pub fn find_functions(&self, function_name: &String) -> Option<Uuid> {
    self.find_internal(function_name, &(|s: &SymbolTable, n: &String| {
      match s.functions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false)
  }

  fn find_symbol_by_uuid(&self, uuid: &Uuid) -> Symbol {
    self.find_internal(uuid, &(|s: &SymbolTable, u: &Uuid| {
      match s.symbols.get(u) {
        Some(x) => Some(x.clone()),
        None => None
      }
    }), true, true).unwrap() // TODO is this valid?
  }

  fn find_internal<S, N, R>(&self, name: &N, selector: &S, check_usings: bool, keep_check_usings: bool) -> Option<R>
    where S: Fn(&SymbolTable, &N) -> Option<R> {
    match selector(&self, &name) {
      Some(x) => Some(x),
      None => {
        if !check_usings { return None }

        for using in &self.usings {
          match using.borrow().find_internal(name, selector, keep_check_usings, keep_check_usings) {
            Some(uuid) => return Some(uuid),
            None => {}
          }
        }

        match self.parent.clone() {
          Some(parent) => parent.borrow().find_internal(name, selector, check_usings, keep_check_usings),
          None => None,
        }
      }
    }
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
