use super::errors::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
  EndOfInput,
  Error,
  Comment,
  Macro,
  String,
  InterpolatedString,
  Number,
  Type,
  Identifier,
  LParen,
  RParen,
  LSquare,
  RSquare,
  LCurly,
  RCurly,
  Keyword,
  Symbol,
  Dot,
  Comma,
  Colon,
  Arrow,
  Underscore,
  SemiColon
}

#[derive(Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub start: usize,
  pub end: usize
}

impl Token {
  pub fn new(token_type: TokenType, lexeme: String, start: usize, end: usize) -> Token {
    Token {token_type, lexeme, start, end}
  }

  pub fn print(&self) {
    print!("<[{:?}] '{}' {} {}>", self.token_type, self.lexeme, self.start, self.end);
  }
}

pub struct TokenStack {
  token_vec: Vec<Token>,
  index: usize
}

impl TokenStack {
  pub fn push(&mut self, token: Token) {
    self.token_vec.push(token);
  }

  pub fn peek(&self) -> Token {
    if self.index >= self.token_vec.len() {
      Token::new(TokenType::EndOfInput, "EOI".to_string(),usize::MAX, usize::MAX)
    } else {
      self.token_vec[self.index].clone()
    }
  }

  pub fn consume(&mut self) -> Token {
    let t = if self.index >= self.token_vec.len() { 
      Token::new(TokenType::EndOfInput, "EOI".to_string(), usize::MAX, usize::MAX) 
    } else { 
      self.token_vec[self.index].clone() 
    };
    self.index += 1;
    t
  }

  pub fn is_empty(&self) -> bool {
    self.token_vec.len() == self.index
  }

  pub fn iterable(&self) -> Vec<Token> {
    self.token_vec[self.index..self.token_vec.len()].to_vec()
  }

  pub fn new() -> TokenStack {
    TokenStack { token_vec: Vec::new(), index: 0 }
  }
}

pub fn lex(input: String) -> (TokenStack, Vec<OceanError>) {
  let input_length = input.len();
  let input_chars: Vec<_> = input.chars().collect(); // I understand both chars and this collect is not great but I am learning :)
  let mut lexeme = String::new(); //we probably don't need this here :/
  let mut index = 0;
  let mut tokens = TokenStack::new();
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
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_'   => lexeme.push_str(&n.to_string()),
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }

        //check against every other thing it could be
        match lexeme.as_str() {
          "_" => tokens.push(Token::new(TokenType::Underscore, lexeme.clone(), start_index, index)),
          "i8" | "i16" | "i32" | "i64" | "f32" | "f64" |
          "u8" | "u16" | "u32" | "u64" | "string" | "auto" |
          "bool" | "func" | "void" | "ref" | "lazy" |
          "optional" | "comp" => {
            tokens.push(Token::new(TokenType::Type, lexeme.clone(), start_index, index));
          }
          "if" | "else" | "return" | "continue" |
          "break" | "loop" | "enum" | "pack" |
          "switch" | "default" | "let" | "cast" | 
          "for" | "in" | "as" | "use" => {
            tokens.push(Token::new(TokenType::Keyword, lexeme.clone(), start_index, index));
          }
          _ => {
            tokens.push(Token::new(TokenType::Identifier, lexeme.clone(), start_index, index));
          }
        }

        lexeme.clear();
      },
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

        tokens.push(Token::new(TokenType::Number, lexeme.clone(), start_index, index));
        lexeme.clear();
      },
      '\"' | '\'' | '`' => {
        let delim = c;
        index += 1;
        let mut found_end = false;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\'' => if delim == '\'' {
                found_end = true;
                index += 1;
                break 
              } 
              else 
              { 
                lexeme.push_str(&n.to_string()) 
              },
            '\"' => if delim == '\"' {
                found_end = true;
                index += 1;
                break 
              } 
              else 
              { 
                lexeme.push_str(&n.to_string()) 
              },
            '`'  => if delim == '`' {
                found_end = true;
                index += 1;
                break 
              } 
              else 
              { 
                lexeme.push_str(&n.to_string()) 
              },
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
                  _ => lexeme.push_str(&x.to_string())
                }
              }
            }
            _ => lexeme.push_str(&n.to_string())
          }
          index += 1;
        }
        
        if !found_end {
          errors.push(OceanError::LexError(
            Severity::Error,
            Token::new(TokenType::String, lexeme.clone(), start_index, index),
            "Unending string".to_string()
          ))
        } else if (delim == '`') {
          tokens.push(Token::new(TokenType::InterpolatedString, lexeme.clone(), start_index, index))
        } else {
          tokens.push(Token::new(TokenType::String, lexeme.clone(), start_index, index));
        }
        lexeme.clear();
      }
      '#' => {
        index += 1;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\n' => {
              index -= 1;
              break;
            }
            _ => lexeme.push_str(&n.to_string())
          }
          index += 1;
        }
        tokens.push(Token::new(TokenType::Comment, lexeme.clone(), start_index, index));
        lexeme.clear();
      },
      '@' => {
        index += 1;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '\n' => {
              index -= 1;
              break;
            }
            _ => lexeme.push_str(&n.to_string())
          }
          index += 1;
        }
        tokens.push(Token::new(TokenType::Macro, lexeme.clone(), start_index, index));
        lexeme.clear();
      },
      ':' | '>' | '<' | '?' | '.' | '/' | ';' |
      '~' | '!' | '$' | '%' | '&' | '^' | '*' |
      '-' | '+' | '=' | '|' | '\\'| ','  => {
        let start = index;
        index += 1;
        lexeme.push_str(&c.to_string());
        while index < start + 3 && index < input_length {
          let n = input_chars[index];
          match n {
            ':' | '>' | '<' | '?' | '.' | '/' | ';' |
            '~' | '!' | '$' | '%' | '&' | '^' | '*' |
            '-' | '+' | '=' | '|' | '\\'| ',' => lexeme.push_str(&n.to_string()),
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1;
        }
        
        //check all the other things it could be
        match lexeme.as_str() {
          "." => tokens.push(Token::new(TokenType::Dot, lexeme.clone(), start_index, index)),
          "," => tokens.push(Token::new(TokenType::Comma, lexeme.clone(), start_index, index)),
          ":" => tokens.push(Token::new(TokenType::Colon, lexeme.clone(), start_index, index)),
          "->" => tokens.push(Token::new(TokenType::Arrow, lexeme.clone(), start_index, index)),
          ";" => tokens.push(Token::new(TokenType::SemiColon, lexeme.clone(), start_index, index)),
          _ => tokens.push(Token::new(TokenType::Symbol, lexeme.clone(), start_index, index))
        }

        lexeme.clear();
      },
      '(' => tokens.push(Token::new(TokenType::LParen, "(".to_string(), start_index, index)),
      ')' => tokens.push(Token::new(TokenType::RParen, ")".to_string(), start_index, index)),
      '[' => tokens.push(Token::new(TokenType::LSquare, "[".to_string(), start_index, index)),
      ']' => tokens.push(Token::new(TokenType::RSquare, "]".to_string(), start_index, index)),
      '{' => tokens.push(Token::new(TokenType::LCurly, "{".to_string(), start_index, index)),
      '}' => tokens.push(Token::new(TokenType::RCurly, "}".to_string(), start_index, index)),
      ' ' | '\t' | '\r' | '\n' => {},
      _ => errors.push(OceanError::LexError(
        Severity::Error,
        Token::new(TokenType::Error, c.to_string(), start_index, index), 
        "Unrecognized token".to_string()))
    }
    index += 1;
  }
  (tokens, errors)
}
