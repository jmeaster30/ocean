use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Boolean(bool),
  Array(Array),
  Layout(Layout),
  FunctionPointer(FunctionPointer),
  Reference(Reference),

  Unsigned8(u8),
  Unsigned16(u16),
  Unsigned32(u32),
  Unsigned64(u64),
  Unsigned128(u128),

  Signed8(i8),
  Signed16(i16),
  Signed32(i32),
  Signed64(i64),
  Signed128(i128),

  Float,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionPointer {
  pub module: Option<String>,
  pub function: String,
}

impl FunctionPointer {
  pub fn new(module: Option<String>, function: String) -> Self {
    Self { module, function }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
  Variable(VariableRef),
  ArrayIndex(ArrayIndexRef),
  LayoutIndex(LayoutIndexRef),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableRef {
  pub name: String,
}

impl VariableRef {
  pub fn new(name: String) -> Self {
    Self { name }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndexRef {
  pub reference: Box<Value>,
  pub index: Box<Value>,
}

impl ArrayIndexRef {
  pub fn new(reference: Box<Value>, index: Box<Value>) -> Self {
    Self { reference, index }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutIndexRef {
  pub reference: Box<Value>,
  pub index: String,
}

impl LayoutIndexRef {
  pub fn new(reference: Box<Value>, index: String) -> Self {
    Self { reference, index }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  pub length: Box<Value>,
  pub values: Vec<Value>,
}

impl Array {
  pub fn new(length: Box<Value>) -> Self {
    Self {
      length,
      values: Vec::new(),
    }
  }

  pub fn create(length: Box<Value>, values: Vec<Value>) -> Self {
    Self { length, values }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
  pub values: HashMap<String, Value>,
}

impl Layout {
  pub fn new(values: HashMap<String, Value>) -> Self {
    Self { values }
  }
}
