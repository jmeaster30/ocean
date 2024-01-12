use crate::ocean::frontend::ast::*;
use crate::util::token::Token;

pub fn parse_annotations(ast: &mut Program) {
  for node in &mut ast.statements {
    parse_annotation_statement_node(node);
  }
}

fn parse_annotation_statement_node(statement_node: &mut StatementNode) {
  for data in &mut statement_node.data {
    match data {
      StatementNodeData::Annotation(annotation) => annotation.annotation_ast = Some(annotation_parser(&annotation.token.lexeme, &statement_node.statement)),
      _ => {}
    }
  }
  match &mut statement_node.statement {
    Some(statement) => parse_annotation_statement(statement),
    None => {}
  }
}

fn parse_annotation_statement(_: &mut Statement) {
  // TODO: need to move annotations into unions and packs so
}

fn annotation_parser(annotation_content: &String, linked_statement: &Option<Statement>) -> AnnotationNode {
  let trimmed_content = annotation_content.trim_matches(&['@', '\n', ' ', '\t'] as &[_]);
  let (annotation_name, annotation_body) = match trimmed_content.split_once(char::is_whitespace) {
    Some(annotation) => annotation,
    None => (trimmed_content, ""),
  };

  match annotation_name.to_lowercase().as_str() {
    "hydro" => {
      AnnotationNode::Hydro
    }
    "annotation" => {
      AnnotationNode::FunctionAnnotation
    }
    "cast" => {
      AnnotationNode::Cast
    }
    "operator" => {
      if let Some(Statement::Function(_)) = linked_statement {
        parse_operator_annotation(annotation_body)
      } else {
        panic!("The operator annotation must be attached to a function")
      }
    }
    _ => {
      AnnotationNode::None
    }
  }
}

#[derive(Clone, Debug)]
enum OperatorAnnotationTokenType {
  Operator,
  Colon,
  Number,
  Identifier,
  LeftParen,
  RightParen,
  LeftSquare,
  RightSquare,
  EndOfInput,
  Error,
}

#[derive(Clone, Debug)]
enum OperatorAnnotationParseState {
  Start,
  LeftHandSide,
  LeftHandSidePrecedence,
  Operator,
  RightHandSideStart,
  RightHandSide,
  RightHandSidePrecedence,
  Exit,
}

