use super::errors::*;
use super::lexer::*;
use super::parser::ast::*;
use super::parser::parse;
use super::CompilationUnit;

pub enum Pass {
  Lexer(Vec<Token>, Vec<OceanError>),
  Parser(Option<Program>, Vec<OceanError>),
  Check(String),
}

pub fn lexer_pass(comp_unit: &CompilationUnit) -> Pass {
  let (tokens, lexical_errors) = lex(comp_unit.file_content.clone());
  Pass::Lexer(tokens, lexical_errors)
}

pub fn parser_pass(comp_unit: &CompilationUnit) -> Pass {
  let last_pass = comp_unit.passes.last();
  match last_pass {
    Some(pass) => match pass {
      Pass::Lexer(token_stack, _) => {
        let (ast, parse_errors) = parse(&token_stack, None);
        Pass::Parser(ast, parse_errors)
      }
      _ => Pass::Parser(
        None,
        vec![OceanError::Base(
          Severity::Error,
          "Parser pass must immediately follow the lexer pass".to_string(),
        )],
      ),
    },
    None => Pass::Parser(
      None,
      vec![OceanError::Base(
        Severity::Error,
        "Parser pass must immediately follow the lexer pass".to_string(),
      )],
    ),
  }
}
