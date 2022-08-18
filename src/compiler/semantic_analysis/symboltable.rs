use std::collections::HashMap;

pub enum Symbol {
  Function(FunctionSymbol),
  Auto(AutoSymbol),
  Base(BaseSymbol),
  Modified(ModifiedSymbol),
  Array(ArraySymbol),
  Unknown,
}

pub struct ArraySymbol {
  storage: Box<Symbol>,
  index: Box<Symbol>,
}

pub struct AutoSymbol {
  constraints: Option<Vec<Symbol>>, // Some(Vec::new) <- any.... None <- none
  members: Vec<String>,
}

pub struct FunctionSymbol {
  parameters: Vec<Symbol>,
  returns: Vec<Symbol>,
}

pub struct ModifiedSymbol {
  reference: bool,
  mutable: bool,
  comp: bool,
  base_type: Box<Symbol>,
}

pub struct BaseSymbol {
  members: HashMap<String, Symbol>,
}

pub struct SymbolTable {
  types: HashMap<String, Symbol>,
  variables: HashMap<String, Symbol>,
  parent_scope: Option<Box<SymbolTable>>,
}
