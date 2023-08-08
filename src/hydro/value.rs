use super::instruction::Instruction;

#[derive(Debug, Clone)]
pub enum Value {
  Boolean(bool),
  Character(char),
  String(String),
  Function(Function),
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
pub struct Function {
  pub name: String,
  pub parameters: Vec<String>,
  pub body: Vec<Instruction>
}

impl Function {
  pub fn new(name: String, parameters: Vec<String>, body: Vec<Instruction>) -> Self { Self { name, parameters, body } }
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
pub enum Reference {
  Variable(VariableRef),
  Index(IndexRef)
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