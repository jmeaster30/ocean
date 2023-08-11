use super::{executioncontext::ExecutionContext, instruction::*, value::Value};

use crate::hydro::module::Module;
use crate::hydro::value::{IndexRef, Reference};

pub trait Executable {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> bool;
}

impl Instruction {
  pub fn execute(&self, module: &Module, context: &mut ExecutionContext) -> bool {
    match self {
      Instruction::PushValue(x) => x.execute(module, context),
      Instruction::PopValue(x) => x.execute(module, context),
      Instruction::Add(x) => x.execute(module, context),
      Instruction::Subtract(x) => x.execute(module, context),
      Instruction::Multiply(x) => x.execute(module, context),
      Instruction::Divide(x) => x.execute(module, context),
      Instruction::Modulo(x) => x.execute(module, context),
      Instruction::LeftShift(x) => x.execute(module, context),
      Instruction::RightShift(x) => x.execute(module, context),
      Instruction::BitwiseAnd(x) => x.execute(module, context),
      Instruction::BitwiseOr(x) => x.execute(module, context),
      Instruction::BitwiseXor(x) => x.execute(module, context),
      Instruction::BitwiseNot(x) => x.execute(module, context),
      Instruction::And(x) => x.execute(module, context),
      Instruction::Or(x) => x.execute(module, context),
      Instruction::Xor(x) => x.execute(module, context),
      Instruction::Not(x) => x.execute(module, context),
      Instruction::Equal(x) => x.execute(module, context),
      Instruction::NotEqual(x) => x.execute(module, context),
      Instruction::LessThan(x) => x.execute(module, context),
      Instruction::GreaterThan(x) => x.execute(module, context),
      Instruction::LessThanEqual(x) => x.execute(module, context),
      Instruction::GreaterThanEqual(x) => x.execute(module, context),
      Instruction::Jump(x) => x.execute(module, context),
      Instruction::Branch(x) => x.execute(module, context),
      Instruction::Call(x) => x.execute(module, context),
      Instruction::Return(x) => x.execute(module, context),
      Instruction::Load(x) => x.execute(module, context),
      Instruction::Store(x) => x.execute(module, context),
      Instruction::AllocArray(x) => x.execute(module, context),
      Instruction::AllocMap(x) => x.execute(module, context),
      Instruction::Index(x) => x.execute(module, context),
    }
  }
}

impl Executable for PushValue {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    context.stack.push(self.value.clone());
    context.program_counter += 1;
    true
  }
}

impl Executable for PopValue {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.pop().is_none() {
      todo!("Create exceptions and throw an exception here");
    }
    context.program_counter += 1;
    true
  }
}

impl Executable for Add {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.add(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Subtract {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.sub(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Multiply {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.mult(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Divide {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.div(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Modulo {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.modulo(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LeftShift {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftleft(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for RightShift {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftright(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseAnd {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitand(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseOr {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseXor {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitxor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for BitwiseNot {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      todo!("Create exceptions and throw an exception here");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitnot(a));

    context.program_counter += 1;
    true
  }
}

impl Executable for And {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.and(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Or {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.or(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Xor {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.xor(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Not {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.not(a));

    context.program_counter += 1;
    true
  }
}

impl Executable for Equal {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.equal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for NotEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.notequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LessThan {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthan(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for GreaterThan {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthan(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for LessThanEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthanequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for GreaterThanEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthanequal(a, b));

    context.program_counter += 1;
    true
  }
}

impl Executable for Jump {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    context.program_counter = self.index;
    true
  }
}

impl Executable for Branch {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      todo!("Create exceptions and throw an exception here");
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
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      todo!("Create exceptions and throw an exception here")
    }

    // make call and loop through execution context
    let func = context.stack.pop().unwrap();
    match func {
      Value::FunctionPointer(func_pointer) => {
        let target_module = match func_pointer.module {
          Some(module_name) => match module.modules.get(&module_name) {
            Some(modu) => modu,
            None => todo!("Create exceptions and throw an exception here"),
          },
          None => module
        };

        let mut arguments = Vec::new();
        let target_function = target_module.functions.get(func_pointer.function.clone().as_str()).unwrap();
        for param in &target_function.parameters {
          let param_value = context.stack.pop().unwrap();
          arguments.push((param.clone(), param_value));
        }

        let return_value = target_module.execute(func_pointer.function, arguments, Some(Box::new(context.clone())));

        if return_value.is_some() {
          context.stack.push(return_value.unwrap());
        }
      }
      _ => todo!("Create exceptions and throw an exception here")
    }

    context.program_counter += 1;
    true
  }
}

impl Executable for Return {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      todo!("Create exceptions and throw an exception here");
    }

    let result = context.stack.pop().unwrap();
    context.return_value = Some(result);

    context.program_counter += 1;
    false
  }
}

impl Executable for Load {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 1 {
      todo!("Create exceptions and throw an exception here");
    }

    let reference = context.stack.pop().unwrap();
    let result = context.resolve(reference);
    context.stack.push(result);

    context.program_counter += 1;
    true
  }
}

impl Executable for Store {
  fn execute(&self, _module: &Module, _context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

impl Executable for Index {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> bool {
    if context.stack.len() < 2 {
      todo!("Create exceptions and throw an exception here");
    }

    let index = context.stack.pop().unwrap();
    let reference = context.stack.pop().unwrap();
    context.stack.push(Value::Reference(Reference::Index(
      IndexRef::new(Box::new(reference), Box::new(index))
    )));

    context.program_counter += 1;
    true
  }
}

impl Executable for AllocArray {
  fn execute(&self, _module: &Module, _context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

impl Executable for AllocMap {
  fn execute(&self, _module: &Module, _context: &mut ExecutionContext) -> bool {
    todo!();
  }
}

