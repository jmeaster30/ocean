use super::{executioncontext::ExecutionContext, instruction::*, value::Value};

use std::collections::HashMap;

pub fn execute(instructions: &Vec<Instruction>, args: Vec<(String, Value)>, parent_context: Option<&ExecutionContext>) -> Option<Value> {
  let mut context = ExecutionContext {
    parent_execution_context: match parent_context {
      Some(x) => Some(Box::new(*x)),
      None => None,
    },
    stack: Vec::new(),
    program_counter: 0,
    instructions: instructions.to_vec(),
    variables: HashMap::new(),
    return_value: None,
  };

  for arg in args {
    context.variables.insert(arg.0, arg.1);
  }
  
  for inst in instructions {
    inst.execute(&mut context);

    if context.return_value.is_some() {
      break;
    }
  }

  return context.return_value;
}

pub trait Executable {
  fn execute(&self, context: &mut ExecutionContext);
}

impl Instruction {
  fn execute(&self, context: &mut ExecutionContext) {
    
  }
}

impl Executable for PushValue {
  fn execute(&self, context: &mut ExecutionContext) {
    context.stack.push(self.value);
    context.program_counter += 1;
  }
}

impl Executable for PopValue {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.pop().is_none() {
      panic!("Stack was empty when it was expected to have some value :(");
    }
    context.program_counter += 1;
  }
}

impl Executable for Add {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the add operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.add(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Subtract {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the sub operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.sub(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Multiply {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the mult operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.mult(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Divide {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the div operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.div(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Modulo {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the mod operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.modulo(b, context));

    context.program_counter += 1;
  }
}

impl Executable for LeftShift {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the left shift operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.shiftleft(b, context));

    context.program_counter += 1;
  }
}

impl Executable for RightShift {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the right shift operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.shiftright(b, context));

    context.program_counter += 1;
  }
}

impl Executable for BitwiseAnd {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit and operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.bitand(b, context));

    context.program_counter += 1;
  }
}

impl Executable for BitwiseOr {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit or operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.bitor(b, context));

    context.program_counter += 1;
  }
}

impl Executable for BitwiseXor {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit xor operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.bitxor(b, context));

    context.program_counter += 1;
  }
}

impl Executable for BitwiseNot {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the bit not operation :(");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(a.bitnot(context));

    context.program_counter += 1;
  }
}

impl Executable for And {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the and operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.and(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Or {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the or operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.or(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Xor {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the xor operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.xor(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Not {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the not operation :(");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(a.not(context));

    context.program_counter += 1;
  }
}

impl Executable for Equal {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.equal(b, context));

    context.program_counter += 1;
  }
}

impl Executable for NotEqual {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the not equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.notequal(b, context));

    context.program_counter += 1;
  }
}

impl Executable for LessThan {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the less than operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.lessthan(b, context));

    context.program_counter += 1;
  }
}

impl Executable for GreaterThan {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the greater than operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.greaterthan(b, context));

    context.program_counter += 1;
  }
}

impl Executable for LessThanEqual {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the less than equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.lessthanequal(b, context));

    context.program_counter += 1;
  }
}

impl Executable for GreaterThanEqual {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the greater than equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(a.greaterthanequal(b, context));

    context.program_counter += 1;
  }
}

impl Executable for Jump {
  fn execute(&self, context: &mut ExecutionContext) {
    context.program_counter = self.index;
  }
}

impl Executable for Branch {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the branch operation :(");
    }

    let a = context.stack.pop().unwrap().bool(context);

    if a {
      context.program_counter = self.true_index;
    } else {
      context.program_counter = self.false_index;
    }
  }
}

impl Executable for Call {
  fn execute(&self, context: &mut ExecutionContext) {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the call operation :(")
    }

    // make call and loop through execution context
    let func_value = context.stack.pop().unwrap().resolve(context);
    match func_value {
      Value::Function(func) => {
        let mut arguments = Vec::new();
        for param in func.parameters {
          let param_value = context.stack.pop().unwrap();
          arguments.push((param, param_value));
        }

        let return_value = execute(&func.body, arguments, Some(context));

        if return_value.is_some() {
          context.stack.push(return_value.unwrap());
        }
      }
      _ => panic!("Value resolved to a type that was not a function :(")
    }

    context.program_counter += 1;
  }
}

impl Executable for Return {
  fn execute(&self, context: &mut ExecutionContext) {

  }
}

impl Executable for LoadVariable { 
  fn execute(&self, context: &mut ExecutionContext) {

  }
}

impl Executable for StoreVariable {
  fn execute(&self, context: &mut ExecutionContext) {

  }
}

impl Executable for LoadIndex {
  fn execute(&self, context: &mut ExecutionContext) {

  }
}

impl Executable for StoreIndex {
  fn execute(&self, context: &mut ExecutionContext) {

  }
}

