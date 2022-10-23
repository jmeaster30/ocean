use super::errors::*;
use super::hydro::typechecker::hydro_semantic_check;
use super::lexer::*;
use super::parser::ast::*;
use super::parser::parse;
use super::semantic_analysis::symboltable::*;
use super::semantic_analysis::*;
use super::CompilationUnit;

use super::hydro::instruction::*;
use super::hydro::lexer::*;
use super::hydro::parser::*;

pub enum Pass {
  Lexer(Vec<Token>, Vec<OceanError>),
  Parser(Option<Program>, Vec<OceanError>),
  SemanticCheck(Program, Option<SymbolTable>, Vec<OceanError>),

  HydroLexer(Vec<HydroToken>, Vec<OceanError>),
  HydroParser(Vec<Instruction>, Vec<OceanError>),
  HydroSemanticCheck(Vec<Instruction>, Option<SymbolTable>, Vec<OceanError>),
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
      _ => panic!(),
    },
    None => panic!(),
  }
}

pub fn hydro_semantic_pass(comp_unit: &CompilationUnit) -> Pass {
  let last_pass = comp_unit.passes.last();
  match last_pass {
    Some(pass) => match pass {
      Pass::HydroParser(instructions, _) => {
        let (typed_instructions, symbol_table, type_errors) = hydro_semantic_check(&instructions);
        Pass::HydroSemanticCheck(typed_instructions, symbol_table, type_errors)
      }
      _ => panic!(),
    },
    None => panic!(),
  }
}
