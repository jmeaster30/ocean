use super::lexer::*;
use super::parser::ast::*;
use super::parser::parse;
use super::semantic_analysis::symboltable::*;
use super::semantic_analysis::*;
use super::CompilationUnit;
use crate::util::errors::*;

pub enum Pass {
  Lexer(Vec<Token>, Vec<OceanError>),
  Parser(Option<Program>, Vec<OceanError>),
  SemanticCheck(Program, Option<SymbolTable>, Vec<OceanError>),
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
      _ => panic!(),
    },
    None => panic!(),
  }
}

pub fn semantic_pass(comp_unit: &CompilationUnit) -> Pass {
  let last_pass = comp_unit.passes.last();
  match last_pass {
    Some(pass) => match pass {
      Pass::Parser(Some(program), _) => {
        let (typed_program, symbol_table, type_errors) = semantic_check(program);
        Pass::SemanticCheck(program.clone(), symbol_table, type_errors)
      }
      _ => panic!(),
    },
    _ => panic!(),
  }
}
