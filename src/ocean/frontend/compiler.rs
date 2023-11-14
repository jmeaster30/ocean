use std::{env, fs, io};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parserphase1::parse_phase_one;
use crate::ocean::Ocean;

impl Ocean {
  pub fn compile(file_path: &str) -> Result<(), io::Error>{
    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());

    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let tokens = lex(file_contents);
    for token in &tokens {
      println!("{}", token)
    }

    let phase_one_ast = parse_phase_one(&tokens);
    println!("{:?}", phase_one_ast);

    Ok(())
  }
}