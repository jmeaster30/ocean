pub mod errors;
pub mod lexer;
pub mod parser;
pub mod passes;

use self::errors::*;
use self::lexer::*;
use self::passes::*;

pub struct CompilationUnit {
  filename: String,
  file_content: String,
  dependencies: Vec<CompilationUnit>,
  passes: Vec<Pass>,
}

impl CompilationUnit {
  pub fn new(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit {
      filename,
      file_content,
      dependencies: Vec::new(),
      passes: Vec::new(),
    }
  }

  pub fn compile(&mut self) {
    println!("{}", self.filename);

    let pass_list: Vec<(fn(&CompilationUnit) -> Pass, fn(&Pass) -> bool)> = vec![
      (lexer_pass, |pass| {
        println!("lex check");
        match pass {
          Pass::Lexer(tokens, _) => !tokens.is_empty(),
          _ => false,
        }
      }),
      (parser_pass, |pass| {
        println!("parse check");
        match pass {
          Pass::Parser(Some(ast), _) => {
            println!("{}", ast);
            true
          }
          _ => false,
        }
      }),
    ];

    for (pass, success) in pass_list {
      let pass_result = pass(&self);
      if !success(&pass_result) {
        self.passes.push(pass_result);
        println!("fail");
        break;
      }
      self.passes.push(pass_result);
    }

    self.print_errors();

    println!("Good :)");
  }

  pub fn print_errors(&self) {
    for pass in &self.passes {
      match pass {
        Pass::Lexer(_, errors) | Pass::Parser(_, errors) => {
          for error in errors {
            display_error(self, &error)
          }
        }
        _ => {}
      }
    }
  }
}

pub fn compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::new(filename, file_content);
  main.compile();
}
