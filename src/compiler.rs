#![allow(
  unused_variables,
  unused_imports,
  unreachable_patterns,
  unused_assignments,
  dead_code
)]

pub mod lexer;
pub mod macros;
pub mod parser;
pub mod passes;
pub mod semantic_analysis;

use self::lexer::*;
use self::passes::*;
use crate::util::errors::*;

pub struct CompilationUnit {
  pub filename: String,
  pub file_content: String,
  pub dependencies: Vec<CompilationUnit>,
  pub passes: Vec<Pass>,
}

impl CompilationUnit {
  pub fn from_file(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit {
      filename,
      file_content,
      dependencies: Vec::new(),
      passes: Vec::new(),
    }
  }

  pub fn compile(&mut self, max_pass: Option<i32>) {
    println!("Compiling '{}'...", self.filename);

    let pass_list: Vec<(fn(&CompilationUnit) -> Pass, fn(&Pass) -> bool)> = vec![
      (lexer_pass, |pass| {
        println!("lex check");
        match pass {
          Pass::Lexer(tokens, _) => {
            //for token in tokens {
            //  println!("{}", token);
            //}
            !tokens.is_empty()
          }
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
      (semantic_pass, |pass| {
        println!("semantic check");
        match pass {
          Pass::SemanticCheck(ast, Some(symbol_table), errors) => errors.is_empty(),
          _ => false,
        }
      }),
    ];

    let mut pass_index = 0;
    for (pass, success) in pass_list {
      let pass_result = pass(&self);
      pass_index += 1;
      if !success(&pass_result) || (max_pass.is_some() && pass_index >= max_pass.unwrap()) {
        self.passes.push(pass_result);
        //println!("fail");
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
        Pass::Lexer(_, errors) | Pass::Parser(_, errors) | Pass::SemanticCheck(_, _, errors) => {
          for error in errors {
            display_ocean_error(self, &error)
          }
        }
        _ => {}
      }
    }
  }
}

pub fn compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::from_file(filename, file_content);
  main.compile(None);
}
