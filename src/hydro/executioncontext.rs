use super::value::Value;
use super::instruction::Instruction;

use std::collections::HashMap;
use ocean_macros::make_add_numbers;

#[derive(Clone)]
pub struct ExecutionContext {
  pub parent_execution_context: Option<Box<ExecutionContext>>,
  pub stack: Vec<Value>,
  pub program_counter: usize,
  pub instructions: Vec<Instruction>,
  pub variables: HashMap<String, Value>,
  pub return_value: Option<Value>,
}

impl ExecutionContext {
  pub fn resolve(&self, value: Value) -> Value {
    match value {
      Value::Reference(x) => todo!("resolve ref values"),
      _ => value.clone(),
    }
  }

  pub fn bool(&self, value: Value) -> bool {
    match self.resolve(value) {
      Value::Boolean(x) => x,
      _ => panic!("Bool does not make sense for this Value type :("),
    }
  }

  pub fn add(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (Value::Character(a), Value::Character(b)) => Value::String(a.to_string() + b.to_string().as_str()),
      (Value::Character(a), Value::String(b)) => Value::String(a.to_string() + b.as_str()),
      (Value::String(a), Value::Character(b)) => Value::String(a + b.to_string().as_str()),
      (Value::String(a), Value::String(b)) => Value::String(a + b.as_str()), // there has to be a better way than '.as_str()'
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn sub(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn mult(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn div(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn modulo(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn shiftleft(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

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

  pub fn shiftright(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

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

  pub fn bitand(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitor(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitxor(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn bitnot(&self, a: Value) -> Value {
    let a_value = self.resolve(a);

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

  pub fn and(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a && b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn or(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a || b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn xor(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a != b),
        _ => panic!("Operator unimplemented for type"),
      }
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn not(&self, a: Value) -> Value {
    let a_value = self.resolve(a);

    match a_value {
      Value::Boolean(a) => Value::Boolean(!a),
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn equal(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

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

  pub fn notequal(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a != b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() != b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a != b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a != b),
      (a, b) => make_add_numbers!(a, b),
    }

  }

  pub fn lessthan(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a < b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a < b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() < b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a < b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a < b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn greaterthan(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a > b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a > b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() > b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a > b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a > b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn lessthanequal(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a <= b),
      (Value::Character(a), Value::Character(b)) => Value::Boolean(a <= b),
      (Value::Character(a), Value::String(b)) => Value::Boolean(a.to_string() <= b),
      (Value::String(a), Value::Character(b)) => Value::Boolean(a <= b.to_string()),
      (Value::String(a), Value::String(b)) => Value::Boolean(a <= b),
      (a, b) => make_add_numbers!(a, b),
    }
  }

  pub fn greaterthanequal(&self, a: Value, b: Value) -> Value {
    let a_value = self.resolve(a);
    let b_value = self.resolve(b);

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