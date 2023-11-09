use std::collections::HashMap;
use crate::hydro::debugcontext::DebugContext;
use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::module::Module;
use crate::hydro::value::{Type, Value};

pub struct CompilationUnit {
  modules: HashMap<String, Module>,
}

impl CompilationUnit {
  pub fn new() -> Self {
    Self { modules: HashMap::new() }
  }

  pub fn merge(&mut self, compilation_unit: &CompilationUnit) {
    for module in compilation_unit.modules.values() {
      self.add_module(module);
    }
  }

  pub fn add_module(&mut self, module: &Module) {
    self.modules.insert(module.name.clone(), module.clone());
  }

  pub fn remove_module(&mut self, module_name: String) {
    self.modules.remove(module_name.as_str());
  }

  pub fn get_module(&self, module_name: &str) -> Option<&Module> {
    self.modules.get(module_name)
  }

  pub fn contains_module(&self, module_name: &str) -> bool {
    self.modules.contains_key(module_name)
  }

  pub fn resolve_type(&self, type_to_resolve: Type, context: &ExecutionContext) -> Result<Type, Exception> {
    match type_to_resolve {
      Type::Any => Ok(type_to_resolve),
      Type::Float32 => Ok(type_to_resolve),
      Type::Float64 => Ok(type_to_resolve),
      Type::Boolean => Ok(type_to_resolve),
      Type::Unsigned8 => Ok(type_to_resolve),
      Type::Unsigned16 => Ok(type_to_resolve),
      Type::Unsigned32 => Ok(type_to_resolve),
      Type::Unsigned64 => Ok(type_to_resolve),
      Type::Unsigned128 => Ok(type_to_resolve),
      Type::Signed8 => Ok(type_to_resolve),
      Type::Signed16 => Ok(type_to_resolve),
      Type::Signed32 => Ok(type_to_resolve),
      Type::Signed64 => Ok(type_to_resolve),
      Type::Signed128 => Ok(type_to_resolve),
      Type::FunctionPointer(args, returns) => Ok(Type::FunctionPointer(args, returns)),
      Type::Reference(subtype) => {
        let resolved_subtype = self.resolve_type(*subtype, context)?;
        Ok(Type::Reference(Box::new(resolved_subtype)))
      }
      Type::Array(length, subtype) => {
        let resolved_subtype = self.resolve_type(*subtype, context)?;
        Ok(Type::Array(length, Box::new(resolved_subtype)))
      }
      Type::Layout(module_name, layout_name, Some(subtype_map)) => Ok(Type::Layout(module_name, layout_name, Some(subtype_map))),
      Type::Layout(module_name, layout_name, None) => match module_name.clone().as_str() {
        "this" => match self.modules.get(context.current_module.as_str()) {
          Some(module) => match module.layout_templates.get(layout_name.as_str()) {
            Some(template) => Ok(template.to_type(module_name)),
            None => return Err(Exception::new(context.clone(), format!("Layout '{}' not found in module '{}'", layout_name, module_name).as_str())),
          }
          None => return Err(Exception::new(context.clone(), format!("Module '{}' not found.", context.current_module).as_str())),
        },
        module_name => match self.modules.get(module_name) {
          Some(template_module) => match template_module.layout_templates.get(layout_name.as_str()) {
            Some(template) => Ok(template.to_type(module_name.to_string())),
            None => return Err(Exception::new(context.clone(), format!("Layout '{}' not found in module '{}'", layout_name, module_name).as_str())),
          },
          None => return Err(Exception::new(context.clone(), format!("Module '{}' not found.", module_name).as_str())),
        },
      },
    }
  }

