use crate::hydro::compilationunit::CompilationUnit;
use crate::hydro::debugcontext::DebugContext;
use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::instruction::*;
use crate::hydro::intrinsic::intrinsicmanager::INTRINSIC_MANAGER;
use crate::hydro::value::Value;

pub trait Debuggable {
  fn debug(&self, compilation_unit: &CompilationUnit, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception>;
}

impl Instruction {
  pub fn debug(&self, compilation_unit: &CompilationUnit, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
    match self {
      Instruction::PushValue(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::PopValue(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Duplicate(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Swap(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Rotate(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Add(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Subtract(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Multiply(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Divide(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Modulo(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::LeftShift(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::RightShift(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::BitwiseAnd(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::BitwiseOr(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::BitwiseXor(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::BitwiseNot(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::And(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Or(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Xor(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Not(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Equal(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::NotEqual(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::LessThan(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::GreaterThan(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::LessThanEqual(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::GreaterThanEqual(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Jump(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Branch(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Call(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Return(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Load(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Store(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::GetArrayIndex(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::SetArrayIndex(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::GetLayoutIndex(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::SetLayoutIndex(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Allocate(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::AllocateArray(x) => x.debug(compilation_unit, context, debug_context),
      Instruction::Cast(x) => x.debug(compilation_unit, context, debug_context),
    }
  }
}

impl Debuggable for Call {
  fn debug(&self, compilation_unit: &CompilationUnit, context: &mut ExecutionContext, debug_context: &mut DebugContext) -> Result<bool, Exception> {
    let metric_name = "call".to_string();
    debug_context.metric_tracker.start(context.get_call_stack(), metric_name.clone());

    if context.stack.len() < 1 {
      debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
      return Err(Exception::new(context.clone(), "Unexpected number of stack values. Expected 1 and got none."));
    }

    // make call and loop through execution context
    let func = context.stack.pop().unwrap();
    match func {
      Value::FunctionPointer(func_pointer) => {
        // TODO I think I can get rid of this target_module stuff maybe
        let target_module = match func_pointer.module.clone() {
          Some(module_name) => match compilation_unit.get_module(module_name.as_str()) {
            Some(modu) => modu,
            None => {
              debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
              return Err(Exception::new(context.clone(), format!("Could not find module '{}'", module_name).as_str()));
            }
          },
          None => compilation_unit.get_module(context.current_module.as_str()).unwrap(),// this should always be an available module
        };

        match target_module.functions.get(func_pointer.function.clone().as_str()) {
          Some(target_function) => {
            if context.stack.len() < target_function.parameters.len() {
              debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
              return Err(Exception::new(context.clone(), format!("Unexpected number of stack values. Expected {} and got {}.", target_function.parameters.len(), context.stack.len()).as_str()));
            }

            let mut arguments = Vec::new();
            for _ in &target_function.parameters {
              let param_value = context.stack.pop().unwrap();
              arguments.insert(0, param_value);
            }

            let return_value = compilation_unit.debug(match func_pointer.module.clone() {
              Some(module_name) => module_name,
              None => context.current_module.clone()
            }, func_pointer.function, arguments, Some(Box::new(context.clone())), debug_context);
            match return_value {
              Ok(optional_return) => match optional_return {
                Some(value) => context.stack.push(value),
                None => {}
              },
              Err(e) => {
                debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
                return Err(e);
              }
            }
          }
          None => match target_module.intrinsics.get(func_pointer.function.clone().as_str()) {
            Some(target_intrinsic) => {
              if context.stack.len() < target_intrinsic.parameters.len() {
                debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
                return Err(Exception::new(context.clone(), format!("Unexpected number of stack values. Expected {} and got {}.", target_intrinsic.parameters.len(), context.stack.len()).as_str()));
              }

              let mut arguments = Vec::new();
              for _ in &target_intrinsic.parameters {
                let param_value = context.stack.pop().unwrap();
                arguments.insert(0, param_value);
              }

              let code = match target_intrinsic.get_intrinsic_code("vm".to_string()) {
                Ok(code) => code,
                Err(message) => {
                  debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
                  return Err(Exception::new(context.clone(), message.as_str()));
                }
              };

              let return_value = INTRINSIC_MANAGER.call(code, context, arguments);
              match return_value {
                Ok(mut values) => context.stack.append(&mut values),
                Err(e) => {
                  debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
                  return Err(e);
                }
              }
            }
            None => {
              debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
              return Err(Exception::new(context.clone(), format!("Could not find function '{}' in module '{}'", func_pointer.function.clone().as_str(), target_module.name).as_str()));
            }
          },
        };
      }
      _ => {
        debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
        return Err(Exception::new(context.clone(), "Non-invokable value was attempted to be invoked"));
      }
    }

    context.program_counter += 1;
    debug_context.metric_tracker.stop(context.get_call_stack(), metric_name.clone());
    Ok(true)
  }
}
