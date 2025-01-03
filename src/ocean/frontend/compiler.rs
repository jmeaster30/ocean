use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parser::parserphase::parse;
use crate::ocean::frontend::parser::precedencetable::PrecedenceTable;
use crate::ocean::Ocean;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use crate::ocean::frontend::compilationunit::ast::AstNodes;
use crate::ocean::frontend::compilationunit::CompilationUnit;
use crate::ocean::frontend::compilationunit::token::tokens::Tokens;
use crate::util::errors::{Error, Severity};
use crate::util::cli_args::DebugOutputMode;

impl Ocean {
  pub fn compile(file_path: &str, token_mode: DebugOutputMode, ast_mode: DebugOutputMode) -> CompilationUnit {
    let now = Instant::now();
    let path = match Path::new(file_path).canonicalize() {
      Ok(path) => path,
      Err(error) => {
        println!("Could not canonicalize path '{}' from current directory '{:?}' ({})", file_path, env::current_dir(), error);
        return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
      }
    };
    println!("Compiling '{}'...", path.display());
    let project_root = path.parent().unwrap();

    let mut file = match File::open(path.clone()) {
      Ok(file) => file,
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };
    let mut file_contents = String::new();
    match file.read_to_string(&mut file_contents) {
      Ok(_) => {}
      Err(error) => return CompilationUnit::errored(file_path.to_string(), Error::new(Severity::Error, (0, 0), error.to_string()))
    };

    let (tokens, ast_nodes, dependencies, errors) = Ocean::internal_compile(project_root.display().to_string(), file_path, &file_contents, token_mode, ast_mode);
    let new_now = Instant::now();
    println!("Compilation of '{}' completed in: {:?}", path.display(), new_now.duration_since(now));
    CompilationUnit::program(file_path.to_string(), tokens, ast_nodes, dependencies, errors)
  }

  fn internal_compile(project_root: String, file_path: &str, file_contents: &String, token_mode: DebugOutputMode, ast_mode: DebugOutputMode) -> (Tokens, AstNodes, Vec<Rc<RefCell<CompilationUnit>>>, Vec<Error>) {
    let mut errors = Vec::new();
    let (tokens, mut lex_errors) = lex(&file_contents);
    errors.append(&mut lex_errors);

    match token_mode {
      DebugOutputMode::Print => {
        for token in &tokens {
          println!("{}", token)
        }
      }
      DebugOutputMode::File => {
        let mut file = File::create(file_path.to_string() + ".tokens").unwrap();
        for token in &tokens {
          file.write_all(format!("{}\n", token).as_bytes()).unwrap();
        }
      }
      _ => {}
    }

    let (mut ast, mut parse_errors) = parse(&tokens);
    errors.append(&mut parse_errors);

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
    
    match ast_mode {
      DebugOutputMode::Print => println!("{:#?}", ast),
      DebugOutputMode::File => {
        let mut file = File::create(file_path.to_string() + ".ast").unwrap();
        file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
      }
      _ => {}
    }
    (tokens, ast, Vec::new(), errors)
  }
}

fn append_errors(errors: &mut Vec<Error>, mut new_errors: Vec<Error>) {
  errors.append(&mut new_errors);
}
