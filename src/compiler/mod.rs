pub mod lexer;
pub mod parser;
pub mod errors;

use self::lexer::*;
use self::errors::*;

pub struct CompilationUnit {
  filename: String,
  file_content: String,
  //subunits: Vec<CompilationUnit>,
  tokens: TokenStack,
  // ast
  // symbol table
  errors: Vec<OceanError>,
}

impl CompilationUnit {
  pub fn new(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit { 
      filename, 
      file_content, 
      tokens: TokenStack::new(), 
      errors: Vec::new()
    }
  }

  pub fn compile(&mut self) {
    println!("{}", self.filename);
    // Lexical pass
    (self.tokens, self.errors) = lex(self.file_content.clone());
    if !self.errors.is_empty() {
      self.print_errors();
      return;
    }

    // Parser pass
    //(self.ast, self.errors) = parse(self.tokens);
    if !self.errors.is_empty() {
      self.print_errors();
      return;
    }

    // extra passes
    println!("Good :)");
  }

  pub fn print_errors(&self) {
    for error in &self.errors {
      display_error(self, error);
    }
  }
}

pub fn compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::new(filename, file_content);
  main.compile();
}