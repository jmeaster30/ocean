#![allow(
  unused_variables,
  unused_imports,
  unreachable_patterns,
  unused_assignments,
  dead_code
)]

use super::{hydro::{instruction::Instruction, lexer::hydro_lex, parser::hydro_parse}, errors::{OceanError, Severity}, lexer::Token};

#[derive(Clone, Debug)]
pub enum MacroContents {
  Hydro(Vec<Instruction>),
  Unknown(String)
}

pub fn parse_macro_contents(source_token: Token) -> (MacroContents, Vec<OceanError>) {
  let mut errors = Vec::new();
  let source = source_token.lexeme.clone().trim_matches('@').trim().to_string();
  let source_split = source.split_once(' ');

  // TODO calculate offset for better error reporting
  
  match source_split {
    Some((macro_type, macro_source)) => {
      match macro_type.to_ascii_lowercase().as_str() {
        "hydro" => {
          let (tokens, mut lex_errors) = hydro_lex(macro_source.to_string());
          errors.append(&mut lex_errors);
          let instructions = hydro_parse(&tokens);
          (MacroContents::Hydro(instructions), errors)
        }
        _ => {
          errors.push(OceanError::MacroError(
            Severity::Error,
            (source_token.start, source_token.end),
            "Unknown macro type".to_string()
          ));
          (MacroContents::Unknown(macro_type.to_string()), errors)
        }
      }
    }
    None => {
      errors.push(OceanError::MacroError(
        Severity::Error,
        (source_token.start, source_token.end),
        "Unknown macro type".to_string()
      ));
      (MacroContents::Unknown("Unknown".to_string()), errors)
    }
  }
}