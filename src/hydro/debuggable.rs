use crate::hydro::debugcontext::DebugContext;
use crate::hydro::exception::Exception;
use crate::hydro::executable::Executable;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::instruction::*;
use crate::hydro::intrinsic::intrinsicmanager::INTRINSIC_MANAGER;
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
      Instruction::Duplicate(x) => x.debug(module, context, debug_context),
      Instruction::Swap(x) => x.debug(module, context, debug_context),
      Instruction::Rotate(x) => x.debug(module, context, debug_context),
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
      Instruction::GetArrayIndex(x) => x.debug(module, context, debug_context),
      Instruction::SetArrayIndex(x) => x.debug(module, context, debug_context),
      Instruction::GetLayoutIndex(x) => x.debug(module, context, debug_context),
      Instruction::SetLayoutIndex(x) => x.debug(module, context, debug_context),
      Instruction::Allocate(x) => x.debug(module, context, debug_context),
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for Duplicate {
  fn debug(&self, module: &Module, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
    let metric_name = "duplicate".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for Swap {
  fn debug(&self, module: &Module, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
    let metric_name = "swap".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for Rotate {
  fn debug(&self, module: &Module, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
    let metric_name = "rotate".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));

    if context.stack.len() < 1 {
      debug_context.metric_tracker.stop(format!(
        "{}.{}.{}",
        context.current_module.clone(),
        context.current_function.clone(),
        metric_name.clone(),
      ));
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
              debug_context.metric_tracker.stop(format!(
                "{}.{}.{}",
                context.current_module.clone(),
                context.current_function.clone(),
                metric_name.clone(),
              ));
              return Err(Exception::new(
                context.clone(),
                format!("Could not find module '{}'", module_name).as_str(),
              ))
            }
          },
          None => module,
        };


        match target_module
          .functions
          .get(func_pointer.function.clone().as_str())
        {
          Some(target_function) => {
            if context.stack.len() < target_function.parameters.len() {
              debug_context.metric_tracker.stop(format!(
                "{}.{}.{}",
                context.current_module.clone(),
                context.current_function.clone(),
                metric_name.clone(),
              ));
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
                debug_context.metric_tracker.stop(format!(
                  "{}.{}.{}",
                  context.current_module.clone(),
                  context.current_function.clone(),
                  metric_name.clone(),
                ));
                return Err(e);
              }
            }
          },
          None => match target_module.intrinsics.get(func_pointer.function.clone().as_str()) {
            Some(target_intrinsic) => {
              if context.stack.len() < target_intrinsic.parameters.len() {
                debug_context.metric_tracker.stop(format!(
                  "{}.{}.{}",
                  context.current_module.clone(),
                  context.current_function.clone(),
                  metric_name.clone(),
                ));
                return Err(Exception::new(
                  context.clone(),
                  format!(
                    "Unexpected number of stack values. Expected {} and got {}.",
                    target_intrinsic.parameters.len(),
                    context.stack.len()
                  )
                    .as_str(),
                ));
              }

              let mut arguments = Vec::new();
              for _ in &target_intrinsic.parameters {
                let param_value = context.stack.pop().unwrap();
                arguments.insert(0, param_value);
              }

              let code = match target_intrinsic.get_intrinsic_code("vm".to_string()) {
                Ok(code) => code,
                Err(message) => {
                  debug_context.metric_tracker.stop(format!(
                    "{}.{}.{}",
                    context.current_module.clone(),
                    context.current_function.clone(),
                    metric_name.clone(),
                  ));
                  return Err(Exception::new(
                    context.clone(),
                    message.as_str(),
                  ));
                }
              };

              let return_value = INTRINSIC_MANAGER.call(code, context, arguments);
              match return_value {
                Ok(mut values) => context.stack.append(&mut values),
                Err(e) => {
                  debug_context.metric_tracker.stop(format!(
                    "{}.{}.{}",
                    context.current_module.clone(),
                    context.current_function.clone(),
                    metric_name.clone(),
                  ));
                  return Err(e);
                }
              }
            }
            None => {
              debug_context.metric_tracker.stop(format!(
                "{}.{}.{}",
                context.current_module.clone(),
                context.current_function.clone(),
                metric_name.clone(),
              ));
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
          }
        };


      }
      _ => {
        debug_context.metric_tracker.stop(format!(
          "{}.{}.{}",
          context.current_module.clone(),
          context.current_function.clone(),
          metric_name.clone(),
        ));
        return Err(Exception::new(
          context.clone(),
          "Non-invokable value was attempted to be invoked",
        ));
      }
    }

    context.program_counter += 1;
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
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
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for GetArrayIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "get_array_index".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for SetArrayIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "set_array_index".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for GetLayoutIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "get_layout_index".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for SetLayoutIndex {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "set_layout_index".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}

impl Debuggable for Allocate {
  fn debug(
    &self,
    module: &Module,
    context: &mut ExecutionContext,
    debug_context: &mut DebugContext,
  ) -> Result<bool, Exception> {
    let metric_name = "allocate".to_string();
    debug_context.metric_tracker.start(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    let result = self.execute(module, context);
    debug_context.metric_tracker.stop(format!(
      "{}.{}.{}",
      context.current_module.clone(),
      context.current_function.clone(),
      metric_name.clone(),
    ));
    return result;
  }
}
