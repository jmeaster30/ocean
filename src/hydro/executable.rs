use super::{executioncontext::ExecutionContext, instruction::*, value::Value};

use std::collections::HashMap;

pub fn execute(instructions: &Vec<Instruction>, args: Vec<(String, Value)>, parent_context: Option<Box<ExecutionContext>>) -> Option<Value> {
  let mut context = ExecutionContext {
    parent_execution_context: parent_context,
    stack: Vec::new(),
    program_counter: 0,
    instructions: instructions.to_vec(),
    variables: HashMap::new(),
    return_value: None,
  };

  for arg in args {
    context.variables.insert(arg.0, arg.1);
  }
  
  while context.program_counter.clone() < instructions.len() {
    let inst = context.instructions[context.program_counter.clone()].clone();
    let cont = inst.execute(&mut context);
    if !cont {
      break;
    }
  }

  return context.return_value.clone();
}

pub trait Executable {
  fn execute(&self, context: &mut ExecutionContext) -> bool;
}

impl Instruction {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    match self {
      Instruction::PushValue(x) => x.execute(context),
      Instruction::PopValue(x) => x.execute(context),
      Instruction::Add(x) => x.execute(context),
      Instruction::Subtract(x) => x.execute(context),
      Instruction::Multiply(x) => x.execute(context),
      Instruction::Divide(x) => x.execute(context),
      Instruction::Modulo(x) => x.execute(context),
      Instruction::LeftShift(x) => x.execute(context),
      Instruction::RightShift(x) => x.execute(context),
      Instruction::BitwiseAnd(x) => x.execute(context),
      Instruction::BitwiseOr(x) => x.execute(context),
      Instruction::BitwiseXor(x) => x.execute(context),
      Instruction::BitwiseNot(x) => x.execute(context),
      Instruction::And(x) => x.execute(context),
      Instruction::Or(x) => x.execute(context),
      Instruction::Xor(x) => x.execute(context),
      Instruction::Not(x) => x.execute(context),
      Instruction::Equal(x) => x.execute(context),
      Instruction::NotEqual(x) => x.execute(context),
      Instruction::LessThan(x) => x.execute(context),
      Instruction::GreaterThan(x) => x.execute(context),
      Instruction::LessThanEqual(x) => x.execute(context),
      Instruction::GreaterThanEqual(x) => x.execute(context),
      Instruction::Jump(x) => x.execute(context),
      Instruction::Branch(x) => x.execute(context),
      Instruction::Call(x) => x.execute(context),
      Instruction::Return(x) => x.execute(context),
      Instruction::LoadVariable(x) => x.execute(context),
      Instruction::StoreVariable(x) => x.execute(context),
      Instruction::LoadIndex(x) => x.execute(context),
      Instruction::StoreIndex(x) => x.execute(context),
    }
  }
}

impl Executable for PushValue {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    context.stack.push(self.value.clone());
    context.program_counter += 1;
    true
  }
}

impl Executable for PopValue {
  fn execute(&self, mut context: &mut ExecutionContext) -> bool {
    if context.stack.pop().is_none() {
      panic!("Stack was empty when it was expected to have some value :(");
    }
    context.program_counter += 1;
    true
  }
}

impl Executable for Add {
  fn execute(&self, mut context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the add operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.add(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Subtract {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the sub operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.sub(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Multiply {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the mult operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.mult(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Divide {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the div operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.div(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Modulo {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the mod operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.modulo(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LeftShift {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the left shift operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftleft(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for RightShift {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the right shift operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftright(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseAnd {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit and operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitand(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseOr {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit or operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseXor {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the bit xor operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitxor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseNot {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the bit not operation :(");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitnot(a));

    context.program_counter += 1;
    true
  }
}

impl Executable for And {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the and operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.and(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Or {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the or operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.or(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Xor {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the xor operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.xor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Not {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the not operation :(");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.not(a));

    context.program_counter += 1;
    true
  }
}

impl Executable for Equal {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.equal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for NotEqual {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the not equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.notequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LessThan {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the less than operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthan(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for GreaterThan {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the greater than operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthan(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LessThanEqual {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the less than equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthanequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for GreaterThanEqual {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      panic!("Stack didn't have enough elements for the greater than equal operation :(");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthanequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Jump {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    context.program_counter = self.index;
    true
  }
}

impl Executable for Branch {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the branch operation :(");
    }

    let a = context.stack.pop().unwrap();
    let result = context.bool(a);

    if result {
      context.program_counter = self.true_index;
    } else {
      context.program_counter = self.false_index;
    }
    true
  }
}

impl Executable for Call {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the call operation :(")
    }

    // make call and loop through execution context
    let func = context.stack.pop().unwrap();
    let func_value = context.resolve(func);
    match func_value {
      Value::Function(func) => {
        let mut arguments = Vec::new();
        for param in func.parameters {
          let param_value = context.stack.pop().unwrap();
          arguments.push((param, param_value));
        }

        let return_value = execute(&func.body, arguments, Some(Box::new(context.clone())));

        if return_value.is_some() {
          context.stack.push(return_value.unwrap());
        }
      }
      _ => panic!("Value resolved to a type that was not a function :(")
    }

    context.program_counter += 1;
    true
  }
}

impl Executable for Return {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      panic!("Stack didn't have enough elements for the branch operation :(");
    }

    let result = context.stack.pop().unwrap();
    context.return_value = Some(result);

    context.program_counter += 1;
    false
  }
}

impl Executable for LoadVariable { 
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

impl Executable for StoreVariable {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

impl Executable for LoadIndex {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

impl Executable for StoreIndex {
  fn execute(&self, context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

