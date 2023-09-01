use super::{executioncontext::ExecutionContext, instruction::*, value::Value};
use crate::hydro::exception::Exception;

use crate::hydro::module::Module;
use crate::hydro::value::{ArrayIndexRef, LayoutIndexRef, Reference};

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
      Instruction::AllocLayout(x) => x.execute(module, context),
      Instruction::ArrayIndex(x) => x.execute(module, context),
      Instruction::LayoutIndex(x) => x.execute(module, context),
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
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    context.program_counter = self.index;
    Ok(true)
  }
}

impl Executable for Branch {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got none.",
      ));
    }

    let a = context.stack.pop().unwrap();
    let result = context.bool(a);

    if result {
      context.program_counter = self.true_index;
    } else {
      context.program_counter = self.false_index;
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
            None => {
              return Err(Exception::new(
                context.clone(),
                format!("Could not find module '{}'", module_name).as_str(),
              ))
            }
          },
          None => module,
        };

        let mut arguments = Vec::new();
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

        for param in &target_function.parameters {
          let param_value = context.stack.pop().unwrap();
          arguments.push((param.clone(), param_value));
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

impl Executable for ArrayIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 2 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let index = context.stack.pop().unwrap();
    let reference = context.stack.pop().unwrap();
    context
      .stack
      .push(Value::Reference(Reference::ArrayIndex(ArrayIndexRef::new(
        Box::new(reference),
        Box::new(index),
      ))));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for LayoutIndex {
  fn execute(&self, _module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 1 and got 0.",
      ));
    }

    let reference = context.stack.pop().unwrap();
    context.stack.push(Value::Reference(Reference::LayoutIndex(
      LayoutIndexRef::new(Box::new(reference), self.member.clone()),
    )));

    context.program_counter += 1;
    Ok(true)
  }
}

impl Executable for AllocArray {
  fn execute(&self, _module: &Module, _context: &mut ExecutionContext) -> Result<bool, Exception> {
    todo!();
  }
}

impl Executable for AllocLayout {
  fn execute(&self, module: &Module, context: &mut ExecutionContext) -> Result<bool, Exception> {
    if context.stack.len() < 1 {
      return Err(Exception::new(
        context.clone(),
        "Unexpected number of stack values. Expected 2 and got 1.",
      ));
    }

    let layout_template = match self.module_name.clone() {
      Some(template_module_name) => match module.modules.get(template_module_name.as_str()) {
        Some(template_module) => match template_module
          .layout_templates
          .get(self.layout_template_name.as_str())
        {
          Some(template) => template,
          None => {
            return Err(Exception::new(
              context.clone(),
              format!(
                "Layout '{}' not found in module '{}'",
                self.layout_template_name, template_module_name
              )
              .as_str(),
            ))
          }
        },
        None => {
          return Err(Exception::new(
            context.clone(),
            format!("Module '{}' not found.", template_module_name).as_str(),
          ))
        }
      },
      None => match module
        .layout_templates
        .get(self.layout_template_name.as_str())
      {
        Some(template) => template,
        None => {
          return Err(Exception::new(
            context.clone(),
            format!(
              "Layout '{}' not found in module '{}'",
              self.layout_template_name, module.name
            )
            .as_str(),
          ))
        }
      },
    };

    let allocated = layout_template.create_value();
    let value_reference = context.stack.pop().unwrap();
    match &value_reference {
      Value::Reference(reference) => context.modify(reference, allocated)?,
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
