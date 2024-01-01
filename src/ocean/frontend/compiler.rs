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

impl Ocean {
  pub fn compile(file_path: &str, token_mode: &str, ast_mode: &str) -> Result<(), io::Error> {
    let path = Path::new(file_path);
    println!("Compiling '{}' (absolute '{:?}' from '{:?}')", path.display(), fs::canonicalize(path), env::current_dir());

    let mut file = File::open(path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

    let tokens = match lex(&file_contents) {
      Ok(tokens) => tokens,
      Err(errors) => {
        for error in errors {
          error.display_message(file_contents.as_bytes(), &file_path.to_string());
        }
        return Ok(()); // TODO I don't know if this is really okay though hmmm
      }
    };

    match token_mode {
      "print" => {
        for token in &tokens {
          println!("{}", token)
        }
      }
      "file" => {
        let mut file = File::create(file_path.to_string() + ".tokens")?;
        for token in &tokens {
          file.write_all(format!("{}\n", token).as_bytes())?;
        }
      }
      _ => {}
    }

    let (mut ast, mut parse_errors) = parse_phase_one(&tokens);

    parse_annotations(&mut ast);

    let mut precedence_table = PrecedenceTable::new();
    precedence_table.add_prefix_operator("-", 1000);
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
    precedence_table.add_binary_operator(".", usize::MAX, usize::MAX - 1);

    match parse_phase_two(&mut ast, &mut precedence_table) {
      Ok(()) => {}
      Err(mut errors) => parse_errors.append(&mut errors),
    }
    match ast_mode {
      "print" => println!("{:#?}", ast),
      "file" => {
        let mut file = File::create(file_path.to_string() + ".ast_p2")?;
        file.write_all(format!("{:#?}", ast).as_bytes())?;
      }
      _ => {}
    }

    if !parse_errors.is_empty() {
      for error in parse_errors {
        error.display_message(file_contents.as_bytes(), &file_path.to_string());
      }
    }

    Ok(())
  }
}
