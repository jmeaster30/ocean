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

#[derive(Clone, Debug)]
pub enum SymbolTableEntry {
  Variable(Variable),
  Base(BaseSymbolType),
  Pack(Pack),
  Union(Union),
  Interface(Interface),
  Function(Function),
}

#[derive(Clone, Debug, New, Eq, PartialEq, Hash)]
pub struct Variable {
  declaration_span: (usize, usize),
  name: String,
  lazy: bool,
  reference: bool,
  assignable: bool,
  symbol_type: SymbolType,
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SymbolType {
  Base(Uuid),
  Pack(Uuid),
  Union(Uuid),
  Function(Uuid),
  Interface(Uuid),
}

#[derive(Clone, Debug)]
pub struct SymbolTable {
  path_name: Option<String>,
  parent: Option<Rc<RefCell<SymbolTable>>>,
  hard_scope: bool,
  usings: Vec<Rc<RefCell<SymbolTable>>>,
  uuid_map: DoubleMap<Uuid, QuerySymbolType>,
  symbols: HashMap<Uuid, SymbolTableEntry>,
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

  pub fn add_pack_declaration(&mut self, pack_name: &String, pack_span: (usize, usize)) -> Vec<Error> {
    if let Some(pack_uuid) = self.find_pack(pack_name, true) {
      let declared_pack = match self.find_symbol_by_uuid(&pack_uuid, true) {
        SymbolTableEntry::Pack(p) => p,
        _ => panic!("Should not happen :("),
      };
      vec![Error::new_with_metadata(
        Severity::Error,
        pack_span,
        "Pack with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_pack.declaration_span, "Pack already defined here".to_string())
      )]
    } else {
      let mut errors = Vec::new();
      if let Some(found_union_conflict) = self.find_union(pack_name, true) {
        let declared_union = match self.find_symbol_by_uuid(&found_union_conflict, true) {
          SymbolTableEntry::Union(u) => u,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          pack_span,
          "Union with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_union.declaration_span, "Union already defined here".to_string())
        ))
      }
      if let Some(found_interface_conflict) = self.find_interface(pack_name, true) {
        let declared_interface = match self.find_symbol_by_uuid(&found_interface_conflict, true) {
          SymbolTableEntry::Interface(i) => i,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          pack_span,
          "Interface with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_interface.declaration_span, "Interface already defined here".to_string())
        ))
      }

