use crate::hydro::pipeline::HydroCompilationUnit;
use crate::ocean::parser::ast::ErrorStatement;
use crate::ocean::CompilationUnit;

use super::span::Spanned;

/*
    this was taken from the SerentiyOS/jakt repo and
    modified slightly to work how I wanted it to.
*/

#[derive(Clone, Debug)]
pub enum Severity {
  Hint,
  Warning,
  Error,
}

impl Severity {
  pub fn name(&self) -> String {
    match self {
      Severity::Hint => "Hint".to_string(),
      Severity::Warning => "Warning".to_string(),
      Severity::Error => "Error".to_string(),
    }
  }

  pub fn ansi_color_code(&self) -> String {
    match self {
      Severity::Hint => "94".to_string(),    // Bright Blue
      Severity::Warning => "33".to_string(), // Yellow
      Severity::Error => "31".to_string(),   // Red
    }
  }
}

#[derive(Debug, Clone)]
pub enum OceanError {
  Base(Severity, String),
  LexError(Severity, (usize, usize), String),
  MacroError(Severity, (usize, usize), String),
  ParseError(ErrorStatement),
  SemanticError(Severity, (usize, usize), String),
}

pub fn display_ocean_error(compilation_unit: &CompilationUnit, error: &OceanError) {
  let file_contents = compilation_unit.file_content.as_bytes();
  let file_name = &compilation_unit.filename;
  display_error(error, file_contents, file_name)
}

pub fn display_hydro_error(compilation_unit: &HydroCompilationUnit, error: &OceanError) {
  let file_contents = compilation_unit.file_content.as_bytes();
  let file_name = &compilation_unit.filename;
  display_error(error, file_contents, file_name)
}

pub fn display_error(error: &OceanError, file_contents: &[u8], file_name: &String) {
  match error {
    OceanError::Base(severity, message) => display_message(
      severity,
      message.to_string(),
      0,
      0,
      file_contents,
      file_name,
    ),
    OceanError::LexError(severity, span, message)
    | OceanError::MacroError(severity, span, message) => display_message(
      severity,
      message.to_string(),
      span.0,
      span.1,
      file_contents,
      file_name,
    ),
    OceanError::ParseError(error) => {
      let (start_offset, end_offset) = error.get_span();
      display_message(
        &error.severity,
        error.message.to_string(),
        start_offset,
        end_offset,
        file_contents,
        file_name,
      )
    }
    OceanError::SemanticError(severity, span, message) => display_message(
      severity,
      message.to_string(),
      span.0,
      span.1,
      file_contents,
      file_name,
    ),
  }
}

pub fn display_message(
  severity: &Severity,
  message: String,
  start_offset: usize,
  end_offset: usize,
  file_contents: &[u8],
  file_name: &String,
) {
  println!(
    "\u{001b}[{};1m{}: \u{001b}[95;1m{}\u{001b}[0m",
    severity.ansi_color_code(),
    severity.name(),
    message
  );

  let line_spans = line_spans(file_contents);

  let mut line_index = 0;
  let largest_line_number = line_spans.len();

  let width = format!("{}", largest_line_number).len();

  while line_index < line_spans.len() {
    if start_offset >= line_spans[line_index].0 && start_offset <= line_spans[line_index].1 {
      let column_index = start_offset - line_spans[line_index].0;
      println!(
        "{}+----[\u{001b}[{}m{}:{}:{}\u{001b}[0m]----",
        " ".repeat(width + 2),
        severity.ansi_color_code(),
        file_name,
        line_index + 1,
        column_index + 1
      );
      println!("{}|", " ".repeat(width + 2));
      if line_index > 0 {
        print_source_line(
          &severity,
          file_contents,
          line_spans[line_index - 1],
          start_offset,
          end_offset,
          line_index - 1,
          largest_line_number,
        );
      }
      print_source_line(
        &severity,
        file_contents,
        line_spans[line_index],
        start_offset,
        end_offset,
        line_index,
        largest_line_number,
      );

      print!(
        "{}|{}",
        " ".repeat(width + 2),
        " ".repeat(start_offset - line_spans[line_index].0 + 1)
      );
      println!("\u{001b}[{}m^- {}\u{001b}[0m", "96", message);

      while line_index < line_spans.len() && end_offset > line_spans[line_index].0 {
        line_index += 1;
        if line_index >= line_spans.len() {
          break;
        }
        print_source_line(
          &severity,
          file_contents,
          line_spans[line_index],
          start_offset,
          end_offset,
          line_index,
          largest_line_number,
        );
      }

      break;
    } else {
      line_index += 1
    }
  }

  println!(
    "\u{001b}[0m{}+-----{}-----",
    " ".repeat(width + 2),
    "-".repeat(file_name.len() + 4)
  );
}

fn print_source_line(
  severity: &Severity,
  file_contents: &[u8],
  file_span: (usize, usize),
  start_offset: usize,
  end_offset: usize,
  line_number: usize,
  largest_line_number: usize,
) {
  let mut index = file_span.0;

  let width = format!("{}", largest_line_number).len();

  print!(" {:<width$} | ", line_number + 1);
  while index <= file_span.1 {
    let c;
    if index < file_span.1 {
      c = file_contents[index];
    } else if start_offset == end_offset && index == start_offset {
      c = b'_';
    } else {
      c = b' ';
    }

    if (index >= start_offset && index <= end_offset)
      || (start_offset == end_offset && index == start_offset)
    {
      // In the error span
      print!(
        "\u{001b}[{}m{}\u{001b}[0m",
        severity.ansi_color_code(),
        c as char
      )
    } else {
      print!("{}", c as char)
    }
    index += 1;
  }

  println!();
}

fn line_spans(contents: &[u8]) -> Vec<(usize, usize)> {
  let mut idx = 0;
  let mut output = vec![];

  let mut start = idx;
  while idx < contents.len() {
    if contents[idx] == b'\n' {
      output.push((start, idx));
      start = idx + 1;
    }
    idx += 1;
  }
  if start < idx {
    output.push((start, idx));
  }

  output
}
