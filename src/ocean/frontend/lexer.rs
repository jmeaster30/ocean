use crate::ocean::frontend::tokentype::TokenType;
use crate::util::errors::{Error, Severity};
use crate::util::token::Token;

pub fn lex(input: &String) -> Result<Vec<Token<TokenType>>, Vec<Error>> {
  let mut errors = Vec::new();
  let input_length = input.len();
  let input_chars = input.chars().collect::<Vec<char>>();
  let mut lexeme = String::new();
  let mut index = 0;
  let mut tokens = Vec::new();
  let line_start = 1;
  let line_end = 1;
  let column_start = 1;
  let column_end = 1;
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
        let token_type = match lexeme.as_str() {
          "function" => TokenType::Function,
          "func" => TokenType::FunctionType,
          "i8" | "i16" | "i32" | "i64" | "i128" | "f32" | "f64" | "u8" | "u16" | "u32" | "u64" | "u128" | "string" | "bool" | "char" => TokenType::Type,
          "auto" | "ref" | "mut" | "lazy" => TokenType::TypePrefix,
          "if" => TokenType::If,
          "else" => TokenType::Else,
          "return" => TokenType::Return,
          "continue" => TokenType::Continue,
          "while" => TokenType::While,
          "break" => TokenType::Break,
          "loop" => TokenType::Loop,
          "union" => TokenType::Union,
          "pack" => TokenType::Pack,
          "for" => TokenType::For,
          "in" => TokenType::In,
          "is" => TokenType::Is,
          "as" => TokenType::As,
          "using" => TokenType::Using,
          "match" => TokenType::Match,
          "true" => TokenType::True,
          "false" => TokenType::False,
          "let" => TokenType::Let,
          "interface" => TokenType::Interface,
          _ => TokenType::Identifier,
        };

        tokens.push(Token::new(lexeme.clone(), token_type, (start_index, index), (line_start, line_end), (column_start, column_end)));

        lexeme.clear();
      }
      '0'..='9' => {
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

        tokens.push(Token::new(lexeme.clone(), TokenType::Number, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      '@' => {
        lexeme.push_str(&c.to_string());
        index += 1;
        if index < input_length && input_chars[index] == '@' {
          lexeme.push_str(&input_chars[index].to_string());
          index += 1;
          let mut found_end = false;
          while index < input_length {
            let n = input_chars[index];
            match n {
              '@' => {
                index += 1;
                if index < input_length && input_chars[index] == '@' {
                  lexeme.push_str(&input_chars[index - 1].to_string());
                  lexeme.push_str(&input_chars[index].to_string());
                  found_end = true;
                  break;
                }
                index -= 1;
                lexeme.push_str(&n.to_string());
              }
              _ => lexeme.push_str(&n.to_string()),
            }
            index += 1;
          }
          if !found_end {
            panic!("Unending block macro")
          } else {
            tokens.push(Token::new(lexeme.clone(), TokenType::AnnotationBlock, (start_index, index), (line_start, line_end), (column_start, column_end)));
          }
        } else {
          while index < input_length {
            let n = input_chars[index];
            match n {
              '\r' | '\n' => break,
              _ => lexeme.push_str(&n.to_string()),
            }
            index += 1;
          }
          tokens.push(Token::new(lexeme.clone(), TokenType::Annotation, (start_index, index), (line_start, line_end), (column_start, column_end)));
        }
        lexeme.clear();
      }
      '\"' | '\'' | '`' => {
        let delim = c;
        lexeme.push_str(&c.to_string());
        index += 1;
        let mut found_end = false;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\'' => {
              if delim == '\'' {
                found_end = true;
                lexeme.push_str(&n.to_string());
                break;
              } else {
                lexeme.push_str(&n.to_string())
              }
            }
            '\"' => {
              if delim == '\"' {
                found_end = true;
                lexeme.push_str(&n.to_string());
                break;
              } else {
                lexeme.push_str(&n.to_string())
              }
            }
            '`' => {
              if delim == '`' {
                found_end = true;
                lexeme.push_str(&n.to_string());
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
          tokens.push(Token::new(lexeme.clone(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end)));
          errors.push(Error::new(Severity::Error, (start_index, index), "Unending string.".to_string()));
        } else if delim == '`' {
          tokens.push(Token::new(lexeme.clone(), TokenType::InterpolatedString, (start_index, index), (line_start, line_end), (column_start, column_end)))
        } else {
          tokens.push(Token::new(lexeme.clone(), TokenType::String, (start_index, index), (line_start, line_end), (column_start, column_end)));
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
          if !found_end {
            tokens.push(Token::new(lexeme.clone(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end)));
            errors.push(Error::new(Severity::Warning, (start_index, index), "Unending comment block.".to_string()));
          } else {
            tokens.push(Token::new(lexeme.clone(), TokenType::Comment, (start_index, index), (line_start, line_end), (column_start, column_end)));
          }
        } else {
          while index < input_length - 1 {
            index += 1;
            let n = input_chars[index];
            match n {
              '\r' | '\n' => {
                index -= 1;
                break;
              }
              _ => lexeme.push_str(&n.to_string()),
            }
          }
          tokens.push(Token::new(lexeme.clone(), TokenType::Comment, (start_index, index), (line_start, line_end), (column_start, column_end)));
        }

        lexeme.clear();
      }
      ';' => {
        index += 1;
        tokens.push(Token::new(";".to_string(), TokenType::Semicolon, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' => {
        lexeme.push_str(&input_chars[index].to_string());
        index += 1;
        while index < input_length - 1 && lexeme.len() < 3 {
          let n = input_chars[index];
          match n {
            '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' => {
              lexeme.push_str(&n.to_string());
            }
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }

        if index - start_index + 1 == 4 {
          index -= 1;
        }

        let token_type = match lexeme.as_str() {
          "." => TokenType::Dot,
          ":" => TokenType::Colon,
          "->" => TokenType::Arrow,
          "=>" => TokenType::DoubleArrow,
          "..." => TokenType::Spread,
          ";" => TokenType::Semicolon,
          _ => TokenType::Symbol,
        };
        tokens.push(Token::new(lexeme.clone(), token_type, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      ',' => tokens.push(Token::new(",".to_string(), TokenType::Comma, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '\r' | '\n' => {
        lexeme.push_str(&input_chars[index].to_string());
        index += 1;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\r' | '\n' => {
              lexeme.push_str(&n.to_string());
            }
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }

        tokens.push(Token::new(lexeme.clone(), TokenType::Newline, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      '(' => tokens.push(Token::new("(".to_string(), TokenType::LeftParen, (start_index, index), (line_start, line_end), (column_start, column_end))),
      ')' => tokens.push(Token::new(")".to_string(), TokenType::RightParen, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '[' => tokens.push(Token::new("[".to_string(), TokenType::LeftSquare, (start_index, index), (line_start, line_end), (column_start, column_end))),
      ']' => tokens.push(Token::new("]".to_string(), TokenType::RightSquare, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '{' => tokens.push(Token::new("{".to_string(), TokenType::LeftCurly, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '}' => tokens.push(Token::new("}".to_string(), TokenType::RightCurly, (start_index, index), (line_start, line_end), (column_start, column_end))),
      ' ' | '\t' => {}
      _ => {
        tokens.push(Token::new(input_chars[index].to_string(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end)));
        errors.push(Error::new(Severity::Error, (start_index, index), "Unknown token.".to_string()));
      }
    }
    index += 1;
  }

  tokens.push(Token::new("".to_string(), TokenType::EndOfInput, (index, index), (line_start, line_end), (column_start, column_end)));

  if errors.is_empty() {
    Ok(tokens)
  } else {
    Err(errors)
  }
}
