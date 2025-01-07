use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
use crate::ocean::frontend::compilationunit::token::tokens::Tokens;
use crate::util::errors::{Error, Severity};
use crate::util::string_map::StringMap;
use crate::util::token::Token;

pub fn lex(input: &Vec<char>) -> (Tokens, Vec<Error>) {
  let mut reserved_symbols_map = StringMap::new();
  reserved_symbols_map.insert(".", TokenType::Dot);
  reserved_symbols_map.insert(":", TokenType::Colon);
  reserved_symbols_map.insert("->", TokenType::Arrow);
  reserved_symbols_map.insert("=>", TokenType::DoubleArrow);
  reserved_symbols_map.insert("...", TokenType::Spread);
  reserved_symbols_map.insert(";", TokenType::Semicolon);

  let mut reserved_keywords_map = StringMap::new();
  reserved_keywords_map.insert("function", TokenType::Function);
  reserved_keywords_map.insert("func", TokenType::FunctionType);
  reserved_keywords_map.insert("i8", TokenType::I8);
  reserved_keywords_map.insert("i16", TokenType::I16);
  reserved_keywords_map.insert("i32", TokenType::I32);
  reserved_keywords_map.insert("i64", TokenType::I64);
  reserved_keywords_map.insert("i128", TokenType::I128);
  reserved_keywords_map.insert("u8", TokenType::U8);
  reserved_keywords_map.insert("u16", TokenType::U16);
  reserved_keywords_map.insert("u32", TokenType::U32);
  reserved_keywords_map.insert("u64", TokenType::U64);
  reserved_keywords_map.insert("u128", TokenType::U128);
  reserved_keywords_map.insert("f32", TokenType::F32);
  reserved_keywords_map.insert("f64", TokenType::F64);
  reserved_keywords_map.insert("auto", TokenType::Auto);
  reserved_keywords_map.insert("ref", TokenType::Ref);
  reserved_keywords_map.insert("mut", TokenType::Mut);
  reserved_keywords_map.insert("lazy", TokenType::Lazy);
  reserved_keywords_map.insert("if", TokenType::If);
  reserved_keywords_map.insert("else", TokenType::Else);
  reserved_keywords_map.insert("return", TokenType::Return);
  reserved_keywords_map.insert("continue", TokenType::Continue);
  reserved_keywords_map.insert("while", TokenType::While);
  reserved_keywords_map.insert("break", TokenType::Break);
  reserved_keywords_map.insert("loop", TokenType::Loop);
  reserved_keywords_map.insert("union", TokenType::Union);
  reserved_keywords_map.insert("pack", TokenType::Pack);
  reserved_keywords_map.insert("for", TokenType::For);
  reserved_keywords_map.insert("in", TokenType::In);
  reserved_keywords_map.insert("is", TokenType::Is);
  reserved_keywords_map.insert("as", TokenType::As);
  reserved_keywords_map.insert("using", TokenType::Using);
  reserved_keywords_map.insert("match", TokenType::Match);
  reserved_keywords_map.insert("true", TokenType::True);
  reserved_keywords_map.insert("false", TokenType::False);
  reserved_keywords_map.insert("let", TokenType::Let);
  reserved_keywords_map.insert("interface", TokenType::Interface);

  let mut errors = Vec::new();
  let input_length = input.len();
  //let mut lexeme = String::new();
  let mut index = 0;
  let mut tokens = Tokens::new();
  let line_start = 1;
  let line_end = 1;
  let column_start = 1;
  let column_end = 1;
  let mut trivia = Vec::new();
  while index < input_length {
    let start_index = index;
    let c = input[index];
    println!("lexing character: {}", c);
    match c {
      'A'..='Z' | 'a'..='z' | '_' => {
        index += 1;
        while index < input_length {
          match input[index] {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => index += 1,
            _ => break,
          }
        }

        //check against every other thing it could be
        let token_type = match reserved_keywords_map.get_by_char_slice(&input[start_index..index]) {
          Some(token_type) => *token_type,
          None => TokenType::Identifier
        };
        println!("TokenType {:?} start {} end {}", token_type, start_index, index);

        tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), token_type, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      }
      '0'..='9' => {
        index += 1;
        let mut decimal = false;
        while index < input_length {
          let n = input[index];
          match n {
            '0'..='9' => index += 1,
            '.' if !decimal => {
              decimal = true;
              index += 1;
            }
            _ => break,
          }
        }

        if input[index - 1] == '.' {
          index -= 1;
        }

        tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::Number, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      }
      '@' => {
        index += 1;
        if index < input_length && input[index] == '@' {
          index += 1;
          let mut found_end = false;
          while index < input_length {
            let n = input[index];
            match n {
              '@' => {
                index += 1;
                if index < input_length && input[index] == '@' {
                  found_end = true;
                  break;
                }
              }
              _ => index += 1,
            }
          }
          if !found_end {
            panic!("Unending block macro")
          } else {
            tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::AnnotationBlock, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
          }
        } else {
          while index < input_length {
            match input[index] {
              'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => index += 1,
              _ => break,
            }
          }
          tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::Annotation, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        }
        trivia.clear();
      }
      '\"' | '\'' | '`' => {
        let delim = c;
        index += 1;
        let mut found_end = false;
        while index < input_length {
          match input[index] {
            '\'' if delim == '\'' => {
              found_end = true;
              index += 1;
              break;
            }
            '\"' if delim == '\"' => {
              found_end = true;
              index += 1;
              break;
            }
            '`' if delim == '`' => {
              found_end = true;
              index += 1;
              break;
            }
            '\\' => index += if index != input_length - 1 { 2 } else { 1 },
            _ => index += 1,
          }
        }

        if !found_end {
          tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
          errors.push(Error::new(Severity::Error, (start_index, index), "Unending string.".to_string()));
        } else if delim == '`' {
          tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::InterpolatedString, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()))
        } else {
          tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), TokenType::String, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        }
        trivia.clear();
      }
      '#' => {
        if index < input_length - 1 && input[index + 1] == '/' {
          index += 1;
          let mut found_end = false;
          while index < input_length - 1 {
            index += 1;
            match input[index] {
              '/' if index < input_length - 1 && input[index + 1] == '#' => {
                index += 1;
                found_end = true;
                break;
              }
              _ => {},
            }
          }
          if !found_end {
            trivia.push(Token::new(input[start_index..index].iter().collect(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end)));
            errors.push(Error::new(Severity::Warning, (start_index, index), "Unending comment block.".to_string()));
          } else {
            trivia.push(Token::new(input[start_index..index].iter().collect(), TokenType::Comment, (start_index, index), (line_start, line_end), (column_start, column_end)));
          }
        } else {
          while index < input_length - 1 {
            index += 1;
            match input[index] {
              '\r' | '\n' => {
                index -= 1;
                break;
              }
              _ => {},
            }
          }
          trivia.push(Token::new(input[start_index..index].iter().collect(), TokenType::Comment, (start_index, index), (line_start, line_end), (column_start, column_end)));
        }
      }
      ';' => {
        index += 2;
        tokens.push(Token::new_with_trivia(";".to_string(), TokenType::Semicolon, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      }
      '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' => {
        index += 1;
        while index < input_length - 1 && index - start_index < 3 {
          match input[index] {
            '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' => index += 1,
            _ => break
          }
        }

        //if index - start_index + 1 == 4 {
        //  index -= 1;
        //}

        let token_type = match reserved_symbols_map.get_by_char_slice(&input[start_index..index]) {
          Some(token_type) => *token_type,
          None => TokenType::Symbol
        };
        tokens.push(Token::new_with_trivia(input[start_index..index].iter().collect(), token_type, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      }
      ',' => {
        index += 1;
        tokens.push(Token::new_with_trivia(",".to_string(), TokenType::Comma, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      ' ' | '\t' | '\r' | '\n' => {
        index += 1;
        while index < input_length {
          let n = input[index];
          match n {
            ' ' | '\t' | '\r' | '\n' => index += 1,
            _ => break,
          }
        }

        trivia.push(Token::new(input[start_index..index].iter().collect(), TokenType::Whitespace, (start_index, index), (line_start, line_end), (column_start, column_end)));
      }
      '(' => {
        index += 1;
        tokens.push(Token::new_with_trivia("(".to_string(), TokenType::LeftParen, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      ')' => {
        index += 1;
        tokens.push(Token::new_with_trivia(")".to_string(), TokenType::RightParen, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      '[' => {
        index += 1;
        tokens.push(Token::new_with_trivia("[".to_string(), TokenType::LeftSquare, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      ']' => {
        index += 1;
        tokens.push(Token::new_with_trivia("]".to_string(), TokenType::RightSquare, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      '{' => {
        index += 1;
        tokens.push(Token::new_with_trivia("{".to_string(), TokenType::LeftCurly, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      '}' => {
        index += 1;
        tokens.push(Token::new_with_trivia("}".to_string(), TokenType::RightCurly, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        trivia.clear();
      },
      _ => {
        index += 1;
        tokens.push(Token::new_with_trivia(input[index].to_string(), TokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));
        errors.push(Error::new(Severity::Error, (start_index, index), "Unknown token.".to_string()));
      }
    }
  }

  tokens.push(Token::new_with_trivia("".to_string(), TokenType::EndOfInput, (index, index), (line_start, line_end), (column_start, column_end), trivia.clone()));

  (tokens, errors)
}

#[cfg(test)]
mod tests {
  use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;
  use crate::ocean::frontend::compilationunit::token::tokentype::TokenType;
  use crate::ocean::frontend::lexer::lex;
  use crate::util::testing::assert_token_eq;

  #[test]
  fn lexer_let_with_number() {
    let input = "let a = 8";
    let (result_tokens, errors) = lex(&input.to_string().chars().collect());
    assert_eq!(errors.len(), 0);
    assert_eq!(result_tokens.len(), TokenIndex::at(5));
    assert_token_eq(&result_tokens[TokenIndex::at(0)],
                    "let", TokenType::Let, (0, 3), (1, 1), (0, 3));
    assert_token_eq(&result_tokens[TokenIndex::at(1)],
                    "a", TokenType::Identifier, (4, 5), (1, 1), (4, 5));
    assert_token_eq(&result_tokens[TokenIndex::at(2)],
                    "=", TokenType::Symbol, (6, 7), (1, 1), (6, 7));
    assert_token_eq(&result_tokens[TokenIndex::at(3)],
                    "8", TokenType::Number, (8, 9), (1, 1), (8, 9));
    assert_token_eq(&result_tokens[TokenIndex::at(4)],
                    "", TokenType::EndOfInput, (9, 9), (1, 1), (9, 9));
  }

  #[test]
  fn lexer_using_std_io() {
    let input = "using std.io";
    let (result_tokens, errors) = lex(&input.to_string().chars().collect());
    assert_eq!(errors.len(), 0);
    assert_eq!(result_tokens.len(), TokenIndex::at(5));
    assert_token_eq(&result_tokens[TokenIndex::at(0)],
                    "using", TokenType::Using, (0, 5), (1, 1), (0, 5));
    assert_token_eq(&result_tokens[TokenIndex::at(1)],
                    "std", TokenType::Identifier, (6, 9), (1, 1), (6, 9));
    assert_token_eq(&result_tokens[TokenIndex::at(2)],
                    ".", TokenType::Dot, (9, 10), (1, 1), (9, 10));
    assert_token_eq(&result_tokens[TokenIndex::at(3)],
                    "io", TokenType::Identifier, (10, 12), (1, 1), (10, 12));
    assert_token_eq(&result_tokens[TokenIndex::at(4)],
                    "", TokenType::EndOfInput, (12, 12), (1, 1), (13, 13));
  }

  #[test]
  fn lexer_parentheses() {
    let input = "(8.0 + (3.5 * 6))";
    let (result_tokens, errors) = lex(&input.to_string().chars().collect());
    assert_eq!(errors.len(), 0);
    assert_eq!(result_tokens.len(), TokenIndex::at(10));
    assert_token_eq(&result_tokens[TokenIndex::at(0)],
                    "(", TokenType::LeftParen, (0, 1), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(1)],
                    "8.0", TokenType::Number, (1, 4), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(2)],
                    "+", TokenType::Symbol, (5, 6), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(3)],
                    "(", TokenType::LeftParen, (7, 8), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(4)],
                    "3.5", TokenType::Number, (8, 11), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(5)],
                    "*", TokenType::Symbol, (12, 13), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(6)],
                    "6", TokenType::Number, (14, 15), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(7)],
                    ")", TokenType::RightParen, (15, 16), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(8)],
                    ")", TokenType::RightParen, (16, 17), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(9)],
                    "", TokenType::EndOfInput, (17, 17), (1, 1), (13, 13));
  }

  #[test]
  fn lexer_annotation() {
    let input = "@Operator(symbol: '+', order: Infix)";
    let (result_tokens, errors) = lex(&input.to_string().chars().collect());
    assert_eq!(errors.len(), 0);
    assert_eq!(result_tokens.len(), TokenIndex::at(11));
    assert_token_eq(&result_tokens[TokenIndex::at(0)],
                    "@Operator", TokenType::Annotation, (0, 9), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(1)],
                    "(", TokenType::LeftParen, (9, 10), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(2)],
                    "symbol", TokenType::Identifier, (10, 16), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(3)],
                    ":", TokenType::Colon, (16, 17), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(4)],
                    "'+'", TokenType::String, (18, 21), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(5)],
                    ",", TokenType::Comma, (21, 22), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(6)],
                    "order", TokenType::Identifier, (23, 28), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(7)],
                    ":", TokenType::Colon, (28, 29), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(8)],
                    "Infix", TokenType::Identifier, (30, 35), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(9)],
                    ")", TokenType::RightParen, (35, 36), (1, 1), (13, 13));
    assert_token_eq(&result_tokens[TokenIndex::at(10)],
                    "", TokenType::EndOfInput, (36, 36), (1, 1), (13, 13));
  }
}