use crate::hydro::debugcontext::DebugContext;
use crate::hydro::exception::Exception;
use crate::hydro::executable::Executable;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::instruction::*;
use crate::hydro::module::Module;
use crate::hydro::value::Value;

pub trait Debuggable {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception>;
}

impl Instruction {
  pub fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    match self {
      Instruction::PushValue(x) => x.debug(module, context, debug_context),
      Instruction::PopValue(x) => x.debug(module, context, debug_context),
      Instruction::Add(x) => x.debug(module, context, debug_context),
      Instruction::Subtract(x) => x.debug(module, context, debug_context),
      Instruction::Multiply(x) => x.debug(module, context, debug_context),
      Instruction::Divide(x) => x.debug(module, context, debug_context),
      Instruction::Modulo(x) => x.debug(module, context, debug_context),
      Instruction::LeftShift(x) => x.debug(module, context, debug_context),
      Instruction::RightShift(x) => x.debug(module, context, debug_context),
      Instruction::BitwiseAnd(x) => x.debug(module, context, debug_context),
      Instruction::BitwiseOr(x) => x.debug(module, context, debug_context),
      Instruction::BitwiseXor(x) => x.debug(module, context, debug_context),
      Instruction::BitwiseNot(x) => x.debug(module, context, debug_context),
      Instruction::And(x) => x.debug(module, context, debug_context),
      Instruction::Or(x) => x.debug(module, context, debug_context),
      Instruction::Xor(x) => x.debug(module, context, debug_context),
      Instruction::Not(x) => x.debug(module, context, debug_context),
      Instruction::Equal(x) => x.debug(module, context, debug_context),
      Instruction::NotEqual(x) => x.debug(module, context, debug_context),
      Instruction::LessThan(x) => x.debug(module, context, debug_context),
      Instruction::GreaterThan(x) => x.debug(module, context, debug_context),
      Instruction::LessThanEqual(x) => x.debug(module, context, debug_context),
      Instruction::GreaterThanEqual(x) => x.debug(module, context, debug_context),
      Instruction::Jump(x) => x.debug(module, context, debug_context),
      Instruction::Branch(x) => x.debug(module, context, debug_context),
      Instruction::Call(x) => x.debug(module, context, debug_context),
      Instruction::Return(x) => x.debug(module, context, debug_context),
      Instruction::Load(x) => x.debug(module, context, debug_context),
      Instruction::Store(x) => x.debug(module, context, debug_context),
      Instruction::ArrayIndex(x) => x.debug(module, context, debug_context),
      Instruction::LayoutIndex(x) => x.debug(module, context, debug_context),
      Instruction::AllocArray(x) => x.debug(module, context, debug_context),
      Instruction::AllocLayout(x) => x.debug(module, context, debug_context),
    }
  }
}

impl Debuggable for PushValue {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "push".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for PopValue {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "pop".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Add {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "add".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Subtract {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "subtract".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Multiply {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "multiply".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Divide {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "divide".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Modulo {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "modulo".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for LeftShift {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "shiftleft".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for RightShift {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "shiftright".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for BitwiseAnd {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "bitand".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for BitwiseOr {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "bitor".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for BitwiseXor {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "bitxor".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for BitwiseNot {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "bitnot".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for And {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "and".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Or {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "or".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Xor {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "xor".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}
impl Debuggable for Not {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "not".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}
impl Debuggable for Equal {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "equal".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for NotEqual {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "notequal".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for LessThan {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "lessthan".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for GreaterThan {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "greaterthan".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for LessThanEqual {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "lessthanequal".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for GreaterThanEqual {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "greaterthanequal".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Jump {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "jump".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Branch {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "branch".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Call {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "call".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );

    if context.stack.len() < 1 {
      debug_context.stop_core_metric(
        context.current_module.clone(),
        context.current_function.clone(),
        metric_name,
      );
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
              debug_context.stop_core_metric(
                context.current_module.clone(),
                context.current_function.clone(),
                metric_name,
              );
              return Err(Exception::new(
                context.clone(),
                format!("Could not find module '{}'", module_name).as_str(),
              ));
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
            debug_context.stop_core_metric(
              context.current_module.clone(),
              context.current_function.clone(),
              metric_name,
            );
            return Err(Exception::new(
              context.clone(),
              format!(
                "Could not find function '{}' in module '{}'",
                func_pointer.function.clone().as_str(),
                target_module.name
              )
              .as_str(),
            ));
          }
        };

        if context.stack.len() < target_function.parameters.len() {
          debug_context.stop_core_metric(
            context.current_module.clone(),
            context.current_function.clone(),
            metric_name,
          );
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

        let return_value = target_module.debug(
          func_pointer.function,
          arguments,
          Some(Box::new(context.clone())),
          debug_context,
        );
        match return_value {
          Ok(optional_return) => match optional_return {
            Some(value) => context.stack.push(value),
            None => {}
          },
          Err(e) => {
            debug_context.stop_core_metric(
              context.current_module.clone(),
              context.current_function.clone(),
              metric_name,
            );
            return Err(e);
          }
        }
      }
      _ => {
        debug_context.stop_core_metric(
          context.current_module.clone(),
          context.current_function.clone(),
          metric_name,
        );
        return Err(Exception::new(
          context.clone(),
          "Non-invokable value was attempted to be invoked",
        ));
      }
    }

    context.program_counter += 1;
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    Ok(true)
  }
}

impl Debuggable for Return {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "return".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Load {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "load".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for Store {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "store".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for ArrayIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "array_index".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for LayoutIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "layout_index".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for AllocArray {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "allocarray".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}

impl Debuggable for AllocLayout {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "alloclayout".to_string();
    debug_context.start_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    );
    let result = self.execute(module, context);
    debug_context.stop_core_metric(
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name,
    );
    return result;
  }
}
