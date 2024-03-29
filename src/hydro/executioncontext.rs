use super::value::Value;

use crate::hydro::exception::Exception;
use crate::hydro::value::Reference;
use ocean_macros::{make_add_operations, make_bit_operations, make_comparison_operations};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionContext {
  pub parent_execution_context: Option<Box<ExecutionContext>>,
  pub stack: Vec<Value>,
  pub program_counter: usize,
  pub variables: HashMap<String, Value>,
  pub return_value: Option<Value>,
  pub current_function: String,
  pub current_module: String,
}

impl ExecutionContext {
  pub fn copy(&self) -> Self {
    Self {
      parent_execution_context: self.parent_execution_context.clone(),
      stack: self.stack.clone(),
      program_counter: self.program_counter,
      variables: self.variables.clone(),
      return_value: self.return_value.clone(),
      current_function: self.current_function.clone(),
      current_module: self.current_module.clone(),
    }
  }

  pub fn get_call_stack(&self) -> Vec<String> {
    let mut results = Vec::new();
    match &self.parent_execution_context {
      Some(next_context) => {
        let mut call_stack = next_context.get_call_stack();
        results.append(&mut call_stack);
      }
      None => {}
    }
    results.push(format!("{}.{}", self.current_module, self.current_function));
    results
  }

  pub fn print_stacktrace(&self) {
    println!("\tModule: '{}' Function: '{}' at PC: {}", self.current_module, self.current_function, self.program_counter);
    match &self.parent_execution_context {
      Some(next_context) => next_context.print_stacktrace(),
      None => {}
    }
  }

