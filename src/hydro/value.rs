use ocean_macros::*;

use super::instruction::Instruction;
use super::executioncontext::ExecutionContext;

use std::cell::RefCell;
use std::rc::Rc;

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

impl Value {
  pub fn resolve(&self, context: Rc<RefCell<ExecutionContext>>) -> Value {
    match self {
      Value::Reference(x) => todo!("resolve ref values"), 
      _ => self.clone(),
    }
  }

  pub fn bool(&self, context: Rc<RefCell<ExecutionContext>>) -> bool {
    match self.resolve(context) {
      Value::Boolean(x) => x,
      _ => panic!("Bool does not make sense for this Value type :("),
    }
  }

  

  pub fn add(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Character(a), Value::Character(b)) => Value::String(a.to_string() + b.to_string().as_str()),
      (Value::Character(a), Value::String(b)) => Value::String(a.to_string() + b.as_str()),
      (Value::String(a), Value::Character(b)) => Value::String(a + b.to_string().as_str()),
      (Value::String(a), Value::String(b)) => Value::String(a + b.as_str()), // there has to be a better way than '.as_str()'
      (a, b) => make_add_numbers!(),
    }
  }

  pub fn sub(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn mult(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn div(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn modulo(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn shiftleft(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Unsigned8(a), Value::Unsigned8(b)) => Value::Unsigned8(a << b),
      (Value::Unsigned16(a), Value::Unsigned8(b)) => Value::Unsigned16(a << b),
      (Value::Unsigned32(a), Value::Unsigned8(b)) => Value::Unsigned32(a << b),
      (Value::Unsigned64(a), Value::Unsigned8(b)) => Value::Unsigned64(a << b),
      (Value::Unsigned128(a), Value::Unsigned8(b)) => Value::Unsigned128(a << b),
      (Value::Signed8(a), Value::Unsigned8(b)) => Value::Signed8(a << b),
      (Value::Signed16(a), Value::Unsigned8(b)) => Value::Signed16(a << b),
      (Value::Signed32(a), Value::Unsigned8(b)) => Value::Signed32(a << b),
      (Value::Signed64(a), Value::Unsigned8(b)) => Value::Signed64(a << b),
      (Value::Signed128(a), Value::Unsigned8(b)) => Value::Signed128(a << b),
      
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn shiftright(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Unsigned8(a), Value::Unsigned8(b)) => Value::Unsigned8(a >> b),
      (Value::Unsigned16(a), Value::Unsigned8(b)) => Value::Unsigned16(a >> b),
      (Value::Unsigned32(a), Value::Unsigned8(b)) => Value::Unsigned32(a >> b),
      (Value::Unsigned64(a), Value::Unsigned8(b)) => Value::Unsigned64(a >> b),
      (Value::Unsigned128(a), Value::Unsigned8(b)) => Value::Unsigned128(a >> b),
      (Value::Signed8(a), Value::Unsigned8(b)) => Value::Signed8(a >> b),
      (Value::Signed16(a), Value::Unsigned8(b)) => Value::Signed16(a >> b),
      (Value::Signed32(a), Value::Unsigned8(b)) => Value::Signed32(a >> b),
      (Value::Signed64(a), Value::Unsigned8(b)) => Value::Signed64(a >> b),
      (Value::Signed128(a), Value::Unsigned8(b)) => Value::Signed128(a >> b),
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn bitand(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitor(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitxor(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitnot(&self, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);

    match a_value {
      Value::Unsigned8(a) => Value::Unsigned8(!a),
      Value::Unsigned16(a) => Value::Unsigned16(!a),
      Value::Unsigned32(a) => Value::Unsigned32(!a),
      Value::Unsigned64(a) => Value::Unsigned64(!a),
      Value::Unsigned128(a) => Value::Unsigned128(!a),
      Value::Signed8(a) => Value::Signed8(!a),
      Value::Signed16(a) => Value::Signed16(!a),
      Value::Signed32(a) => Value::Signed32(!a),
      Value::Signed64(a) => Value::Signed64(!a),
      Value::Signed128(a) => Value::Signed128(!a),
      _ => panic!("Operator unimplemented for type"),
    }
  }
  
  pub fn and(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a && b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }
  
  pub fn or(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a || b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn xor(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a != b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn not(&self, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);

    match a_value {
      Value::Boolean(a) => Value::Boolean(!a),
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn equal(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a == b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a == b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() == b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a == b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a == b),
      (a, b) => make_add_numbers!(a, b),
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn notequal(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a != b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() != b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a != b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a != b),
      (a, b) => make_add_numbers!(a, b),
    }

  }

  pub fn lessthan(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a < b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a < b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() < b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a < b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a < b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn greaterthan(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a > b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a > b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() > b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a > b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a > b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn lessthanequal(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a <= b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a <= b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() <= b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a <= b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a <= b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn greaterthanequal(&self, value: Value, context: Rc<RefCell<ExecutionContext>>) -> Value {
    let a_value = self.resolve(context);
    let b_value = value.resolve(context);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a >= b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a >= b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() >= b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a >= b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a >= b),
      (a, b) => make_add_numbers!(a, b),
    }
  }
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