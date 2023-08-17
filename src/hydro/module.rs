use crate::hydro::debugcontext::DebugContext;
use std::collections::HashMap;

use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::function::Function;
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::value::Value;

#[derive(Debug, Clone)]
pub struct Module {
  pub name: String,
  pub unresolved_modules: Vec<String>,
  pub modules: HashMap<String, Module>,
  pub layout_templates: HashMap<String, LayoutTemplate>,
  pub functions: HashMap<String, Function>,
}

impl Module {
  pub fn new(
    name: String,
    modules: Vec<Module>,
    layout_templates: Vec<LayoutTemplate>,
    functions: Vec<Function>,
  ) -> Self {
    Self {
      name,
      unresolved_modules: Vec::new(),
      modules: modules
        .iter()
        .map(|x| (x.clone().name, x.clone()))
        .collect::<HashMap<String, Module>>(),
      layout_templates: layout_templates
        .iter()
        .map(|x| (x.clone().name, x.clone()))
        .collect::<HashMap<String, LayoutTemplate>>(),
      functions: functions
        .iter()
        .map(|x| (x.clone().name, x.clone()))
        .collect::<HashMap<String, Function>>(),
    }
  }

  pub fn build(name: &str) -> Self {
    Self {
      name: name.to_string(),
      unresolved_modules: Vec::new(),
      modules: HashMap::new(),
      layout_templates: HashMap::new(),
      functions: HashMap::new(),
    }
  }

  pub fn import(mut self, module: Module) -> Self {
    self.modules.insert(module.name.clone(), module);
    self
  }

  pub fn import_unresolved(mut self, module: String) -> Self {
    self.unresolved_modules.push(module.clone());
    self
  }

  pub fn layout(mut self, layout: LayoutTemplate) -> Self {
    self.layout_templates.insert(layout.name.clone(), layout);
    self
  }

  pub fn function(mut self, function: Function) -> Self {
    self.functions.insert(function.name.clone(), function);
    self
  }

  pub fn resolve(&mut self, reference_modules: &Vec<Module>) {
    for module_name in &self.unresolved_modules {
      let found_module = reference_modules
        .iter()
        .find(|x| x.name == module_name.clone());
      match found_module {
        Some(module) => {
          self.modules.insert(module_name.clone(), module.clone());
        }
        None => panic!(
          "Couldn't find module '{}' required by '{}'",
          module_name, self.name
        ),
      }
    }

    let unresolved_modules = self
      .unresolved_modules
      .iter()
      .filter(|x| !self.modules.contains_key(x.clone()))
      .map(|x| x.clone())
      .collect::<Vec<String>>();
    self.unresolved_modules = unresolved_modules;
    if self.unresolved_modules.len() != 0 {
      println!("Unable to resolve dependencies of module '{}'", self.name);
    }
  }

  pub fn execute(
    &self,
    function_name: String,
    arguments: Vec<(String, Value)>,
    parent_context: Option<Box<ExecutionContext>>,
  ) -> Result<Option<Value>, Exception> {
    let mut context = ExecutionContext {
      parent_execution_context: parent_context,
      stack: Vec::new(),
      program_counter: 0,
      variables: HashMap::new(),
      return_value: None,
      current_function: function_name.clone(),
      current_module: self.name.clone(),
    };

    let args = arguments
      .iter()
      .map(|x| (x.0.clone(), x.1.clone()))
      .collect::<HashMap<String, Value>>();

    let current_function = self.functions.get(&*function_name).unwrap();

    for param in &current_function.parameters {
      match args.get(param.as_str()) {
        Some(value) => {
          context.variables.insert(param.clone(), value.clone());
        }
        None => {}
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

  pub fn debug(
    &self,
    function_name: String,
    arguments: Vec<(String, Value)>,
    parent_context: Option<Box<ExecutionContext>>,
    debug_context: &mut DebugContext,
  ) -> Result<Option<Value>, Exception> {
    let mut context = ExecutionContext {
      parent_execution_context: parent_context.clone(),
      stack: Vec::new(),
      program_counter: 0,
      variables: HashMap::new(),
      return_value: None,
      current_function: function_name.clone(),
      current_module: self.name.clone(),
    };

    // every function that is called from the code will have a parent_context set. When the parent context is not there then we are in the main function before pc 0
    match parent_context {
      Some(_) => {}
      None => {
        debug_context.console(self, Some(&mut context), None, None);
      }
    }

    debug_context.start_core_metric(
      self.name.clone(),
      function_name.clone(),
      "total".to_string(),
    );

    let args = arguments
      .iter()
      .map(|x| (x.0.clone(), x.1.clone()))
      .collect::<HashMap<String, Value>>();

    let current_function = self.functions.get(&*function_name).unwrap();

    for param in &current_function.parameters {
      match args.get(param.as_str()) {
        Some(value) => {
          context.variables.insert(param.clone(), value.clone());
        }
        None => {}
      }
    }

    while context.program_counter.clone() < current_function.body.len() {
      // check for break points
      if debug_context.is_break_point(
        self.name.clone(),
        function_name.clone(),
        context.program_counter,
      ) {
        let current_pc = Some(context.program_counter.clone());
        debug_context.console(self, Some(&mut context), current_pc, None);
      }

      //check for profile points here

      let inst = current_function.body[context.program_counter.clone()].clone();
      let cont = inst.debug(self, &mut context, debug_context);
      match cont {
        Ok(should_continue) if !should_continue => break,
        Err(exception) => {
          exception.print_stacktrace();
          let current_pc = Some(context.program_counter.clone());
          debug_context.console(self, Some(&mut context), current_pc, None);
          debug_context.stop_core_metric(
            self.name.clone(),
            function_name.clone(),
            "total".to_string(),
          );
          return Err(exception);
        }
        _ => {}
      }
    }

    debug_context.stop_core_metric(
      self.name.clone(),
      function_name.clone(),
      "total".to_string(),
    );
    Ok(context.return_value.clone())
  }
}
