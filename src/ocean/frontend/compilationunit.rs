use std::cell::RefCell;
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
  pub dependencies: Vec<Rc<RefCell<CompilationUnit>>>,
  pub errors: Vec<Error>,
}

impl CompilationUnit {
  pub fn errored(filepath: String, error: Error) -> Self {
    Self {
      filepath,
      program: None,
      dependencies: Vec::new(),
      errors: vec![error]
    }
  }

  pub fn program(filepath: String, program: Program, dependencies: Vec<Rc<RefCell<CompilationUnit>>>, errors: Vec<Error>) -> Self {
    Self {
      filepath,
      program: Some(program),
      dependencies,
      errors
    }
  }

  pub fn add_dependency(&mut self, compilation_unit: Rc<RefCell<CompilationUnit>>) {
    self.dependencies.push(compilation_unit.clone());
  }

  pub fn print_errors(&self) {
    if !self.errors.is_empty() {
      let error_context_size = match env::var("OCEAN_ERROR_LINE_CONTEXT") {
        Ok(value) => value.parse::<usize>().unwrap(),
        Err(_) => 2,
      };

      match File::open(self.filepath.clone()) {
        Ok(mut file) => {
          let mut file_contents = String::new();
          file.read_to_string(&mut file_contents).unwrap();

          for error in &self.errors {
            error.display_message(file_contents.as_bytes(), &self.filepath, error_context_size);
          }
        }
        Err(_) => {
          for error in &self.errors {
            error.display_message_without_file(&self.filepath);
          }
        }
      }
    }

    for dependency in &self.dependencies {
      dependency.borrow().print_errors();
    }
  }
}