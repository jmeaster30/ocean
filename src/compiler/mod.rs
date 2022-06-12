pub mod errors;
pub mod lexer;
pub mod parser;

use self::errors::*;
use self::lexer::*;
use self::parser::ast::*;
use self::parser::*;

pub struct CompilationUnit {
  filename: String,
  file_content: String,
  //subunits: Vec<CompilationUnit>,
  tokens: TokenStack,
  ast: Option<Program>,
  // symbol table
  errors: Vec<OceanError>,
}

impl CompilationUnit {
  pub fn new(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit {
      filename,
      file_content,
      tokens: TokenStack::new(),
      ast: None,
      errors: Vec::new(),
    }
  }

  pub fn compile(&mut self) {
    println!("{}", self.filename);
    // Lexical pass
    let (tokens, mut lexical_errors) = lex(self.file_content.clone());
    self.tokens = tokens;
    self.errors.append(&mut lexical_errors);

    for token in self.tokens.iter() {
      token.print();
      println!("");
    }

    // Parser pass
    let (ast, mut parse_errors) = parse(&mut self.tokens);
    self.ast = ast;
    self.errors.append(&mut parse_errors);

    if !self.errors.is_empty() {
      self.print_errors();
    }

    // extra passes
    println!("Good :)");
  }

  pub fn print_errors(&self) {
    for error in &self.errors {
      display_error(self, error);
      println!();
    }
  }
}

pub fn compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::new(filename, file_content);
  main.compile();
}
