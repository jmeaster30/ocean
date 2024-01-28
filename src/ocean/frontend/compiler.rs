use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parser::annotationparser::parse_annotations;
use crate::ocean::frontend::parser::parserphase1::parse_phase_one;
use crate::ocean::frontend::parser::parserphase2::parse_phase_two;
use crate::ocean::frontend::parser::precedencetable::PrecedenceTable;
use crate::ocean::Ocean;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::{env, fs, io};
use std::time::Instant;
use crate::ocean::frontend::ast::Program;
use crate::util::errors::Error;

impl Ocean {
  pub fn compile(file_path: &str, token_mode: &str, ast_mode: &str) -> Result<(), io::Error> {
    let now = Instant::now();
    let error_context_size = match env::var("OCEAN_ERROR_LINE_CONTEXT") {
      Ok(value) => value.parse::<usize>().unwrap(),
      Err(_) => 2,
    };

    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());

    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    match Ocean::internal_compile(file_path, &file_contents, token_mode, ast_mode) {
      Ok(_) => {
        let new_now = Instant::now();
        println!("Compilation Completed In: {:?}", new_now.duration_since(now));
        Ok(())
      }
      Err(parse_errors) => {
        let new_now = Instant::now();
        println!("Compilation Completed In: {:?}", new_now.duration_since(now));
        for error in parse_errors {
          error.display_message(file_contents.as_bytes(), &file_path.to_string(), error_context_size);
        }
        Ok(()) // TODO unsure if this is "Ok"
      }
    }
  }

  fn internal_compile(file_path: &str, file_contents: &String, token_mode: &str, ast_mode: &str) -> Result<Program, Vec<Error>> {
    let tokens = lex(&file_contents)?;

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
      Err(mut errors) => parse_errors.append(&mut errors),
    }
    match ast_mode {
      "print" => println!("{:#?}", ast),
      "file" => {
        let mut file = File::create(file_path.to_string() + ".ast").unwrap();
        file.write_all(format!("{:#?}", ast).as_bytes()).unwrap();
      }
      _ => {}
    }
    Ok(ast)
  }
}
