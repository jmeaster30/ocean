use super::{executioncontext::ExecutionContext, instruction::*, value::Value};
use crate::hydro::exception::Exception;

use crate::hydro::module::Module;

pub trait Executable {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception>;
}

impl Instruction {
  pub fn execute(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
  ) -> Result<bool, Exception> {
    match self {
      Instruction::PushValue(x) => x.execute(module, context),
      Instruction::PopValue(x) => x.execute(module, context),
      Instruction::Duplicate(x) => x.execute(module, context),
      Instruction::Swap(x) => x.execute(module, context),
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
      Instruction::Allocate(x) => x.execute(module, context),
      Instruction::GetArrayIndex(x) => x.execute(module, context),
      Instruction::SetArrayIndex(x) => x.execute(module, context),
      Instruction::GetLayoutIndex(x) => x.execute(module, context),
      Instruction::SetLayoutIndex(x) => x.execute(module, context),
    }
  }
}

impl Executable for PushValue {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    context.stack.push(self.value.clone());
    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for PopValue {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.pop().is_none() {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }
    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Duplicate {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 but got 0.",
      ))
    }

    let value = context.stack.pop().unwrap();
    context.stack.push(value.clone());
    context.stack.push(value.clone());

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Swap {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 but got 1.",
      ))
    }

    let a = context.stack.pop().unwrap();
    let b = context.stack.pop().unwrap();
    context.stack.push(a.clone());
    context.stack.push(b.clone());

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Add {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.add(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Subtract {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.sub(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Multiply {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.mult(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Divide {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.div(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Modulo {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.modulo(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for LeftShift {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftleft(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for RightShift {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.shiftright(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for BitwiseAnd {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitand(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for BitwiseOr {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitor(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for BitwiseXor {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitxor(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for BitwiseNot {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.bitnot(a));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for And {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.and(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Or {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.or(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Xor {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.xor(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Not {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let a = context.stack.pop().unwrap();

    context.stack.push(context.not(a));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Equal {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.equal(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for NotEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.notequal(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for LessThan {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthan(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for GreaterThan {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthan(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for LessThanEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.lessthanequal(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for GreaterThanEqual {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let b = context.stack.pop().unwrap();
    let a = context.stack.pop().unwrap();

    context.stack.push(context.greaterthanequal(a, b));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Jump {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    let current_function = module.functions.get(context.current_function.as_str()).unwrap();
    match current_function.get_target_pointer(self.target.clone()) {
      Ok(index) => context.program_counter = index,
      Err(message ) => return Err(Exception::new(context.clone(), message.as_str()))
    }
    Ok(true)
  }
}

impl Executable for Branch {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    let a = context.stack.pop().unwrap();
    let result = context.bool(a);

    let current_function = module.functions.get(context.current_function.as_str()).unwrap();

    if result {
      match current_function.get_target_pointer(self.true_target.clone()) {
        Ok(index) => context.program_counter = index,
        Err(message ) => return Err(Exception::new(context.clone(), message.as_str()))
      }
    } else {
      match current_function.get_target_pointer(self.false_target.clone()) {
        Ok(index) => context.program_counter = index,
        Err(message ) => return Err(Exception::new(context.clone(), message.as_str()))
      }
    }
    Ok(true)
  }
}

impl Executable for Call {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    // make call and loop through execution context
    let func = context.stack.pop().unwrap();
    match func {
      Value::FunctionPointer(func_pointer) => {
        let target_module = match func_pointer.module {
          Some(module_name) => match module.modules.get(&module_name) {
            Some(modu) => modu,
            None => if module.name == module_name {
              module
            } else {
              return Err(Exception::new(
                context.clone(),
                format!("Could not find module '{}'", module_name).as_str(),
              ))
            }
          },
          None => module,
        };

        let target_function = match target_module
          .functions
          .get(func_pointer.function.clone().as_str())
        {
          Some(func) => func,
          None => {
            return Err(Exception::new(
              context.clone(),
              format!(
                "Could not find function '{}' in module '{}'",
                func_pointer.function.clone().as_str(),
                target_module.name
              )
              .as_str(),
            ))
          }
        };

        if context.stack.len() < target_function.parameters.len() {
          return Err(Exception::new(
            context.clone(),
            format!(
              "Unexpected number of stack values. Expected {} and got {}.",
              target_function.parameters.len(),
              context.stack.len()
            )
            .as_str(),
          ));
        }

        let mut arguments = Vec::new();
        for _ in &target_function.parameters {
          let param_value = context.stack.pop().unwrap();
          arguments.insert(0, param_value);
        }

        let return_value = target_module.execute(
          func_pointer.function,
          arguments,
          Some(Box::new(context.clone())),
        );
        match return_value {
          Ok(optional_return) => match optional_return {
            Some(value) => context.stack.push(value),
            None => {}
          },
          Err(e) => return Err(e),
        }
      }
      _ => {
        return Err(Exception::new(
          context.clone(),
          "Non-invokable value was attempted to be invoked",
        ))
      }
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Return {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    let result = context.stack.pop().unwrap();
    context.return_value = Some(result);

    context.program_counter += 1;
    Ok(false)
  }
}

impl Executable for Load {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    let reference = context.stack.pop().unwrap();
    let result = context.resolve(reference);
    match result {
      Ok(value) => context.stack.push(value),
      Err(e) => return Err(e),
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Store {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let value = context.stack.pop().unwrap();
    let reference_value = context.stack.pop().unwrap();

    match &reference_value {
      Value::Reference(reference) => context.modify(reference, value)?,
      _ => {
        return Err(Exception::new(
          context.clone(),
          "Cannot store value into non-reference value",
        ))
      }
    }

    context.stack.push(reference_value.clone());

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for GetArrayIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let index = context.stack.pop().unwrap();
    let array = context.stack.pop().unwrap();

    match index.to_u64() {
      Ok(result) => match array.index(result) {
        Ok(value_from_array) => {
          context.stack.push(array);
          context.stack.push(value_from_array);
        }
        Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
      }
      Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for SetArrayIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 3 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 3 and got less.",
      ));
    }

    let value = context.stack.pop().unwrap();
    let index = context.stack.pop().unwrap();
    let mut array = context.stack.pop().unwrap();

    match index.to_u64() {
      Ok(result) => match array.set_index(result, value) {
        Ok(()) => context.stack.push(array),
        Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
      }
      Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for GetLayoutIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got 0.",
      ));
    }

    let layout = context.stack.pop().unwrap();

    match layout.get_member(self.member.clone()) {
      Ok(result) => {
        context.stack.push(layout);
        context.stack.push(result);
      },
      Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for SetLayoutIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got less.",
      ));
    }

    let value = context.stack.pop().unwrap();
    let mut layout = context.stack.pop().unwrap();

    match layout.set_member(self.member.clone(), value) {
      Ok(_) => context.stack.push(layout),
      Err(message) => return Err(Exception::new(context.clone(), message.as_str())),
    }

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for Allocate {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let allocated = module
      .resolve_type(self.allocated_type.clone(), context)?
      .default();
    let value_reference = context.stack.pop().unwrap();
    match &value_reference {
      Value::Reference(reference) => context.init(reference, allocated)?,
      _ => {
        return Err(Exception::new(
          context.clone(),
          "Could not allocate layout into a non-reference value",
        ))
      }
    }

    context.stack.push(value_reference.clone());

    context.program_counter += 1;
    Ok(true)
  }
}
