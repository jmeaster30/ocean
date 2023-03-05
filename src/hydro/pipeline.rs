use crate::util::errors::OceanError;

use super::{
  instruction::Instruction,
  lexer::{hydro_lex, HydroToken},
  parser::hydro_parse,
  symboltable::HydroSymbolTable,
  typechecker::hydro_semantic_check,
};

#[derive(Debug, Clone)]
pub struct HydroCompilationUnit {
  pub file_offset: usize,
  pub filename: String,
  pub file_content: String,
  pub dependencies: Vec<HydroCompilationUnit>,
  pub errors: Vec<OceanError>,
  pub tokens: Vec<HydroToken>,
  pub instructions: Vec<Instruction>,
  pub symbol_table: Option<HydroSymbolTable>,
  pub byte_code: Vec<char>,
}

impl HydroCompilationUnit {
  pub fn from_file(filename: String, file_content: String) -> Self {
    Self {
      file_offset: 0,
      filename,
      file_content,
      dependencies: vec![],
      errors: vec![],
      tokens: vec![],
      instructions: vec![],
      symbol_table: None,
      byte_code: vec![],
    }
  }

  pub fn embedded(file_offset: usize, file_content: String) -> Self {
    Self {
      file_offset,
      filename: "embedded".to_string(),
      file_content,
      dependencies: vec![],
      errors: vec![],
      tokens: vec![],
      instructions: vec![],
      symbol_table: None,
      byte_code: vec![],
    }
  }

  pub fn copy(&self) -> Self {
    Self {
      file_offset: self.file_offset,
      filename: self.filename.clone(),
      file_content: self.file_content.clone(),
      dependencies: self.dependencies.clone(),
      errors: self.errors.clone(),
      tokens: self.tokens.clone(),
      instructions: self.instructions.clone(),
      symbol_table: self.symbol_table.clone(),
      byte_code: self.byte_code.clone(),
    }
  }

  pub fn build_ast(&self) -> HydroCompilationUnit {
    let mut result = self.copy();

    let (tokens, mut lexical_errors) = hydro_lex(self.file_content.clone());
    result.tokens = tokens.clone();
    result.errors.append(&mut lexical_errors);

    let instructions = hydro_parse(&tokens);
    result.instructions = instructions;

    println!("{:#?}", result.instructions);

    result
  }

  pub fn typecheck_ast(&self) -> HydroCompilationUnit {
    let mut result = self.copy();

    let (typed_instructions, symbol_table, mut type_errors) =
      hydro_semantic_check(&self.instructions);
    result.instructions = typed_instructions.clone();
    result.symbol_table = symbol_table;
    result.errors.append(&mut type_errors);

    result
  }

  pub fn generate_bytecode(&self) -> HydroCompilationUnit {
    let mut result = self.copy();
    result
  }
}
