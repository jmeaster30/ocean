use crate::ocean::frontend::annotationparser::parse_annotations;
use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parserphase1::parse_phase_one;
use crate::ocean::frontend::parserphase2::parse_phase_two;
use crate::ocean::frontend::precedencetable::PrecedenceTable;
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

    let tokens = lex(file_contents);
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

    let mut phase_one_ast = parse_phase_one(&tokens);
    match ast_mode {
      "print" => println!("{:#?}", phase_one_ast),
      "file" => {
        let mut file = File::create(file_path.to_string() + ".ast_p1")?;
        file.write_all(format!("{:#?}", phase_one_ast).as_bytes())?;
      }
      _ => {}
    }

    parse_annotations(&mut phase_one_ast);

    let mut precedence_table = PrecedenceTable::new();
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
    precedence_table.add_binary_operator(".", 5000, 4999);

    let phase_two_ast = parse_phase_two(&mut phase_one_ast, &mut precedence_table);
    match ast_mode {
      "print" => println!("{:#?}", phase_two_ast),
      "file" => {
        let mut file = File::create(file_path.to_string() + ".ast_p2")?;
        file.write_all(format!("{:#?}", phase_two_ast).as_bytes())?;
      }
      _ => {}
    }

    Ok(())
  }
}
