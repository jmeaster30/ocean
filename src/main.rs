pub mod hydro;
pub mod util;

use util::argsparser::{ArgsParser, Argument};
use std::env;
use crate::hydro::function::Function;
use crate::hydro::layouttemplate::LayoutTemplate;
use crate::hydro::module::Module;
use crate::hydro::value::{FunctionPointer, Reference, Value, VariableRef};
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

    //   .arg(Argument::new("Command")
    //   .first()
    //   .possible_values(vec!["help", "build", "run", "version", "hydro"])
    //   .default("run")
    //   .help("Commands for the compiler"))
    // .arg(Argument::new("Debug Lexer")
    //   .named("-t", "--output-tokens")
    //   .help("Outputs the tokens to a file called '{source_file}.tokens'"))
    // .arg(Argument::new("Debug Parser")
    //   .named("-a", "--output-ast")
    //   .help("Outputs the ast to a file called '{source_file}.ast'"))
    // .arg(Argument::new("Source File")
    //   .last()
    //   .default("main.sea")
    //   .help("The main source file to compile"));
  let _parsed_args = arg_parser.parse(args[1..].to_vec());

  let module = Module::build("Main")
    .import(Module::build("Std")
      .layout(LayoutTemplate::build("point")
        .member("x", Value::Signed128(0))
        .member("y", Value::Signed128(0)))
      .function(Function::build("GetFunnyNumber2")
        .push_value(Value::Unsigned16(420))
        .return_()))
    .function(Function::build("Main")
      .push_value(Value::Reference(Reference::Variable(VariableRef::new("funnyCoordinate".to_string()))))
      .alloc_layout(Some("Std"), "point")
      .push_value(Value::String("x".to_string()))
      .index()
      .push_value(Value::Signed128(8008))
      .store()
      .load()
      .push_value(Value::Reference(Reference::Variable(VariableRef::new("funnyNumber".to_string()))))
      .load()
      .push_value(Value::FunctionPointer(FunctionPointer::new(Some("Std".to_string()), "GetFunnyNumber2".to_string())))
      .call()
      .add()
      .add()
      .return_());

  let return_value = module.execute("Main".to_string(), vec![
    ("funnyNumber".to_string(), Value::Unsigned32(69))
  ], None);

  match return_value {
    Ok(result) => println!("{:#?}", result),
    Err(e) => e.print_stacktrace(),
  }

  Ok(())
}
