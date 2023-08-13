use crate::hydro::frontend::parser::Parser;
use crate::hydro::module::Module;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::{fs, io};

pub struct Hydro {}

impl Hydro {
  pub fn compile(file_path: &str) -> io::Result<Module> {
    let path = Path::new(file_path);
    println!(
      "Compiling '{}' (absolute '{:?}' from '{:?}')",
      path.display(),
      fs::canonicalize(path),
      std::env::current_dir()
    );
    let mut parser = Parser::new(path)?;
    let modules = parser.parse();
    // TODO this stinks
    let mut resolved = Vec::new();
    for module in &modules {
      let mut mod_copy = module.clone();
      mod_copy.resolve(&modules);
      resolved.push(mod_copy);
    }

    match resolved.iter().find(|x| x.name == "main") {
      Some(module) => Ok(module.clone()),
      None => Err(Error::new(ErrorKind::NotFound, "Main module not found :(")),
    }
  }
}
