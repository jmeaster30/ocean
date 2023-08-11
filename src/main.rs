pub mod hydro;
pub mod util;

use util::argsparser::{ArgsParser, Argument};
use std::env;
use crate::hydro::function::Function;
use crate::hydro::instruction::{Add, Instruction, Load, Return};
use crate::hydro::instruction::PushValue;
use crate::hydro::module::Module;
use crate::hydro::value::{Reference, Value, VariableRef};
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

  let module = Module::new(
    "Main".to_string(),
    Vec::new(),
    vec![
      Function::new("Main".to_string(), Vec::new(), vec![
        Instruction::PushValue(PushValue { value: Value::Reference(Reference::Variable(VariableRef::new("funnyNumber".to_string()))) }),
        Instruction::Load(Load { }),
        Instruction::PushValue(PushValue { value: Value::Unsigned16(69) }),
        Instruction::Add(Add { }),
        Instruction::Return(Return {}),
      ])
    ]
  );

  let return_value = module.execute("Main".to_string(), vec![
    ("funnyNumber".to_string(), Value::Unsigned32(69))
  ], None);

  println!("{:#?}", return_value);

  Ok(())
}
