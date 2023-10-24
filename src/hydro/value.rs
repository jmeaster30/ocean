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
  Float32,
  Float64,
  // TODO I want F8 to F128 but we are limited by rust types for now
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
      Type::Float32 => Value::Float32(0.0),
      Type::Float64 => Value::Float64(0.0),
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
      (Type::Float32, Type::Float32) => true,
      (Type::Float64, Type::Float64) => true,
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

  Float32(f32),
  Float64(f64),
}

impl Value {
  pub fn type_of(&self) -> Type {
    match self {
      Value::Boolean(_) => Type::Boolean,
      Value::Array(array) => Type::Array(array.length.to_u64().unwrap(), Box::new(array.value_type.clone())),
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
      Value::Float32(_) => Type::Float32,
      Value::Float64(_) => Type::Float64,
    }
  }

  pub fn to_u64(&self) -> Result<u64, String> {
    match self {
      Value::Unsigned8(x) => Ok(*x as u64),
      Value::Unsigned16(x) => Ok(*x as u64),
      Value::Unsigned32(x) => Ok(*x as u64),
      Value::Unsigned64(x) => Ok(*x),
      Value::Unsigned128(x) => Ok(*x as u64),
      Value::Signed8(x) => Ok(*x as u64),
      Value::Signed16(x) => Ok(*x as u64),
      Value::Signed32(x) => Ok(*x as u64),
      Value::Signed64(x) => Ok(*x as u64),
      Value::Signed128(x) => Ok(*x as u64),
      Value::Float32(x) => Ok(*x as u64),
      Value::Float64(x) => Ok(*x as u64),
      _ => Err(format!("Cannot convert {:?} to u64", self)),
    }
  }

  pub fn to_u8(&self) -> Result<u8, String> {
    match self {
      Value::Unsigned8(x) => Ok(*x),
      _ => Err(format!("Cannot convert {:?} to u8", self)),
    }
  }

  pub fn to_bool(&self) -> Result<bool, String> {
    match self {
      Value::Boolean(x) => Ok(*x),
      _ => Err(format!("Cannot convert {:?} to a boolean value :(", self)),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Value::Boolean(value) => if *value { "true" } else { "false" }.to_string(),
      Value::Array(array) => if Type::subset(&array.value_type, &Type::Unsigned8) {
        String::from_utf8(array.values.iter().map(|x| x.to_u8().unwrap()).collect()).unwrap()
      } else {
        let mut result = "[".to_string();
        for idx in 0..array.values.len() {
          let value = array.values[idx].clone();
          result += value.to_string().as_str();
          if idx != array.values.len() - 1 {
            result += ", "
          }
        }

        result + "]"
      },
      Value::Layout(layout) => {
        let mut result = format!("{}.{}{{", layout.module_name, layout.layout_name);
        for idx in 0..layout.values.keys().len() {
          let key = layout.values.keys().nth(idx).unwrap();
          let value = layout.values.get(key).unwrap();
          result += format!("'{}': {}", key, value.to_string()).as_str();
          if idx != layout.values.len() - 1 {
            result += ", "
          }
        }

        result + "}"
      },
      Value::FunctionPointer(pointer) => format!("function {:?} {}", pointer.module, pointer.function),
      Value::Reference(refer) => format!("{:?}", refer),
      Value::Unsigned8(x) => x.to_string(),
      Value::Unsigned16(x) => x.to_string(),
      Value::Unsigned32(x) => x.to_string(),
      Value::Unsigned64(x) => x.to_string(),
      Value::Unsigned128(x) => x.to_string(),
      Value::Signed8(x) => x.to_string(),
      Value::Signed16(x) => x.to_string(),
      Value::Signed32(x) => x.to_string(),
      Value::Signed64(x) => x.to_string(),
      Value::Signed128(x) => x.to_string(),
      Value::Float32(x) => x.to_string(),
      Value::Float64(x) => x.to_string(),
    }
  }

  pub fn index(&self, index: u64) -> Result<Value, String> {
    match self {
      Value::Array(array) => if (index as usize) <= array.values.len() {
        Ok(array.values[index as usize].clone())
      } else {
        Err(format!("Array index out of bounds. Tried to index array of length {:?} with index {}", *array.length, index))
      },
      _ => Err(format!("Cannot index a {:?} :(", self.type_of())),
    }
  }

  pub fn set_index(&mut self, index: u64, value: Value) -> Result<(), String> {
    match self {
      Value::Array(array) => if (index as usize) <= array.values.len() {
        if Type::subset(&value.type_of(), &array.value_type) {
          array.values[index as usize] = value.clone();
          Ok(())
        } else {
          Err(format!("Cannot insert value {:?} of type {:?} into array of type {:?}", value, value.type_of(), array.value_type))
        }
      } else {
        Err(format!("Array index out of bounds. Tried to index array of length {:?} with index {}", *array.length, index))
      },
      _ => Err(format!("Cannot index a {:?} :(", self.type_of())),
    }
  }

  pub fn get_member(&self, member: String) -> Result<Value, String> {
    match self {
      Value::Layout(layout) => match layout.values.get(member.as_str()) {
        Some(value) => Ok(value.clone()),
        None => Err(format!("{:?} does not have the member '{}'", layout, member)),
      }
      Value::Array(array) => if member.as_str() == "length" {
        Ok(*array.length.clone())
      } else {
        Err(format!("{:?} does not have the member '{}'", array, member))
      }
      _ => Err(format!("{:?} does not have any member variables", self.type_of())),
    }
  }

  pub fn set_member(&mut self, member: String, value: Value) -> Result<(), String> {
    match self {
      Value::Layout(layout) => if layout.values.contains_key(member.as_str()) {
        layout.values.insert(member, value);
        Ok(())
      } else {
        Err(format!("{:?} does not have the member '{}'", layout, member))
      }
      Value::Array(_) => Err(format!("{:?} has no modifiable member variables", self.type_of())),
      _ => Err(format!("{:?} does not have any member variables", self.type_of())),
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
