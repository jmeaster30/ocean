use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::value::Value;

// TODO This way kinda sucks a lot and the name implies it would handle code output for non-vm intrinsic functions

lazy_static! {
  static ref INTRINSIC_MANAGER: IntrinsicManager = IntrinsicManager::initialize();
}

type IntrinsicFunction = fn(&ExecutionContext, Vec<Value>) -> Result<Vec<Value>, Exception>;

pub struct IntrinsicManager {
  mapping: HashMap<String, IntrinsicFunction>,
}

impl IntrinsicManager {
  fn initialize() -> IntrinsicManager {
    let mut manager = IntrinsicManager {
      mapping: HashMap::new()
    };

    manager.add_map("print", print);
    manager.add_map("println", println);

    manager
  }

  fn add_map(&mut self, name: &str, function: IntrinsicFunction) {
    self.mapping.insert(name.to_string(), function);
  }

  pub fn call(&self, intrinsic_name: String, execution_context: &ExecutionContext, arguments: Vec<Value>) -> Result<Vec<Value>, Exception> {
    self.mapping[&intrinsic_name](execution_context, arguments)
  }
}

fn print(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 1 {
    Err(Exception::new(context.clone(), format!(
      "Expected 1 argument for print but got {}", args.len()
    ).as_str()))
  } else {
    print!("{:?}", args[0]);
    Ok(Vec::new())
  }
}

fn println(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 1 {
    Err(Exception::new(context.clone(), format!(
      "Expected 1 argument for print but got {}", args.len()
    ).as_str()))
  } else {
    println!("{:?}", args[0]);
    Ok(Vec::new())
  }
}