  pub fn resolve(&self, value: Value) -> Result<Value, Exception> {
    match value {
      Value::Reference(base_reference) => match base_reference {
        Reference::Variable(variable_reference) => match self.variables.get(variable_reference.name.clone().as_str()) {
          Some(found_variable) => Ok(found_variable.clone()),
          None => Err(Exception::new(self.clone(), format!("Could not find variable of name '{}'", variable_reference.name).as_str())),
        },
        Reference::ArrayIndex(index_reference) => {
          let resolved = self.resolve(index_reference.reference.deref().clone())?;
          match (index_reference.index.deref(), resolved) {
            (Value::Signed8(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Signed16(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Signed32(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Signed64(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Signed128(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Unsigned8(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Unsigned16(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Unsigned32(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Unsigned64(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            (Value::Unsigned128(x), Value::Array(array)) => Ok(array.values[*x as usize].clone()),
            _ => Err(Exception::new(self.clone(), "Value could not be indexed by the specified index")),
          }
        }
        Reference::LayoutIndex(layout_reference) => {
          let resolved = self.resolve(layout_reference.reference.deref().clone())?;
          match (layout_reference.index, resolved) {
            (x, Value::Layout(layout)) => match layout.values.get(x.as_str()) {
              Some(found_result) => Ok(found_result.clone()),
              None => Err(Exception::new(self.clone(), format!("Could not find entry '{}' in layout.", x).as_str())),
            },
            _ => Err(Exception::new(self.clone(), "Value could not be indexed by the specified index")),
          }
        }
      },
      _ => Ok(value.clone()),
    }
  }

  fn get_value_reference(&mut self, reference_value: &Value) -> Result<&mut Value, Exception> {
    let context_copy = self.copy();
    match reference_value {
      Value::Reference(reference) => match reference {
        Reference::Variable(variable_reference) => match self.variables.get_mut(variable_reference.name.clone().as_str()) {
          Some(mutable_value) => Ok(mutable_value),
          None => Err(Exception::new(context_copy, format!("Could not find variable '{}'", variable_reference.name.clone()).as_str())),
        },
        Reference::ArrayIndex(index_reference) => {
          let mutable_value = self.get_value_reference(index_reference.reference.deref())?;
          match (index_reference.index.deref().clone(), mutable_value) {
            (Value::Signed8(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Signed16(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Signed32(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Signed64(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Signed128(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Unsigned8(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Unsigned16(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Unsigned32(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Unsigned64(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            (Value::Unsigned128(x), Value::Array(array)) => Ok(&mut array.values[x as usize]),
            _ => return Err(Exception::new(context_copy, "Value could not be indexed by the specified index")),
          }
        }
        Reference::LayoutIndex(layout_index_ref) => {
          let mutable_value = self.get_value_reference(layout_index_ref.reference.deref())?;
          match (layout_index_ref.index.clone(), mutable_value) {
            (x, Value::Layout(layout)) => match layout.values.get_mut(x.as_str()) {
              Some(mutable_layout_member) => Ok(mutable_layout_member),
              None => return Err(Exception::new(context_copy, format!("Could not find entry '{}' in layout.", x).as_str())),
            },
            _ => return Err(Exception::new(context_copy, "Value could not be indexed by the specified index")),
          }
        }
      },
      _ => Err(Exception::new(context_copy, "Cannot get mutable reference to non-reference value")),
    }
  }

  pub fn init(&mut self, reference: &Reference, value: Value) -> Result<(), Exception> {
    match reference {
      Reference::ArrayIndex(_) => Err(Exception::new(self.clone(), "Initializing memory with non-variable reference doesn't make sense")),
      Reference::LayoutIndex(_) => Err(Exception::new(self.clone(), "Initializing memory with non-variable reference doesn't make sense")),
      Reference::Variable(variable_reference) => match self.variables.get(&variable_reference.name.clone()) {
        Some(_) => Err(Exception::new(self.clone(), format!("Variable '{}' already exists :(", variable_reference.name.clone()).as_str())),
        None => {
          self.variables.insert(variable_reference.name.clone(), value.clone());
          Ok(())
        }
      },
    }
  }

  pub fn modify(&mut self, reference: &Reference, value: Value) -> Result<(), Exception> {
    match reference {
      Reference::ArrayIndex(index_reference) => {
        let value_reference = self.get_value_reference(index_reference.reference.deref())?;
        match (index_reference.index.deref().clone(), value_reference) {
          (Value::Signed8(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Signed16(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Signed32(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Signed64(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Signed128(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Unsigned8(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Unsigned16(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Unsigned32(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Unsigned64(x), Value::Array(array)) => array.values[x as usize] = value,
          (Value::Unsigned128(x), Value::Array(array)) => array.values[x as usize] = value,
          _ => return Err(Exception::new(self.clone(), "Value could not be indexed by the specified index")),
        }
      }
      Reference::LayoutIndex(layout_reference) => {
        let value_reference = self.get_value_reference(layout_reference.reference.deref())?;
        match (layout_reference.index.clone(), value_reference) {
          (x, Value::Layout(layout)) => match layout.values.get(x.as_str()) {
            Some(_) => {
              layout.values.insert(x, value);
            }
            None => return Err(Exception::new(self.clone(), format!("Could not find entry '{}' in layout.", x).as_str())),
          },
          _ => return Err(Exception::new(self.clone(), "Value could not be indexed by the specified index")),
        }
      }
      Reference::Variable(variable_reference) => match self.variables.get(&variable_reference.name.clone()) {
        Some(_) => {
          self.variables.insert(variable_reference.name.clone(), value.clone());
        }
        None => return Err(Exception::new(self.clone(), format!("Variable '{}' does not exist :(", variable_reference.name.clone()).as_str())),
      },
    }
    Ok(())
  }

  pub fn add(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_add_operations!(+),
    }
  }

  pub fn sub(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_add_operations!(-),
    }
  }

  pub fn mult(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_add_operations!(*),
    }
  }

  pub fn div(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_add_operations!(/),
    }
  }

  pub fn modulo(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_add_operations!(%),
    }
  }

  pub fn shiftleft(&self, a_value: Value, b_value: Value) -> Value {
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

  pub fn shiftright(&self, a_value: Value, b_value: Value) -> Value {
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

  pub fn bitand(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_bit_operations!(&),
    }
  }

  pub fn bitor(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_bit_operations!(|),
    }
  }

  pub fn bitxor(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (a, b) => make_bit_operations!(^),
    }
  }

  pub fn bitnot(&self, a_value: Value) -> Value {
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

  pub fn and(&self, a_value: Value, b_value: Value) -> Value {
    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a && b),
        _ => panic!("Operator unimplemented for type"),
      },
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn or(&self, a_value: Value, b_value: Value) -> Value {
    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a || b),
        _ => panic!("Operator unimplemented for type"),
      },
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn xor(&self, a_value: Value, b_value: Value) -> Value {
    match a_value {
      Value::Boolean(a) => match b_value {
        Value::Boolean(b) => Value::Boolean(a != b),
        _ => panic!("Operator unimplemented for type"),
      },
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn not(&self, a_value: Value) -> Value {
    match a_value {
      Value::Boolean(a) => Value::Boolean(!a),
      _ => panic!("Operator unimplemented for type"),
    }
  }

  pub fn equal(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a == b),
      (Value::Array(a), Value::Array(b)) => {
        if a.value_type == b.value_type && a.length == b.length {
          let mut result = true;
          for (left, right) in a.values.iter().zip(b.values.iter()) {
            if !self.equal(left.clone(), right.clone()).to_bool().unwrap() {
              result = false;
              break;
            }
          }
          Value::Boolean(result)
        } else {
          Value::Boolean(false)
        }
      }
      (a, b) => make_comparison_operations!(==),
    }
  }

  pub fn notequal(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a != b),
      (a, b) => make_comparison_operations!(!=),
    }
  }

  pub fn lessthan(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a < b),
      (a, b) => make_comparison_operations!(<),
    }
  }

  pub fn greaterthan(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a > b),
      (a, b) => make_comparison_operations!(>),
    }
  }

  pub fn lessthanequal(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a <= b),
      (a, b) => make_comparison_operations!(<=),
    }
  }

  pub fn greaterthanequal(&self, a_value: Value, b_value: Value) -> Value {
    match (a_value, b_value) {
      (Value::Boolean(a), Value::Boolean(b)) => Value::Boolean(a >= b),
      (a, b) => make_comparison_operations!(>=),
    }
  }
}
