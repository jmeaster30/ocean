// TODO use crate::hydro::frontend::binaryable::Binaryable;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::module::Module;
use crate::hydro::Hydro;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::{fs, io};

pub enum HydroTranslateType {
  Binary,
}

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

  pub fn output(
    translate_type: HydroTranslateType,
    module: &Module,
    path: String,
  ) -> Result<(), Error> {
    let bytes = match translate_type {
      HydroTranslateType::Binary => {
        //TODO let mut mod_output = module.output(9);
        let mut output = vec![b'h', b'y', b'd', b'r', b'o', 0, 0, 0, 0];
        //TODO output.append(&mut mod_output);
        output
      }
    };
    let mut file = File::create(Path::new(path.as_str()))?;
    file.write(bytes.as_slice())?;
    Ok(())
  }
}