      let uuid = Uuid::new_v4();
      let symbol = SymbolTableEntry::Pack(Pack::new(pack_span, pack_name.clone(), Vec::new(), Vec::new(), HashableMap::new()));
      self.packs.insert(pack_name.clone(), uuid);
      self.symbols.insert(uuid, symbol);
      self.uuid_map.insert(uuid, QuerySymbolType::CustomType(pack_name.clone()));
      errors
    }
  }

  pub fn add_union_declaration(&mut self, union_name: &String, union_span: (usize, usize)) -> Vec<Error> {
    if let Some(union_uuid) = self.find_union(union_name, true) {
      let declared_union = match self.find_symbol_by_uuid(&union_uuid, true) {
        SymbolTableEntry::Union(u) => u,
        _ => panic!("Should not happen :("),
      };
      vec![Error::new_with_metadata(
        Severity::Error,
        union_span,
        "Union with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_union.declaration_span, "Union already declared here.".to_string())
      )]
    } else {
      let mut errors = Vec::new();
      if let Some(found_pack_conflict) = self.find_pack(union_name, true) {
        let declared_pack = match self.find_symbol_by_uuid(&found_pack_conflict, true) {
          SymbolTableEntry::Pack(p) => p,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          union_span,
          "Pack with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_pack.declaration_span, "Pack already defined here".to_string())
        ))
      }
      if let Some(found_interface_conflict) = self.find_interface(union_name, true) {
        let declared_interface = match self.find_symbol_by_uuid(&found_interface_conflict, true) {
          SymbolTableEntry::Interface(i) => i,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          union_span,
          "Interface with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_interface.declaration_span, "Interface already defined here".to_string())
        ))
      }
      let uuid = Uuid::new_v4();
      let symbol = SymbolTableEntry::Union(Union::new(union_span, union_name.clone(), HashableMap::new()));
      self.unions.insert(union_name.clone(), uuid);
      self.symbols.insert(uuid, symbol);
      self.uuid_map.insert(uuid, QuerySymbolType::CustomType(union_name.clone()));
      errors
    }
  }

  pub fn add_interface_declaration(&mut self, interface_name: &String, interface_span: (usize, usize)) -> Vec<Error> {
    if let Some(interface_uuid) = self.find_interface(interface_name, true) {
      let declared_interface = match self.find_symbol_by_uuid(&interface_uuid, true) {
        SymbolTableEntry::Interface(i) => i,
        _ => panic!("Should not happen :("),
      };
      vec![Error::new_with_metadata(
        Severity::Error,
        interface_span,
        "Interface with same name already declared.".to_string(),
        ErrorMetadata::new()
            .extra_highlighted_info(declared_interface.declaration_span, "Interface already declared here.".to_string())
      )]
    } else {
      let mut errors = Vec::new();
      if let Some(found_pack_conflict) = self.find_pack(interface_name, true) {
        let declared_pack = match self.find_symbol_by_uuid(&found_pack_conflict, true) {
          SymbolTableEntry::Pack(p) => p,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          interface_span,
          "Pack with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_pack.declaration_span, "Pack already defined here".to_string())
        ))
      }
      if let Some(found_union_conflict) = self.find_union(interface_name, true) {
        let declared_union = match self.find_symbol_by_uuid(&found_union_conflict, true) {
          SymbolTableEntry::Union(i) => i,
          _ => panic!("Should not happen :("),
        };
        errors.push(Error::new_with_metadata(
          Severity::Error,
          interface_span,
          "Union with same name already declared.".to_string(),
          ErrorMetadata::new()
              .extra_highlighted_info(declared_union.declaration_span, "Union already defined here".to_string())
        ))
      }

      let uuid = Uuid::new_v4();
      let symbol = SymbolTableEntry::Interface(Interface::new(interface_span, interface_name.clone(), HashableMap::new()));
      self.unions.insert(interface_name.clone(), uuid);
      self.symbols.insert(uuid, symbol);
      self.uuid_map.insert(uuid, QuerySymbolType::CustomType(interface_name.clone()));
      errors
    }
  }
  
  pub fn add_pack_type_args(&mut self, pack_name: &String, type_args: Vec<Uuid>) -> Vec<Error> {
    Vec::new()
  }
  
  pub fn add_pack_interfaces(&mut self, pack_name: &String, interfaces: Vec<Uuid>) -> Vec<Error> {
    Vec::new()
  }
  
  pub fn add_pack_members(&mut self, pack_name: &String, members: HashableMap<String, Uuid>) -> Vec<Error> {
    Vec::new()
  }

  pub fn find_interface(&self, interface_name: &String, in_current_scope: bool) -> Option<Uuid> {
    self.find_internal(interface_name, &(|s: &SymbolTable, n: &String| {
      match s.interfaces.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), false, false, in_current_scope)
  }

  pub fn find_variable(&self, variable_name: &String, in_current_scope: bool) -> Option<Uuid> {
    self.find_internal(variable_name, &(|s: &SymbolTable, n: &String| {
      match s.variables.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), false, false, in_current_scope)
  }

  pub fn find_pack(&self, pack_name: &String, in_current_scope: bool) -> Option<Uuid> {
    self.find_internal(pack_name, &(|s: &SymbolTable, n: &String| {
      match s.packs.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), false, false, in_current_scope)
  }

  pub fn find_union(&self, union_name: &String, in_current_scope: bool) -> Option<Uuid> {
    self.find_internal(union_name, &(|s: &SymbolTable, n: &String| {
      match s.unions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), false, false, in_current_scope)
  }

  // TODO this may not be correct cause of parameters and junk
  pub fn find_functions(&self, function_name: &String, in_current_scope: bool) -> Option<Uuid> {
    self.find_internal(function_name, &(|s: &SymbolTable, n: &String| {
      match s.functions.get(n) {
        Some(x) => Some(*x),
        None => None
      }
    }), true, false, in_current_scope)
  }

  fn find_symbol_by_uuid(&self, uuid: &Uuid, in_current_scope: bool) -> SymbolTableEntry {
    self.find_internal(uuid, &(|s: &SymbolTable, u: &Uuid| {
      match s.symbols.get(u) {
        Some(x) => Some(x.clone()),
        None => None
      }
    }), true, true, in_current_scope).unwrap() // TODO is this valid?
  }

  fn find_internal<S, N, R>(&self, name: &N, selector: &S, check_usings: bool, keep_check_usings: bool, in_current_scope: bool) -> Option<R>
    where S: Fn(&SymbolTable, &N) -> Option<R> {
    match selector(&self, &name) {
      Some(x) => Some(x),
      None => {
        if !check_usings || in_current_scope { return None }

        for using in &self.usings {
          match using.borrow().find_internal(name, selector, keep_check_usings, keep_check_usings, in_current_scope) {
            Some(uuid) => return Some(uuid),
            None => {}
          }
        }

        match self.parent.clone() {
          Some(parent) => parent.borrow().find_internal(name, selector, check_usings, keep_check_usings, in_current_scope),
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
