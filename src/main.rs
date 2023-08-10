pub mod hydro;
pub mod util;

use util::argsparser::{ArgsParser, Argument};
use std::env;
use crate::hydro::function::Function;
use crate::hydro::instruction::{Add, Instruction, Return};
use crate::hydro::instruction::PushValue;
use crate::hydro::module::Module;
use crate::hydro::value::Value;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let arg_parser = ArgsParser::new("Ocean")
    .version("0.0.1")
    .author("John Easterday <jmeaster30>")
    .description("A C-like programming language (get it like C sounds like sea and oceans are kinda like seas lol)")
    .arg(Argument::new("Command")
      .first()
      .possible_values(vec!["help", "build", "run", "version", "hydro"])
      .default("run")
      .help("Commands for the compiler"))
    .arg(Argument::new("Debug Lexer")
      .named("-t", "--output-tokens")
      .help("Outputs the tokens to a file called '{source_file}.tokens'"))
    .arg(Argument::new("Debug Parser")
      .named("-a", "--output-ast")
      .help("Outputs the ast to a file called '{source_file}.ast'"))
    .arg(Argument::new("Source File")
      .last()
      .default("main.sea")
      .help("The main source file to compile"));
  let _parsed_args = arg_parser.parse(args[1..].to_vec());

  let module = Module::new(
    "Main".to_string(),
    Vec::new(),
    vec![
      Function::new("Main".to_string(), Vec::new(), vec![
        Instruction::PushValue(PushValue { value: Value::Signed8(-50) }),
        Instruction::PushValue(PushValue { value: Value::Unsigned16(69) }),
        Instruction::Add(Add { }),
        Instruction::Return(Return {}),
      ])
    ]
  );

  let return_value = module.execute("Main".to_string(), Vec::new(), None);

  println!("{:#?}", return_value);

  Ok(())
}
