#![allow(warnings)]
pub mod hydro;
mod ocean;
#[cfg(test)]
mod tests;
pub mod util;

extern crate clap;

use crate::hydro::debugcontext::DebugContext;
use crate::hydro::frontend::compiler::HydroTranslateType;
use crate::hydro::value::Value;
use crate::hydro::Hydro;
use crate::ocean::Ocean;
use util::cli_args::{Cli, Command, HydroCommand, DebugOutputMode};
use clap::Parser;

fn main() -> std::io::Result<()> {
  let args = Cli::parse();
  println!("Command: {:?}", args.command);

  match args.command {
    Command::Build { tokens, ast, source_file } => {
      let compilation_unit = Ocean::compile(source_file.as_str(), tokens, ast);
      compilation_unit.print_errors();
    }
    Command::Run { tokens, ast, source_file } => {
      let compilation_unit = Ocean::compile(source_file.as_str(), tokens, ast);
      compilation_unit.print_errors();
      //println!("{:#?}", compilation_unit);
    }
    Command::Hydro { command } => match command {
      HydroCommand::Build { output_file, format, source_file } => {
        let compiled_module = match Hydro::compile(source_file.as_str()) {
          Ok(module) => module,
          Err(errors) => panic!("ERRORS\n{:#?}", errors),
        };
        Hydro::output(
          match format.as_str() {
            "binary" => HydroTranslateType::Binary,
            _ => HydroTranslateType::Binary,
          },
          &compiled_module,
          output_file.clone(),
        )?;
      }
      HydroCommand::Debug { source_file } => {
        let compilation_unit = match Hydro::compile(source_file.as_str()) {
          Ok(compilation_unit) => compilation_unit,
          Err(errors) => panic!("ERRORS\n{:#?}", errors),
        };
        let mut debug_context = DebugContext::new();

        let return_value = compilation_unit.debug("main".to_string(), "main".to_string(), vec![Value::Unsigned32(69)], None, &mut debug_context);

        match return_value {
          Ok(result) => debug_context.console(&compilation_unit, &"main".to_string(), &mut None, result).unwrap(),
          Err(e) => e.print_stacktrace(),
        }
      }
      HydroCommand::Run { source_file } => {
        let compilation_unit = match Hydro::compile(source_file.as_str()) {
          Ok(compilation_unit) => compilation_unit,
          Err(errors) => panic!("ERRORS\n{:#?}", errors),
        };
        let return_value = compilation_unit.execute("main".to_string(), "main".to_string(), vec![Value::Unsigned32(69)], None);

        match return_value {
          Ok(result) => match result {
            Some(value) => println!("{}", value.to_string()),
            None => println!("None"),
          },
          Err(e) => e.print_stacktrace(),
        }
      }
    }
  }

  Ok(())
}