  pub fn execute(&self, module_name: String, function_name: String, arguments: Vec<Value>, parent_context: Option<Box<ExecutionContext>>) -> Result<Option<Value>, Exception> {
    let mut context = ExecutionContext {
      parent_execution_context: parent_context,
      stack: Vec::new(),
      program_counter: 0,
      variables: HashMap::new(),
      return_value: None,
      current_function: function_name.clone(),
      current_module: module_name.clone(),
    };

    let current_function = match self.modules.get(module_name.as_str()) {
      Some(module) => module.functions.get(&*function_name).unwrap(),
      None => return Err(Exception::new(context, format!("Could not find module '{}' in the compilation unit.", module_name).as_str())),
    };

    for (expected_type, got_value) in current_function.parameters.iter().zip(arguments) {
      if Type::subset(&got_value.type_of(), expected_type) {
        context.stack.push(got_value);
      } else {
        return Err(Exception::new(context, format!("Unexpected function parameter type found {:?} but expected {:?}", got_value.type_of(), expected_type).as_str()));
      }
    }

    while context.program_counter.clone() < current_function.body.len() {
      let inst = current_function.body[context.program_counter.clone()].clone();
      let cont = inst.execute(self, &mut context);
      match cont {
        Ok(should_continue) if !should_continue => break,
        Err(exception) => {
          return Err(exception);
        }
        _ => {}
      }
    }

    Ok(context.return_value.clone())
  }

  pub fn debug(&self, module_name: String, function_name: String, arguments: Vec<Value>, parent_context: Option<Box<ExecutionContext>>, debug_context: &mut DebugContext) -> Result<Option<Value>, Exception> {
    let mut context = ExecutionContext {
      parent_execution_context: parent_context.clone(),
      stack: Vec::new(),
      program_counter: 0,
      variables: HashMap::new(),
      return_value: None,
      current_function: function_name.clone(),
      current_module: module_name.clone(),
    };

    // every function that is called from the code will have a parent_context set. When the parent context is not there then we are in the main function before pc 0
    match parent_context {
      Some(_) => {}
      None => {
        match debug_context.console(self, &module_name, &mut Some(&mut context), None) {
          Ok(_) => {}
          Err(readline_error) => return Err(Exception::new(context, readline_error.to_string().as_str())),
        }
        // hacky way of letting the user step immediately when the program runs
        if debug_context.step.clone().is_some() {
          debug_context.step = Some(debug_context.step.clone().unwrap() + 1);
        }
      }
    }

    debug_context.metric_tracker.start(context.get_call_stack(), "total".to_string());

    let current_function = match self.modules.get(module_name.as_str()) {
      Some(module) => module.functions.get(&*function_name).unwrap(),
      None => return Err(Exception::new(context, format!("Could not find module '{}' in the compilation unit.", module_name).as_str())),
    };

    for (expected_type, got_value) in current_function.parameters.iter().zip(arguments) {
      if Type::subset(&got_value.type_of(), expected_type) {
        context.stack.push(got_value);
      } else {
        debug_context.metric_tracker.stop(context.get_call_stack(), "total".to_string());
        return Err(Exception::new(context, format!("Unexpected function parameter type found {:?} but expected {:?}", got_value.type_of(), expected_type).as_str()));
      }
    }

    while context.program_counter.clone() < current_function.body.len() {
      // check for break points
      let should_step_break = debug_context.update_step();
      if should_step_break || debug_context.is_break_point(module_name.clone(), function_name.clone(), context.program_counter) {
        match debug_context.console(self, &module_name, &mut Some(&mut context), None) {
          Ok(_) => {}
          Err(readline_error) => return Err(Exception::new(context, readline_error.to_string().as_str())),
        }
      }

      //check for profile points here

      let inst = current_function.body[context.program_counter.clone()].clone();
      let cont = inst.debug(self, &mut context, debug_context);
      match cont {
        Ok(should_continue) if !should_continue => break,
        Err(exception) => {
          exception.print_stacktrace();
          match debug_context.console(self, &module_name, &mut Some(&mut context), None) {
            Ok(_) => {}
            Err(readline_error) => return Err(Exception::new(context, readline_error.to_string().as_str())),
          }
          debug_context.metric_tracker.stop(context.get_call_stack(), "total".to_string());
          return Err(exception);
        }
        _ => {}
      }
    }

    debug_context.metric_tracker.stop(context.get_call_stack(), "total".to_string());
    Ok(context.return_value.clone())
  }
}