use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use crate::ocean::frontend::ast::Program;
use crate::util::errors::Error;

#[derive(Debug, Clone)]
pub struct CompilationUnit {
  pub filepath: String,
  pub program: Option<Program>,
  pub dependencies: HashMap<String, Rc<RefCell<CompilationUnit>>>,
  pub errors: Vec<Error>,
}

impl CompilationUnit {
  pub fn errored(filepath: String, error: Error) -> Self {
    Self {
      filepath,
      program: None,
      dependencies: HashMap::new(),
      errors: vec![error]
    }
  }

  pub fn program(filepath: String, program: Program, errors: Vec<Error>) -> Self {
    Self {
      filepath,
      program: Some(program),
      dependencies: HashMap::new(),
      errors
    }
  }

  pub fn add_dependency(&mut self, compilation_unit: Rc<RefCell<CompilationUnit>>) {
    self.dependencies.insert(compilation_unit.borrow().filepath.clone(), compilation_unit.clone());
  }

  pub fn print_errors(&self) {
    if !self.errors.is_empty() {
      let error_context_size = match env::var("OCEAN_ERROR_LINE_CONTEXT") {
        Ok(value) => value.parse::<usize>().unwrap(),
        Err(_) => 2,
      };

      let mut file = File::open(self.filepath.clone()).unwrap();
      let mut file_contents = String::new();
      file.read_to_string(&mut file_contents).unwrap();

      for error in &self.errors {
        error.display_message(file_contents.as_bytes(), &self.filepath, error_context_size);
      }
    }

    for dependency in self.dependencies.values() {
      dependency.borrow().print_errors();
    }
  }
}