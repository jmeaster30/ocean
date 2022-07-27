use super::errors::*;
use super::lexer::*;
use super::parser::ast::*;
use super::parser::parse;
use super::CompilationUnit;

use super::hydro::instruction::*;
use super::hydro::lexer::*;
use super::hydro::parser::*;

pub enum Pass {
  Lexer(Vec<Token>, Vec<OceanError>),
  Parser(Option<Program>, Vec<OceanError>),
  HydroLexer(Vec<HydroToken>, Vec<HydroError>),
  HydroParser(Vec<Instruction>, Vec<HydroError>),
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

pub fn hydro_lexer_pass(comp_unit: &CompilationUnit) -> Pass {
  let (tokens, lexical_errors) = hydro_lex(comp_unit.file_content.clone());
  Pass::HydroLexer(tokens, lexical_errors)
}

pub fn hydro_parser_pass(comp_unit: &CompilationUnit) -> Pass {
  let last_pass = comp_unit.passes.last();
  match last_pass {
    Some(pass) => match pass {
      Pass::HydroLexer(token_stack, _) => {
        let instructions = hydro_parse(&token_stack);
        Pass::HydroParser(instructions, Vec::new())
      }
      _ => Pass::HydroParser(
        Vec::new(),
        vec![HydroError::Base(
          Severity::Error,
          "Parser pass must immediately follow the lexer pass".to_string(),
        )],
      ),
    },
    None => Pass::HydroParser(
      Vec::new(),
      vec![HydroError::Base(
        Severity::Error,
        "Parser pass must immediately follow the lexer pass".to_string(),
      )],
    ),
  }
}
