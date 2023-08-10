use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
  Boolean(bool),
  Character(char),
  String(String),
  Array(Array),
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

#[derive(Debug, Clone)]
pub struct Boolean {
  pub value: bool,
}

impl Boolean {
  pub fn new(value: bool) -> Self { Self { value } }
}

#[derive(Debug, Clone)]
pub struct Character {
  pub value: char,
}

impl Character {
  pub fn new(value: char) -> Self { Self { value } }
}

#[derive(Debug, Clone)]
pub struct StringValue {
  pub value: String,
}

impl StringValue {
  pub fn new(value: String) -> Self { Self { value } }
}



#[derive(Debug, Clone)]
pub struct Integer {
  pub value: u128,
  pub negative: bool,
}

impl Integer {
  pub fn new(value: u128, negative: bool) -> Self { Self { value, negative } }
}

#[derive(Debug, Clone)]
pub struct FunctionPointer {
  pub module: Option<String>,
  pub function: String,
}

impl FunctionPointer {
  pub fn new(module: Option<String>, function: String) -> Self { Self { module, function } }
}

#[derive(Debug, Clone)]
pub enum Reference {
  Variable(VariableRef),
  Index(IndexRef),
}

#[derive(Debug, Clone)]
pub struct VariableRef {
  pub name: String,
}

impl VariableRef {
  pub fn new(name: String) -> Self { Self { name } }
}

#[derive(Debug, Clone)]
pub struct IndexRef {
  pub reference: Box<Reference>,
  pub index: Box<Value>,
}

impl IndexRef {
  pub fn new(reference: Box<Reference>, index: Box<Value>) -> Self { Self { reference, index } }
}

#[derive(Debug, Clone)]
pub struct Array {
  pub length: Box<Value>,
  pub values: Vec<Value>,
}

impl Array {
  pub fn new(length: Box<Value>) -> Self { Self { length, values: Vec::new() } }
}

#[derive(Debug, Clone)]
pub struct Map {
  pub values: HashMap<String, Value>,
}

impl Map {
  pub fn new(values: HashMap<String, Value>) -> Self { Self { values } }
}
