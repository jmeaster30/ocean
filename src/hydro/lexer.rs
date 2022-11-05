use crate::util::errors::{OceanError, Severity};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum HydroTokenType {
  Identifier,
  Variable,
  Equal,
  LParen,
  RParen,
  LCurly,
  RCurly,
  LSquare,
  RSquare,
  Colon,
  Comma,
  Dot,
  Type,
  Keyword,
  StringLiteral,
  CharLiteral,
  BooleanLiteral,
  NumberLiteral,
  Newline,
  Error,
  EndOfInput,
}

#[derive(Clone, Debug)]
pub struct HydroToken {
  pub token_type: HydroTokenType,
  pub lexeme: String,
  pub start: usize,
  pub end: usize,
}

impl HydroToken {
  pub fn new(token_type: HydroTokenType, lexeme: String, start: usize, end: usize) -> Self {
    Self {
      token_type,
      lexeme,
      start,
      end,
    }
  }
}

impl fmt::Display for HydroToken {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "<[{:?}] '{}' {} {}>",
      self.token_type, self.lexeme, self.start, self.end
    )
  }
}

pub fn hydro_lex(input: String) -> (Vec<HydroToken>, Vec<OceanError>) {
  let input_length = input.len();
  let input_chars: Vec<_> = input.chars().collect(); // I understand both chars and this collect is not great but I am learning :)
  let mut lexeme = String::new(); //we probably don't need this here :/
  let mut index = 0;
  let mut tokens = Vec::new();
  let mut errors = Vec::new();
  while index < input_length {
    let start_index = index;
    let c = input_chars[index];
    match c {
      'A'..='Z' | 'a'..='z' | '_' => {
        index += 1;
        lexeme.push_str(&c.to_string());
        while index < input_length {
          let n = input_chars[index];
          match n {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => lexeme.push_str(&n.to_string()),
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }

        //check against every other thing it could be
        match lexeme.as_str() {
          "i8" | "i16" | "i32" | "i64" | "f32" | "f64" | "u8" | "u16" | "u32" | "u64"
          | "string" | "auto" | "bool" | "void" | "ref" | "func" => {
            tokens.push(HydroToken::new(
              HydroTokenType::Type,
              lexeme.clone(),
              start_index,
              index,
            ));
          }
          "if" | "else" | "return" | "continue" | "while" | "break" | "type" => {
            tokens.push(HydroToken::new(
              HydroTokenType::Keyword,
              lexeme.clone(),
              start_index,
              index,
            ));
          }
          "true" | "false" => {
            tokens.push(HydroToken::new(
              HydroTokenType::BooleanLiteral,
              lexeme.clone(),
              start_index,
              index,
            ));
          }
          _ => {
            tokens.push(HydroToken::new(
              HydroTokenType::Identifier,
              lexeme.clone(),
              start_index,
              index,
            ));
          }
        }

        lexeme.clear();
      }
      '-' | '0'..='9' => {
        lexeme.push_str(&c.to_string());
        index += 1;
        let mut decimal = false;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '0'..='9' => lexeme.push_str(&n.to_string()),
            '.' => {
              if decimal {
                index -= 1;
                break;
              }
              lexeme.push_str(&n.to_string());
              decimal = true;
            }
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1
        }

        if lexeme.ends_with('.') {
          lexeme.pop();
          index -= 1;
        }

        if lexeme.as_str() == "-" {
          errors.push(OceanError::LexError(
            Severity::Error,
            (start_index, index),
            "Unrecognized token".to_string(),
          ));
        } else {
          tokens.push(HydroToken::new(
            HydroTokenType::NumberLiteral,
            lexeme.clone(),
            start_index,
            index,
          ));
        }

        lexeme.clear();
      }
      '@' => {
        index += 1;
        lexeme.push_str(&c.to_string());
        while index < input_length {
          let n = input_chars[index];
          match n {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => lexeme.push_str(&n.to_string()),
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }

        tokens.push(HydroToken::new(
          HydroTokenType::Variable,
          lexeme.clone(),
          start_index,
          index,
        ));
        lexeme.clear();
      }
      '\"' | '\'' => {
        let delim = c;
        index += 1;
        let mut found_end = false;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\'' => {
              if delim == '\'' {
                found_end = true;
                break;
              } else {
                lexeme.push_str(&n.to_string())
              }
            }
            '\"' => {
              if delim == '\"' {
                found_end = true;
                break;
              } else {
                lexeme.push_str(&n.to_string())
              }
            }
            '\\' => {
              if index == input_length - 1 {
                lexeme.push_str(&n.to_string());
              } else {
                index += 1;
                let x = input_chars[index];
                match x {
                  'n' => lexeme.push_str(&"\n".to_string()),
                  'r' => lexeme.push_str(&"\r".to_string()),
                  't' => lexeme.push_str(&"\t".to_string()),
                  //need to add excape characters for octal, hex, and unicode
                  _ => lexeme.push_str(&x.to_string()),
                }
              }
            }
            _ => lexeme.push_str(&n.to_string()),
          }
          index += 1;
        }

        if !found_end {
          errors.push(OceanError::LexError(
            Severity::Error,
            (start_index, index),
            "Unending string".to_string(),
          ))
        } else if delim == '"' {
          tokens.push(HydroToken::new(
            HydroTokenType::StringLiteral,
            lexeme.clone(),
            start_index,
            index,
          ));
        } else if delim == '\'' {
          tokens.push(HydroToken::new(
            HydroTokenType::CharLiteral,
            lexeme.clone(),
            start_index,
            index,
          ));
        } else {
          panic!("{}", lexeme);
        }
        lexeme.clear();
      }
      '#' => {
        if index < input_length - 1 && input_chars[index + 1] == '/' {
          index += 1;
          lexeme.push_str(&input_chars[index].to_string());
          let mut found_end = false;
          while index < input_length - 1 {
            index += 1;
            let n = input_chars[index];
            match n {
              '/' => {
                lexeme.push_str(&n.to_string());
                if index < input_length - 1 && input_chars[index + 1] == '#' {
                  index += 1;
                  lexeme.push_str(&input_chars[index].to_string());
                  found_end = true;
                  break;
                }
              }
              _ => lexeme.push_str(&n.to_string()),
            }
          }
          if found_end {
            //tokens.push(Token::new(TokenType::Comment, lexeme.clone(), start_index, index));
          } else {
            errors.push(OceanError::LexError(
              Severity::Error,
              (start_index, index),
              "Unended comment block".to_string(),
            ));
          }
        } else {
          while index < input_length - 1 {
            index += 1;
            let n = input_chars[index];
            match n {
              '\n' => {
                index -= 1;
                break;
              }
              _ => lexeme.push_str(&n.to_string()),
            }
          }
        }
        lexeme.clear();
      }
      '.' => tokens.push(HydroToken::new(
        HydroTokenType::Dot,
        ".".to_string(),
        start_index,
        index,
      )),
      ':' => tokens.push(HydroToken::new(
        HydroTokenType::Colon,
        ":".to_string(),
        start_index,
        index,
      )),
      ',' => tokens.push(HydroToken::new(
        HydroTokenType::Comma,
        ",".to_string(),
        start_index,
        index,
      )),
      '=' => tokens.push(HydroToken::new(
        HydroTokenType::Equal,
        "=".to_string(),
        start_index,
        index,
      )),
      '(' => tokens.push(HydroToken::new(
        HydroTokenType::LParen,
        "(".to_string(),
        start_index,
        index,
      )),
      ')' => tokens.push(HydroToken::new(
        HydroTokenType::RParen,
        ")".to_string(),
        start_index,
        index,
      )),
      '[' => tokens.push(HydroToken::new(
        HydroTokenType::LSquare,
        "[".to_string(),
        start_index,
        index,
      )),
      ']' => tokens.push(HydroToken::new(
        HydroTokenType::RSquare,
        "]".to_string(),
        start_index,
        index,
      )),
      '{' => tokens.push(HydroToken::new(
        HydroTokenType::LCurly,
        "{".to_string(),
        start_index,
        index,
      )),
      '}' => tokens.push(HydroToken::new(
        HydroTokenType::RCurly,
        "}".to_string(),
        start_index,
        index,
      )),
      '\n' => tokens.push(HydroToken::new(
        HydroTokenType::Newline,
        "\n".to_string(),
        start_index,
        index,
      )),
      ' ' | '\t' | '\r' => {}
      _ => errors.push(OceanError::LexError(
        Severity::Error,
        (start_index, index),
        "Unrecognized token".to_string(),
      )),
    }
    index += 1;
  }
  tokens.push(HydroToken::new(
    HydroTokenType::EndOfInput,
    "".to_string(),
    index,
    index,
  ));
  (tokens, errors)
}
