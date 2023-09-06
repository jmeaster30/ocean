use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Type {
  Boolean,
  Array(Box<Type>),
  Layout(HashMap<String, Type>),
  FunctionPointer(Vec<Type>, Box<Type>),
  Reference(Box<Type>),
  Unsigned8,
  Unsigned16,
  Unsigned32,
  Unsigned64,
  Unsigned128,
  Signed8,
  Signed16,
  Signed32,
  Signed64,
  Signed128,
  Float,
}

impl Type {
  pub fn min(t: Type) -> Value {

  }

  pub fn max(t: Type) -> Value {

  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl Value {
  pub fn type_of(&self) -> Type {
    match self {
      Value::Boolean(_) => Type::Boolean,
      Value::Array(_) => todo!(),
      Value::Layout(_) => todo!(),
      Value::FunctionPointer(_) => todo!(),
      Value::Reference(_) => todo!(),
      Value::Unsigned8(_) => Type::Unsigned8,
      Value::Unsigned16(_) => Type::Unsigned16,
      Value::Unsigned32(_) => Type::Unsigned32,
      Value::Unsigned64(_) => Type::Unsigned64,
      Value::Unsigned128(_) => Type::Unsigned128,
      Value::Signed8(_) => Type::Signed8,
      Value::Signed16(_) => Type::Signed16,
      Value::Signed32(_) => Type::Signed32,
      Value::Signed64(_) => Type::Signed64,
      Value::Signed128(_) => Type::Signed128,
      Value::Float => Type::Float,
    }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FunctionPointer {
  pub module: Option<String>,
  pub function: String,
}

impl FunctionPointer {
  pub fn new(module: Option<String>, function: String) -> Self {
    Self { module, function }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Reference {
  Variable(VariableRef),
  ArrayIndex(ArrayIndexRef),
  LayoutIndex(LayoutIndexRef),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VariableRef {
  pub name: String,
}

impl VariableRef {
  pub fn new(name: String) -> Self {
    Self { name }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ArrayIndexRef {
  pub reference: Box<Value>,
  pub index: Box<Value>,
}

impl ArrayIndexRef {
  pub fn new(reference: Box<Value>, index: Box<Value>) -> Self {
    Self { reference, index }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LayoutIndexRef {
  pub reference: Box<Value>,
  pub index: String,
}

impl LayoutIndexRef {
  pub fn new(reference: Box<Value>, index: String) -> Self {
    Self { reference, index }
  }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl PartialOrd for Layout {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    todo!()
  }
}