// TODO use crate::hydro::frontend::binaryable::Binaryable;
use crate::hydro::frontend::parser::Parser;
use crate::hydro::module::Module;
use crate::hydro::Hydro;
use std::fs::File;
use std::io::{Error, Write};
use std::path::Path;
use std::{env, fs};
use std::time::Instant;

pub enum HydroTranslateType {
  Binary,
}

impl Hydro {
  pub fn compile(file_path: &str) -> Result<Module, Vec<String>> {
    let now = Instant::now();
    let path = Path::new(file_path);
    let project_root = path.parent().unwrap().to_str().unwrap();
    let std_root = match env::var("HYDRO_STD_ROOT") {
      Ok(value) => Some(value),
      Err(_) => None,
    };

    println!("PROJECT ROOT: {:?}", project_root);
    println!("STD ROOT: {:?}", std_root);

    match Hydro::internal_compile(file_path, project_root, std_root) {
      Ok(found_modules) => match found_modules.iter().find(|x| x.name == "main") {
        Some(module) => {
          let new_now = Instant::now();
          println!("Compilation Completed In: {:?}", new_now.duration_since(now));
          Ok(module.clone())
        },
        None => {
          let new_now = Instant::now();
          println!("Compilation Completed In: {:?}", new_now.duration_since(now));
          Err(vec!["Main module not found :(".to_string()])
        },
      }
      Err((_, errors)) => {
        let new_now = Instant::now();
        println!("Compilation Completed In: {:?}", new_now.duration_since(now));
        Err(errors)
      },
    }
  }

  fn internal_compile(file_path: &str, project_root: &str, std_root: Option<String>) -> Result<Vec<Module>, (bool, Vec<String>)> {
    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());
    let parser = Parser::new(path);
    if parser.is_err() {
      return Err((false, vec![format!("Source file not found '{}'", path.display())]));
    }

    let mut errors = Vec::new();

    let mut modules = parser.unwrap().parse();
    let mut resolved = Vec::new();
    for mut module in &mut modules {
      for unresolved_module in &module.unresolved_modules {
        match Hydro::resolve_module(unresolved_module.as_str(), &resolved, project_root, std_root.clone(), module.name.clone()) {
          Ok((target_module, mut new_found_modules)) => {
            module.modules.insert(unresolved_module.clone(), target_module.clone());
            resolved.append(&mut new_found_modules);
          }
          Err(mut new_errors) => errors.append(&mut new_errors),
        }
      }
      module.unresolved_modules = module.unresolved_modules.iter().filter(|x| !module.modules.contains_key((*x).clone().as_str())).map(|x| x.clone()).collect::<Vec<String>>();
      if module.unresolved_modules.len() != 0 {
        errors.push(format!("Unable to resolve all of the dependencies for '{}'", module.name));
      }
      resolved.push(module.clone());
    }

    if errors.len() == 0 {
      Ok(resolved)
    } else {
      Err((true, errors))
    }
  }

  fn resolve_module(module_name: &str, reference_modules: &Vec<Module>, project_root: &str, std_root: Option<String>, dependent_module_name: String) -> Result<(Module, Vec<Module>), Vec<String>> {
    let mut target_module = match reference_modules.iter().find(|x| x.name == module_name.clone()) {
      Some(module) => Some(module.clone()),
      None => None,
    };
    let mut new_found_modules = Vec::new();
    let mut errors = Vec::new();

    if target_module.is_none() {
      let search_paths = Hydro::get_possible_files(module_name, project_root, std_root.clone());
      for path in search_paths {
        match Hydro::internal_compile(path.as_str(), project_root, std_root.clone()) {
          Ok(found_modules) => {
            for module in found_modules {
              if module.name == "main" { continue; }
              if module.name == module_name { target_module = Some(module.clone()); }
              new_found_modules.push(module);
            }
            break;
          }
          Err((found_source_file, mut new_errors)) => if found_source_file {
            errors.append(&mut new_errors);
            break;
          } else {
            continue;
          }
        }
      }
    }

    match target_module {
      Some(module) => Ok((module.clone(), new_found_modules)),
      None => {
        errors.push(format!("Could not find module '{}' which is a dependency of '{}'", module_name, dependent_module_name));
        Err(errors)
      }
    }
  }

  fn get_possible_files(module_name: &str, project_root: &str, std_root: Option<String>) -> Vec<String> {
    let components = module_name.split(&['.', '/', '\\'][..]).collect::<Vec<&str>>();
    let mut paths = Vec::new();

    for i in 0..components.len() {
      let mut path = "".to_string();
      // this match kinda sucks with the to_strings but it has to do with lifetime bs
      path += match components[0] {
        "std" => match std_root.clone() {
          Some(value) => value,
          None => project_root.to_string(), // fallback to project root!!
        },
        "src" => project_root.to_string(),
        _ => ".".to_string(),
      }.as_str();
      path += "/";
      for j in 0..=i {
        path += components[j];
        path += if j == i { ".h2o" } else { "/" }
      }
      paths.push(path);
    }
    paths.reverse();
    paths
  }

  pub fn output(translate_type: HydroTranslateType, _module: &Module, path: String) -> Result<(), Error> {
    let bytes = match translate_type {
      HydroTranslateType::Binary => {
        //TODO let mut mod_output = module.output(9);
        let output = vec![b'h', b'y', b'd', b'r', b'o', 0, 0, 0, 0];
        //TODO output.append(&mut mod_output);
        output
      }
    };
    let mut file = File::create(Path::new(path.as_str()))?;
    file.write(bytes.as_slice())?;
    Ok(())
  }
}
