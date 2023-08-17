pub mod hydro;
pub mod util;
#[cfg(test)]
mod tests;

use crate::hydro::debugcontext::DebugContext;
use crate::hydro::frontend::compiler::Hydro;
use crate::hydro::value::Value;
use crate::util::argsparser::Command;
use std::env;
use util::argsparser::{ArgsParser, Argument};

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let arg_parser = ArgsParser::new("Ocean")
    .version("0.0.1")
    .author("John Easterday <jmeaster30>")
    .description("A C-like programming language (get it like C sounds like sea and oceans are kinda like seas lol)")
      .command(Command::new("help")
        .description("Print this help message"))
      .command(Command::new("version")
        .description("Print version information"))
      .command(Command::new("hydro-build")
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
          .help("The main source file to compile")));

  match arg_parser.parse(args[1..].to_vec()) {
    Ok(arguments) => match arguments.get("command") {
      Some(command) => match command.as_str() {
        "help" => {
          arg_parser.print_help();
        }
        "version" => {
          arg_parser.print_version_info();
        }
        "hydro-run" => {
          let module = Hydro::compile(arguments.get("Source File").unwrap().as_str());

          let return_value = module?.execute(
            "main".to_string(),
            vec![("funnyNumber".to_string(), Value::Unsigned32(69))],
            None,
          );

          match return_value {
            Ok(result) => println!("{:#?}", result),
            Err(e) => e.print_stacktrace(),
          }
        }
        "hydro-debug" => {
          let module = Hydro::compile(arguments.get("Source File").unwrap().as_str())?;
          let mut debug_context = DebugContext::new();

          let return_value = module.debug(
            "main".to_string(),
            vec![("funnyNumber".to_string(), Value::Unsigned32(69))],
            None,
            &mut debug_context,
          );

          // output some metrics or open debug console?

          match return_value {
            Ok(result) => debug_context.console(&module, None, result),
            Err(e) => e.print_stacktrace(),
          }
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
