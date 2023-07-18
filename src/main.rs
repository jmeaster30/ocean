pub mod hydro;
pub mod util;

use util::argsparser::{ArgsParser, Argument};
use std::env;

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
  let parsed_args = arg_parser.parse(args[1..].to_vec());
  
  Ok(())
}
