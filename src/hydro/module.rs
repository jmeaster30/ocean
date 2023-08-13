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
  pub fn new(name: String, modules: Vec<Module>, layout_templates: Vec<LayoutTemplate>, functions: Vec<Function>) -> Self {
    Self {
      name,
      unresolved_modules: Vec::new(),
      modules: modules.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, Module>>(),
      layout_templates: layout_templates.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, LayoutTemplate>>(),
      functions: functions.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, Function>>(),
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

  pub fn execute(&self, function_name: String, arguments: Vec<(String, Value)>, parent_context: Option<Box<ExecutionContext>>) -> Result<Option<Value>, Exception> {
    let mut context = ExecutionContext {
      parent_execution_context: parent_context,
      stack: Vec::new(),
      program_counter: 0,
      variables: HashMap::new(),
      return_value: None,
      current_function: function_name.clone(),
      current_module: self.name.clone(),
    };

    let args = arguments.iter().map(|x| (x.0.clone(), x.1.clone())).collect::<HashMap<String, Value>>();

    let current_function = self.functions.get(&*function_name).unwrap();

    for param in &current_function.parameters {
      match args.get(param.as_str()) {
        Some(value) => {context.variables.insert(param.clone(), value.clone());},
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

    return Ok(context.return_value.clone());
  }
}