fn tokenize_operator_annotation(input: &str) -> Vec<Token<OperatorAnnotationTokenType>> {
  let input_chars = input.chars().collect::<Vec<char>>();
  let input_length = input.len();
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

        tokens.push(Token::new(lexeme.clone(), OperatorAnnotationTokenType::Identifier, (start_index, index), (line_start, line_end), (column_start, column_end)));

        lexeme.clear();
      }
      '0'..='9' => {
        lexeme.push_str(&c.to_string());
        index += 1;
        while index < input_length {
          let n = input_chars[index];
          match n {
            '0'..='9' => lexeme.push_str(&n.to_string()),
            _ => {
              index -= 1;
              break;
            }
          }
          index += 1
        }

        tokens.push(Token::new(lexeme.clone(), OperatorAnnotationTokenType::Number, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' | ';' => {
        lexeme.push_str(&input_chars[index].to_string());
        index += 1;
        while index < input_length - 1 && lexeme.len() < 3 {
          let n = input_chars[index];
          match n {
            '.' | ':' | '~' | '+' | '-' | '>' | '<' | '?' | '/' | '=' | '^' | '&' | '|' | '*' | '!' | '%' | ';' => {
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
          ":" => OperatorAnnotationTokenType::Colon,
          _ => OperatorAnnotationTokenType::Operator,
        };

        tokens.push(Token::new(lexeme.clone(), token_type, (start_index, index), (line_start, line_end), (column_start, column_end)));
        lexeme.clear();
      }
      '(' => tokens.push(Token::new("(".to_string(), OperatorAnnotationTokenType::LeftParen, (start_index, index), (line_start, line_end), (column_start, column_end))),
      ')' => tokens.push(Token::new(")".to_string(), OperatorAnnotationTokenType::RightParen, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '[' => tokens.push(Token::new("[".to_string(), OperatorAnnotationTokenType::LeftSquare, (start_index, index), (line_start, line_end), (column_start, column_end))),
      ']' => tokens.push(Token::new("]".to_string(), OperatorAnnotationTokenType::RightSquare, (start_index, index), (line_start, line_end), (column_start, column_end))),
      '\r' | '\n' | ' ' | '\t' => {}
      _ => tokens.push(Token::new(input_chars[index].to_string(), OperatorAnnotationTokenType::Error, (start_index, index), (line_start, line_end), (column_start, column_end))),
    }
    index += 1;
  }

  tokens.push(Token::new("".to_string(), OperatorAnnotationTokenType::EndOfInput, (index, index), (line_start, line_end), (column_start, column_end)));
  tokens
}

fn parse_operator_annotation(lexeme: &str) -> AnnotationNode {
  let tokens = tokenize_operator_annotation(lexeme);

  let mut state = OperatorAnnotationParseState::Start;

  let mut operator = None;
  let mut left_hand_side_name = None;
  let mut left_precedence = None;
  let mut right_hand_side_name = None;
  let mut right_precedence = None;

  let mut token_index = 0;
  while token_index < tokens.len() {
    let token = &tokens[token_index];

    match (&token.token_type, &state) {
      (OperatorAnnotationTokenType::Identifier, OperatorAnnotationParseState::Start) => {
        state = OperatorAnnotationParseState::LeftHandSide;
        left_hand_side_name = Some(token.lexeme.clone());
        token_index += 1;
      }
      (OperatorAnnotationTokenType::Operator, OperatorAnnotationParseState::Start) => {
        state = OperatorAnnotationParseState::RightHandSideStart;
        token_index += 1;
        operator = Some(token.lexeme.clone());
      }
      (OperatorAnnotationTokenType::Colon, OperatorAnnotationParseState::LeftHandSide) => {
        state = OperatorAnnotationParseState::LeftHandSidePrecedence;
        token_index += 1;
      }
      (OperatorAnnotationTokenType::Operator, OperatorAnnotationParseState::LeftHandSide) => {
        state = OperatorAnnotationParseState::Operator;
      }
      (OperatorAnnotationTokenType::Number, OperatorAnnotationParseState::LeftHandSidePrecedence) => {
        state = OperatorAnnotationParseState::Operator;
        token_index += 1;
        left_precedence = Some(token.lexeme.parse::<usize>().unwrap());
      }
      (OperatorAnnotationTokenType::Operator, OperatorAnnotationParseState::Operator) => {
        state = OperatorAnnotationParseState::RightHandSideStart;
        token_index += 1;
        operator = Some(token.lexeme.clone());
      }
      (OperatorAnnotationTokenType::Identifier, OperatorAnnotationParseState::RightHandSideStart) => {
        state = OperatorAnnotationParseState::RightHandSide;
        token_index += 1;
        right_hand_side_name = Some(token.lexeme.clone());
      }
      (OperatorAnnotationTokenType::Colon, OperatorAnnotationParseState::RightHandSide) => {
        state = OperatorAnnotationParseState::RightHandSidePrecedence;
        token_index += 1;
      }
      (OperatorAnnotationTokenType::Number, OperatorAnnotationParseState::RightHandSidePrecedence) => {
        state = OperatorAnnotationParseState::Exit;
        token_index += 1;
        right_precedence = Some(token.lexeme.parse::<usize>().unwrap());
      }
      (OperatorAnnotationTokenType::EndOfInput, OperatorAnnotationParseState::RightHandSide) => {
        state = OperatorAnnotationParseState::Exit;
      }
      (OperatorAnnotationTokenType::EndOfInput, OperatorAnnotationParseState::Exit) => break,
      (_, _) => panic!("There was an issue while parsing the operator annotation! {:?} {:?}", state, token), // TODO: Make a better error message here
    }
  }

  match (operator, left_hand_side_name, left_precedence, right_hand_side_name, right_precedence) {
    (Some(operator), Some(left_hand_side_name), left_precedence, Some(right_hand_side_name), right_precedence) => AnnotationNode::Operator(AnnotationOperator::new(operator, OperatorType::Infix, Some(left_hand_side_name), left_precedence, Some(right_hand_side_name), right_precedence)),
    (Some(operator), Some(left_hand_side_name), left_precedence, None, None) => AnnotationNode::Operator(AnnotationOperator::new(operator, OperatorType::Postfix, Some(left_hand_side_name), left_precedence, None, None)),
    (Some(operator), None, None, Some(right_hand_side_name), right_precedence) => AnnotationNode::Operator(AnnotationOperator::new(operator, OperatorType::Prefix, None, None, Some(right_hand_side_name), right_precedence)),
    _ => panic!("There was an issue parsing"),
  }
}
