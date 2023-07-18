#![allow(
  unused_variables,
  unused_imports,
  unreachable_patterns,
  unused_assignments,
  dead_code
)]

use crate::hydro::pipeline::HydroCompilationUnit;
use crate::util::errors::{OceanError, Severity};

use super::lexer::Token;

#[derive(Clone, Debug)]
pub enum MacroContents {
  Hydro(HydroCompilationUnit),
  Unknown(String),
}

pub fn parse_macro_contents(source_token: Token) -> (MacroContents, Vec<OceanError>) {
  let mut errors = Vec::new();
  let source = source_token
    .lexeme
    .clone()
    .trim_matches('@')
    .trim()
    .to_string();
  let source_split = source.split_once(' ');

  // TODO calculate offset for better error reporting

  match source_split {
    Some((macro_type, macro_source)) => match macro_type.to_ascii_lowercase().as_str() {
      "hydro" => {
        let ast = HydroCompilationUnit::embedded(0, macro_source.to_string()).build_ast();
        (MacroContents::Hydro(ast), errors)
      }
      _ => {
        errors.push(OceanError::MacroError(
          Severity::Error,
          (source_token.start, source_token.end),
          "Unknown macro type".to_string(),
        ));
        (MacroContents::Unknown(macro_type.to_string()), errors)
      }
    },
    None => {
      errors.push(OceanError::MacroError(
        Severity::Error,
        (source_token.start, source_token.end),
        "Unknown macro type".to_string(),
      ));
      (MacroContents::Unknown("Unknown".to_string()), errors)
    }
  }
}
