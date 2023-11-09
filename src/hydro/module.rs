use std::collections::HashMap;

use crate::hydro::function::Function;
use crate::hydro::intrinsic::Intrinsic;
use crate::hydro::layouttemplate::LayoutTemplate;

#[derive(Debug, Clone)]
pub struct Module {
  pub name: String,
  pub unresolved_modules: Vec<String>,
  pub modules: Vec<String>,
  pub layout_templates: HashMap<String, LayoutTemplate>,
  pub functions: HashMap<String, Function>,
  pub intrinsics: HashMap<String, Intrinsic>,
}

impl Module {}

impl Module {
  pub fn new(name: String, modules: Vec<String>, layout_templates: Vec<LayoutTemplate>, functions: Vec<Function>) -> Self {
    Self {
      name,
      unresolved_modules: Vec::new(),
      modules,
      layout_templates: layout_templates.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, LayoutTemplate>>(),
      functions: functions.iter().map(|x| (x.clone().name, x.clone())).collect::<HashMap<String, Function>>(),
      intrinsics: HashMap::new(),
    }
  }

  pub fn build(name: &str) -> Self {
    Self {
      name: name.to_string(),
      unresolved_modules: Vec::new(),
      modules: Vec::new(),
      layout_templates: HashMap::new(),
      functions: HashMap::new(),
      intrinsics: HashMap::new(),
    }
  }

  pub fn import(mut self, module_name: String) -> Self {
    self.modules.push(module_name);
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

  pub fn intrinsic(mut self, intrinsic: Intrinsic) -> Module {
    self.intrinsics.insert(intrinsic.name.clone(), intrinsic);
    self
  }
}
