use crate::hydro::exception::Exception;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::value::Value;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::Write;

// TODO This way kinda sucks a lot and the name implies it would handle code output for non-vm intrinsic functions

lazy_static! {
  pub static ref INTRINSIC_MANAGER: IntrinsicManager = IntrinsicManager::initialize();
}

type IntrinsicFunction = fn(&ExecutionContext, Vec<Value>) -> Result<Vec<Value>, Exception>;

pub struct IntrinsicManager {
  mapping: HashMap<String, IntrinsicFunction>,
}

impl IntrinsicManager {
  fn initialize() -> IntrinsicManager {
    let mut manager = IntrinsicManager { mapping: HashMap::new() };

    manager.add_map("print", print);
    manager.add_map("println", println);
    manager.add_map("flush", flush);
    manager.add_map("readline", readline);

    manager
  }

  fn add_map(&mut self, name: &str, function: IntrinsicFunction) {
    self.mapping.insert(name.to_string(), function);
  }

  pub fn call(&self, intrinsic_name: String, execution_context: &ExecutionContext, arguments: Vec<Value>) -> Result<Vec<Value>, Exception> {
    if self.mapping.contains_key(&intrinsic_name) {
      self.mapping[&intrinsic_name](execution_context, arguments)
    } else {
      Err(Exception::new(execution_context.clone(), format!("Intrinsic '{}' is undefined :(", intrinsic_name).as_str()))
    }
  }
}

fn print(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 1 {
    Err(Exception::new(context.clone(), format!("Expected 1 argument for print but got {}", args.len()).as_str()))
  } else {
    print!("{}", args[0].to_string());
    Ok(Vec::new())
  }
}

fn println(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 1 {
    Err(Exception::new(context.clone(), format!("Expected 1 argument for print but got {}", args.len()).as_str()))
  } else {
    println!("{}", args[0].to_string());
    Ok(Vec::new())
  }
}

fn flush(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 0 {
    Err(Exception::new(context.clone(), format!("Expected 0 arguments for readline but got {}", args.len()).as_str()))
  } else {
    match io::stdout().flush() {
      Ok(()) => Ok(Vec::new()),
      Err(io_error) => Err(Exception::new(context.clone(), io_error.to_string().as_str()))
    }
  }
}

fn readline(context: &ExecutionContext, args: Vec<Value>) -> Result<Vec<Value>, Exception> {
  if args.len() != 0 {
    Err(Exception::new(context.clone(), format!("Expected 0 arguments for readline but got {}", args.len()).as_str()))
  } else {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_) => Ok(vec![Value::string(input)]),
      Err(io_error) => Err(Exception::new(context.clone(), io_error.to_string().as_str()))
    }
  }
}