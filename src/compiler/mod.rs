#![allow(
  unused_variables,
  unused_imports,
  unreachable_patterns,
  unused_assignments,
  dead_code
)]

pub mod errors;
pub mod hydro;
pub mod lexer;
pub mod parser;
pub mod passes;
pub mod semantic_analysis;

use self::errors::*;
use self::hydro::lexer::*;
use self::lexer::*;
use self::passes::*;

pub struct CompilationUnit {
  pub is_hydro: bool,
  pub filename: String,
  pub file_content: String,
  pub dependencies: Vec<CompilationUnit>,
  pub passes: Vec<Pass>,
}

impl CompilationUnit {
  pub fn ocean(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit {
      is_hydro: false,
      filename,
      file_content,
      dependencies: Vec::new(),
      passes: Vec::new(),
    }
  }

  pub fn hydro(filename: String, file_content: String) -> CompilationUnit {
    CompilationUnit {
      is_hydro: true,
      filename,
      file_content,
      dependencies: Vec::new(),
      passes: Vec::new(),
    }
  }

  pub fn compile(&mut self, max_pass: Option<i32>) {
    if self.is_hydro {
      self.hydro_compile(max_pass);
    } else {
      self.ocean_compile(max_pass);
    }
  }

  fn hydro_compile(&mut self, max_pass: Option<i32>) {
    println!("Compiling (hydro) '{}'...", self.filename);

    let pass_list: Vec<(fn(&CompilationUnit) -> Pass, fn(&Pass) -> bool)> = vec![
      (hydro_lexer_pass, |pass| {
        println!("hydro lex check");
        match pass {
          Pass::HydroLexer(tokens, _) => {
            for token in tokens {
              println!("{}", token);
            }
            !tokens.is_empty()
          }
          _ => false,
        }
      }),
      (hydro_parser_pass, |pass| {
        println!("hydro parse check");
        match pass {
          Pass::HydroParser(insts, _) => {
            println!("{:#?}", insts);
            true
          }
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
        println!("fail");
        break;
      }
      self.passes.push(pass_result);
    }

    self.print_errors();

    println!("Good :)");
  }

  fn ocean_compile(&mut self, max_pass: Option<i32>) {
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
            display_error(self, &error)
          }
        }
        _ => {}
      }
    }
  }
}

pub fn ocean_compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::ocean(filename, file_content);
  main.compile(None);
}

pub fn hydro_compile(filename: String, file_content: String) {
  let mut main = CompilationUnit::hydro(filename, file_content);
  main.compile(None);
}
