use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Any,
  Boolean,
  Array(u64, Box<Type>),
  Layout(String, String, Option<HashMap<String, Type>>),
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

impl PartialOrd for Type {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    todo!()
  }
}

impl Type {
  pub fn default(&self) -> Value {
    match self {
      Type::Any => todo!("default value for any type. This should likely not be possible"),
      Type::Boolean => Value::Boolean(false),
      Type::Unsigned8 => Value::Unsigned8(0),
      Type::Unsigned16 => Value::Unsigned16(0),
      Type::Unsigned32 => Value::Unsigned32(0),
      Type::Unsigned64 => Value::Unsigned64(0),
      Type::Unsigned128 => Value::Unsigned128(0),
      Type::Signed8 => Value::Signed8(0),
      Type::Signed16 => Value::Signed16(0),
      Type::Signed32 => Value::Signed32(0),
      Type::Signed64 => Value::Signed64(0),
      Type::Signed128 => Value::Signed128(0),
      Type::Float => todo!(),
      Type::FunctionPointer(_, _) => todo!("default value for function pointer. Should this even be possible??"),
      Type::Array(length, subtype) => {
        let mut values = Vec::new();
        for _ in 0..*length {
          values.push((*subtype).default());
        }
        Value::Array(Array {
          value_type: (**subtype).clone(),
          length: Box::new(Value::Unsigned64(*length)),
          values,
        })
      }
    Type::Reference(_) => todo!("default value for reference. Should this even be possible??"),
      Type::Layout(module_name, layout_name, Some(subtypes)) => {
        let mut values = HashMap::new();
        for (member_name, subtype) in subtypes {
          values.insert(member_name.clone(), subtype.default());
        }
        Value::Layout(Layout::new(
          module_name.clone(),
          layout_name.clone(),
          values,
        ))
      }
      Type::Layout(module_name, layout_name, None) => panic!("Unresolved type :( {} {}", module_name, layout_name),
    }
  }

  pub fn min(t: Type) -> Value {
    match t {
      Type::Boolean => Value::Boolean(false),
      Type::Unsigned8 => Value::Unsigned8(u8::MIN),
      Type::Unsigned16 => Value::Unsigned16(u16::MIN),
      Type::Unsigned32 => Value::Unsigned32(u32::MIN),
      Type::Unsigned64 => Value::Unsigned64(u64::MIN),
      Type::Unsigned128 => Value::Unsigned128(u128::MIN),
      Type::Signed8 => Value::Signed8(i8::MIN),
      Type::Signed16 => Value::Signed16(i16::MIN),
      Type::Signed32 => Value::Signed32(i32::MIN),
      Type::Signed64 => Value::Signed64(i64::MIN),
      Type::Signed128 => Value::Signed128(i128::MIN),
      _ => panic!("This type doesn't have a minimum"),
    }
  }

  pub fn max(t: Type) -> Value {
    match t {
      Type::Boolean => Value::Boolean(true),
      Type::Unsigned8 => Value::Unsigned8(u8::MAX),
      Type::Unsigned16 => Value::Unsigned16(u16::MAX),
      Type::Unsigned32 => Value::Unsigned32(u32::MAX),
      Type::Unsigned64 => Value::Unsigned64(u64::MAX),
      Type::Unsigned128 => Value::Unsigned128(u128::MAX),
      Type::Signed8 => Value::Signed8(i8::MAX),
      Type::Signed16 => Value::Signed16(i16::MAX),
      Type::Signed32 => Value::Signed32(i32::MAX),
      Type::Signed64 => Value::Signed64(i64::MAX),
      Type::Signed128 => Value::Signed128(i128::MAX),
      _ => panic!("This type doesn't have a maximum"),
    }
  }

  pub fn subset(sub: &Type, sup: &Type) -> bool {
    // TODO type subsetting
    match (sub, sup) {
      (_, Type::Any) => true,
      (Type::Boolean, Type::Boolean) => true,
      (Type::Unsigned8, Type::Unsigned8) => true,
      (Type::Unsigned16, Type::Unsigned16) => true,
      (Type::Unsigned32, Type::Unsigned32) => true,
      (Type::Unsigned64, Type::Unsigned64) => true,
      (Type::Unsigned128, Type::Unsigned128) => true,
      (Type::Signed8, Type::Signed8) => true,
      (Type::Signed16, Type::Signed16) => true,
      (Type::Signed32, Type::Signed32) => true,
      (Type::Signed64, Type::Signed64) => true,
      (Type::Signed128, Type::Signed128) => true,
      _ => false,
    }
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
      Value::Array(array) => Type::Array(array.length.to_u64(), Box::new(array.value_type.clone())),
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

  pub fn to_u64(&self) -> u64 {
    match self {
      Value::Unsigned8(x) => *x as u64,
      Value::Unsigned16(x) => *x as u64,
      Value::Unsigned32(x) => *x as u64,
      Value::Unsigned64(x) => *x,
      Value::Unsigned128(x) => *x as u64,
      Value::Signed8(x) => *x as u64,
      Value::Signed16(x) => *x as u64,
      Value::Signed32(x) => *x as u64,
      Value::Signed64(x) => *x as u64,
      Value::Signed128(x) => *x as u64,
      _ => panic!("Cannot convert {:?} to u64", self)
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
  pub value_type: Type,
  pub length: Box<Value>,
  pub values: Vec<Value>,
}

impl Array {
  pub fn new(value_type: Type, length: Box<Value>) -> Self {
    Self {
      value_type,
      length,
      values: Vec::new(),
    }
  }

  pub fn create(value_type: Type, length: Box<Value>, values: Vec<Value>) -> Self {
    Self { value_type, length, values }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
  pub module_name: String,
  pub layout_name: String,
  pub values: HashMap<String, Value>,
}

impl Layout {
  pub fn new(module_name: String, layout_name: String, values: HashMap<String, Value>) -> Self {
    Self {
      module_name,
      layout_name,
      values,
    }
  }
}

impl PartialOrd for Layout {
  fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
    todo!()
  }
}
