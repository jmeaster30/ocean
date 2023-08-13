pub mod hydro;
pub mod util;

use util::argsparser::{ArgsParser, Argument};
use std::env;
use std::path::Path;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::value::Value;
use crate::util::argsparser::Command;

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
      .command(Command::new("build")
          .arg(Argument::new("Source File")
              .last()
              .default("main.sea")
              .help("The main source file to compile")))
      .command(Command::new("debug")
          .arg(Argument::new("Source File")
              .last()
              .default("main.sea")
              .help("The main source file to compile")))
      .command(Command::new("run")
          .arg(Argument::new("Source File")
            .last()
            .default("main.sea")
            .help("The main source file to compile")))
      .command(Command::new("hydro")
          .arg(Argument::new("Source File")
              .last()
              .default("main.h2o")
              .help("The main source file to compile")));
  let _parsed_args = arg_parser.parse(args[1..].to_vec());

  let mut parser = Parser::new(Path::new("./scratch/test.h2o"))?;
  let modules = parser.parse();
  // TODO this stinks
  let mut resolved = Vec::new();
  for module in &modules {
    let mut mod_copy = module.clone();
    mod_copy.resolve(&modules);
    resolved.push(mod_copy);
  }

  let main_module = resolved.iter().find(|x| x.name == "main");

  match main_module {
    Some(module) => {
      let return_value = module.execute("main".to_string(), vec![
        ("funnyNumber".to_string(), Value::Unsigned32(69))
      ], None);

      match return_value {
        Ok(result) => println!("{:#?}", result),
        Err(e) => e.print_stacktrace(),
      }
    }
    None => {
      println!("Could not find main module :(")
    }
  }

  Ok(())
}
