use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parser::annotationparser::parse_annotations;
use crate::ocean::frontend::parser::parserphase1::parse_phase_one;
use crate::ocean::frontend::parser::parserphase2::parse_phase_two;
use crate::ocean::frontend::parser::precedencetable::PrecedenceTable;
use crate::ocean::Ocean;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use crate::ocean::frontend::ast::Program;
use crate::ocean::frontend::compilationunit::CompilationUnit;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::ocean::frontend::semanticanalysis::usingpass::{UsingPass, UsingPassContext};
use crate::util::errors::{Error, Severity};

impl Ocean {
  pub fn compile(file_path: &str, token_mode: &str, ast_mode: &str) -> CompilationUnit {
    let now = Instant::now();
    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());

    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
      Ok(_) => {}
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };

    let (program, errors) = Ocean::internal_compile(file_path, &file_contents, token_mode, ast_mode, None);
    let new_now = Instant::now();
    println!("Compilation Completed In: {:?}", new_now.duration_since(now));
    CompilationUnit::program(file_path.to_string(), program, errors)
  }

  pub fn compile_using(file_path: &str, using_context: Rc<RefCell<UsingPassContext>>) -> CompilationUnit {
    let now = Instant::now();

    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());

    let mut file = match File::open(path) {
      Ok(file) => file,
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
      Ok(_) => {}
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };

    let (program, errors) = Ocean::internal_compile(file_path, &file_contents, "", "", Some(using_context.clone()));
    let new_now = Instant::now();
    println!("Compilation Completed In: {:?}", new_now.duration_since(now));
    CompilationUnit::program(file_path.to_string(), program, errors)
  }

  fn internal_compile(file_path: &str, file_contents: &String, token_mode: &str, ast_mode: &str, using_context: Option<Rc<RefCell<UsingPassContext>>>) -> (Program, Vec<Error>) {
    let mut errors = Vec::new();
    let (tokens, mut lex_errors) = lex(&file_contents);
    errors.append(&mut lex_errors);

    match token_mode {
      "print" => {
        for token in &tokens {
          println!("{}", token)
        }
      }
      "file" => {
        let mut file = File::create(file_path.to_string() + ".tokens").unwrap();
        for token in &tokens {
          file.write_all(format!("{}\n", token).as_bytes()).unwrap();
        }
      }
      _ => {}
    }

    let (mut ast, mut parse_errors) = parse_phase_one(&tokens);
    errors.append(&mut parse_errors);

    let context = match using_context {
      Some(context) => context,
      None => Rc::new(RefCell::new(UsingPassContext::new()))
    };
    context.borrow_mut().start_using(file_path.to_string(), (0, 0)).unwrap();

    let mut using_errors = ast.analyze_using(SymbolTable::global_scope(file_path.to_string()), context.clone());
    errors.append(&mut using_errors);

    context.borrow_mut().stop_using();

    parse_annotations(&mut ast);

    let mut precedence_table = PrecedenceTable::new();
    precedence_table.add_prefix_operator("-", 1000);
    precedence_table.add_prefix_operator("!", 1000);
    precedence_table.add_postfix_operator("!", 80);
    precedence_table.add_binary_operator("=", 0, 1);
    precedence_table.add_binary_operator("&&", 10, 11);
    precedence_table.add_binary_operator("||", 10, 11);
    precedence_table.add_binary_operator("==", 20, 21);
    precedence_table.add_binary_operator("!=", 20, 21);
    precedence_table.add_binary_operator("<", 30, 31);
    precedence_table.add_binary_operator(">", 30, 31);
    precedence_table.add_binary_operator("<=", 30, 31);
    precedence_table.add_binary_operator(">=", 30, 31);
    precedence_table.add_binary_operator("+", 40, 41);
    precedence_table.add_binary_operator("-", 40, 41);
    precedence_table.add_binary_operator("*", 50, 51);
    precedence_table.add_binary_operator("/", 50, 51);
    precedence_table.add_binary_operator("%", 50, 51);
    precedence_table.add_binary_operator("is", 1_000_000, 2);
    precedence_table.add_binary_operator(".", usize::MAX, usize::MAX - 1);

    match parse_phase_two(&mut ast, &mut precedence_table) {
      Ok(()) => {}
      Err(mut pp2_errors) => errors.append(&mut pp2_errors),
    }
    match ast_mode {
      "print" => println!("{:#?}", ast),
      "file" => {
        let mut file = File::create(file_path.to_string() + ".ast").unwrap();
        file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
      }
      _ => {}
    }
    (ast, errors)
  }
}
