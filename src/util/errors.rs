/*
    this was taken from the SerentiyOS/jakt repo and
    modified slightly to work how I wanted it to.
*/

use ocean_macros::New;

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

#[derive(Debug, Clone, New)]
pub struct ErrorMetadata {
  #[default(Vec::new())]
  suggestions: Vec<String>,
  #[default(Vec::new())]
  extra_code_spans: Vec<((usize, usize), String)>
}

impl ErrorMetadata {
  pub fn suggestion(mut self, message: String) -> Self {
    self.suggestions.push(message);
    self
  }

  pub fn extra_highlighted_info(mut self, span: (usize, usize), message: String) -> Self {
    self.extra_code_spans.push((span, message));
    self
  }
}

#[derive(Debug, Clone, New)]
pub struct Error {
  pub severity: Severity,
  pub span: (usize, usize),
  pub message: String,
  #[default(None)]
  pub metadata: Option<ErrorMetadata>,
}

impl Error {
  pub fn new_with_metadata(severity: Severity, span: (usize, usize), message: String, metadata: ErrorMetadata) -> Self {
    Self {
      severity,
      span,
      message,
      metadata: Some(metadata),
    }
  }

  pub fn display_message(&self, file_contents: &[u8], file_name: &String, context: usize) {
    eprintln!("\u{001b}[{};1m{}: \u{001b}[95;1m{}\u{001b}[0m", self.severity.ansi_color_code(), self.severity.name(), self.message);

    let line_spans = Error::line_spans(file_contents);

    let mut line_index = 0;
    let largest_line_number = line_spans.len();

    let width = format!("{}", largest_line_number).len();

    while line_index < line_spans.len() {
      if self.span.0 >= line_spans[line_index].0 && self.span.0 <= line_spans[line_index].1 {
        let column_index = self.span.0 - line_spans[line_index].0;
        eprintln!("{}+----[\u{001b}[{}m{}:{}:{}\u{001b}[0m]----", " ".repeat(width + 2), self.severity.ansi_color_code(), file_name, line_index + 1, column_index + 1);
        eprintln!("{}|", " ".repeat(width + 2));
        let mut offset = context;
        while offset > 0 {
          if line_index < offset {
            offset -= 1;
            continue
          }
          Error::print_source_line(&self.severity, file_contents, line_spans[line_index - offset], self.span.0, self.span.1, line_index - offset, largest_line_number);
          offset -= 1;
        }
        Error::print_source_line(&self.severity, file_contents, line_spans[line_index], self.span.0, self.span.1, line_index, largest_line_number);

        eprint!("{}|{}", " ".repeat(width + 2), " ".repeat(self.span.0 - line_spans[line_index].0 + 1));
        eprintln!("\u{001b}[{}m^- {}\u{001b}[0m", "96", self.message);

        while line_index < line_spans.len() && self.span.1 > line_spans[line_index].0 {
          line_index += 1;
          if line_index >= line_spans.len() {
            break;
          }
          Error::print_source_line(&self.severity, file_contents, line_spans[line_index], self.span.0, self.span.1, line_index, largest_line_number);
        }

        let mut offset = 1;
        while line_index + offset < largest_line_number && offset <= context {
          Error::print_source_line(&self.severity, file_contents, line_spans[line_index + offset], self.span.0, self.span.1, line_index + offset, largest_line_number);
          offset += 1;
        }

        break;
      } else {
        line_index += 1
      }
    }

    eprintln!("\u{001b}[0m{}+-----{}-----", " ".repeat(width + 2), "-".repeat(file_name.len() + 4));


  }

  fn print_source_line(severity: &Severity, file_contents: &[u8], file_span: (usize, usize), start_offset: usize, end_offset: usize, line_number: usize, largest_line_number: usize) {
    let mut index = file_span.0;

    let width = format!("{}", largest_line_number).len();

    eprint!(" {:<width$} | ", line_number + 1);
    while index <= file_span.1 {
      let c;
      if index < file_span.1 {
        c = file_contents[index];
      } else if start_offset == end_offset && index == start_offset {
        c = b'_';
      } else {
        c = b' ';
      }

      if (index >= start_offset && index <= end_offset) || (start_offset == end_offset && index == start_offset) {
        // In the error span
        eprint!("\u{001b}[{}m{}\u{001b}[0m", severity.ansi_color_code(), c as char)
      } else {
        eprint!("{}", c as char)
      }
      index += 1;
    }

    eprintln!();
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
}
