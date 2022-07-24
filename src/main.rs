mod compiler;
use compiler::compile;

mod util;
use util::argsparser::{ArgsParser, Argument};

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let args: Vec<String> = env::args().collect();
  let arg_parser = ArgsParser::new("Ocean")
    .version("0.0.1")
    .author("John Easterday <jmeaster>")
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
  let parsed_args = arg_parser.parse(args[1..].to_vec());
  match parsed_args {
    Ok(x) => {
      let command = x.get("Command").unwrap().clone().unwrap();
      if command == "version".to_string() {
        arg_parser.print_version_info();
      } else if command == "help".to_string() {
        arg_parser.print_help();
      } else if command == "run".to_string() || command == "build".to_string() {
        let source = x.get("Source File").unwrap().clone().unwrap();
        println!("{}", source);
        let mut file = File::open(source.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        compile(source.to_string(), contents.to_string());
      } else {
        todo!();
      }
    }
    Err(msg) => {
      println!("Arg parsing error: {}", msg);
      arg_parser.print_usage();
    }
  }

  
  Ok(())
}
