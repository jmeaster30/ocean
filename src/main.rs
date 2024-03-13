pub mod hydro;
mod ocean;
#[cfg(test)]
mod tests;
pub mod util;

use crate::hydro::debugcontext::DebugContext;
use crate::hydro::frontend::compiler::HydroTranslateType;
use crate::hydro::value::Value;
use crate::hydro::Hydro;
use crate::ocean::Ocean;
use crate::util::argsparser::Command;
use std::env;
use util::argsparser::{ArgsParser, Argument};

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  #[rustfmt::skip]
  let arg_parser = {
    ArgsParser::new("Ocean")
      .version("0.0.1")
      .author("Lily Easterday <jmeaster30>")
      .description("A C-like programming language (get it like C sounds like sea and oceans are kinda like seas lol)")
      .command(Command::new("help")
        .description("Print this help message"))
      .command(Command::new("version")
        .description("Print version information"))
      .command(Command::new("run")
        .arg(Argument::new("Tokens")
          .named("-t", "--tokens")
          .takes_value()
          .possible_values(vec!["print", "file", "none"])
          .default("none"))
        .arg(Argument::new("Ast")
          .named("-a", "--ast")
          .takes_value()
          .possible_values(vec!["print", "file", "none"])
          .default("none"))
        .arg(Argument::new("Source File")
          .last()
          .default("main.sea")
          .help("The main source file to compile")))
      .command(Command::new("hydro-build")
        .arg(Argument::new("Output File")
          .default("main.h2o.bin")
          .takes_value()
          .named("o", "output-file"))
        .arg(Argument::new("Output Format")
          .default("binary")
          .takes_value()
          .named("f", "output-format")
          .possible_values(vec!["binary", "source"]))
        .arg(Argument::new("Source File")
          .last()
          .default("main.h2o")
          .help("The main source file to compile")))
      .command(Command::new("hydro-debug")
        .arg(Argument::new("Source File")
          .last()
          .default("main.h2o")
          .help("The main source file to compile")))
      .command(Command::new("hydro-run")
        .arg(Argument::new("Source File")
          .last()
          .default("main.h2o")
          .help("The main source file to compile")))
  };

  match arg_parser.parse(args[1..].to_vec()) {
    Ok(arguments) => match arguments.get("command") {
      Some(command) => match command.as_str() {
        "help" => {
          arg_parser.print_help();
        }
        "version" => {
          arg_parser.print_version_info();
        }
        "hydro-build" => {
          let compiled_module = match Hydro::compile(arguments.get("Source File").unwrap().as_str()) {
            Ok(module) => module,
            Err(errors) => panic!("ERRORS\n{:#?}", errors),
          };
          Hydro::output(
            match arguments.get("Output Format").unwrap().as_str() {
              "binary" => HydroTranslateType::Binary,
              _ => HydroTranslateType::Binary,
            },
            &compiled_module,
            arguments.get("Output File").unwrap().clone(),
          )?;
        }
        "hydro-run" => {
          let compilation_unit = match Hydro::compile(arguments.get("Source File").unwrap().as_str()) {
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
        "hydro-debug" => {
          let compilation_unit = match Hydro::compile(arguments.get("Source File").unwrap().as_str()) {
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
        "build" => {
          let compilation_unit = Ocean::compile(arguments.get("Source File").unwrap().as_str(), arguments.get("Tokens").unwrap().as_str(), arguments.get("Ast").unwrap().as_str());

          compilation_unit.print_errors();
        }
        "run" => {
          let compilation_unit = Ocean::compile(arguments.get("Source File").unwrap().as_str(), arguments.get("Tokens").unwrap().as_str(), arguments.get("Ast").unwrap().as_str());

          compilation_unit.print_errors();
        }
        _ => todo!("Unimplemented command :("),
      },
      None => {
        println!("Expected a command but didn't get one :(");
        arg_parser.print_usage();
      }
    },
    Err(err) => {
      println!("{}", err);
      arg_parser.print_usage();
    }
  };

  Ok(())
}
