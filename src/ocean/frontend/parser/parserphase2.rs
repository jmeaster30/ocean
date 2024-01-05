use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parser::astsymbolstack::{AstSymbol, AstSymbolStack};
use crate::ocean::frontend::parser::parsestatestack::{ParseState, ParseStateStack};
use crate::ocean::frontend::parser::precedencetable::PrecedenceTable;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;
use crate::util::token::Token;
use itertools::Either;
use crate::ocean::frontend::parser::parserphase1::parse_phase_one_partial;

pub fn parse_phase_two(ast: &mut Program, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let undefined_prefix = 20000;
  let undefined_infix = 10000;

  let mut errors = Vec::new();

  for statement_node in &mut ast.statements {
    for data in &statement_node.data {
      match data {
        StatementNodeData::Annotation(annotation) => match &annotation.annotation_ast {
          Some(AnnotationNode::Operator(operator)) => match operator.operator_type {
            OperatorType::Infix => {
              if precedence_table.is_binary_operator(&operator.operator) {
                if let (Some(left_prec), Some(right_prec)) = (operator.left_precedence, operator.right_precedence) {
                  if precedence_table.get_binary_precedence(&operator.operator) != (left_prec, right_prec) {
                    errors.push(Error::new(Severity::Error, annotation.token.get_span(), "Conflicting precedences for the same operator. (TODO add info about the other operator)".to_string()));
                  }
                }
              } else if let (Some(left_prec), Some(right_prec)) = (operator.left_precedence, operator.right_precedence) {
                precedence_table.add_binary_operator(operator.operator.as_str(), left_prec, right_prec);
              } else {
                precedence_table.add_binary_operator(operator.operator.as_str(), undefined_infix, undefined_infix + 1);
              }
            }
            OperatorType::Postfix => {}
            OperatorType::Prefix => {
              if precedence_table.is_prefix_operator(&operator.operator) {
                if let Some(right_prec) = operator.right_precedence {
                  if precedence_table.get_prefix_precedence(&operator.operator) != right_prec {
                    errors.push(Error::new(Severity::Error, annotation.token.get_span(), "Conflicting precedences for the same operator. (TODO add info about the other operator)".to_string()));
                  }
                }
              } else if let Some(right_prec) = operator.right_precedence {
                precedence_table.add_prefix_operator(operator.operator.as_str(), right_prec);
              } else {
                precedence_table.add_prefix_operator(operator.operator.as_str(), undefined_prefix);
              }
            }
          },
          _ => {}
        },
        _ => {}
      }
    }
  }

  for statement_node in &mut ast.statements {
    match &mut statement_node.statement {
      Some(statement) => match parse_phase_two_statement(statement, precedence_table) {
        Ok(()) => {}
        Err(mut new_errors) => errors.append(&mut new_errors),
      },
      None => {}
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}

fn parse_phase_two_statement(statement: &mut Statement, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  match statement {
    Statement::WhileLoop(while_loop) => parse_phase_two_while(while_loop, precedence_table),
    Statement::ForLoop(for_loop) => parse_phase_two_for(for_loop, precedence_table),
    Statement::Loop(loop_stmt) => parse_phase_two_loop(loop_stmt, precedence_table),
    Statement::Branch(branch) => parse_phase_two_branch(branch, precedence_table),
    Statement::Match(match_stmt) => parse_phase_two_match(match_stmt, precedence_table),
    Statement::Assignment(assignment) => parse_phase_two_assignment(assignment, precedence_table),
    Statement::Function(function) => parse_phase_two_function(function, precedence_table),
    Statement::Expression(expression) => parse_phase_two_expression(&mut expression.expression_node, precedence_table),
    _ => Ok(()),
  }
}

fn parse_phase_two_compound(compound_statement: &mut CompoundStatement, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let mut errors = Vec::new();
  for statement_node in &mut compound_statement.body {
    match &mut statement_node.statement {
      Some(statement) => match parse_phase_two_statement(statement, precedence_table) {
        Ok(()) => {}
        Err(mut new_errors) => errors.append(&mut new_errors),
      },
      None => {}
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}

fn parse_phase_two_while(while_loop: &mut WhileLoop, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  join_results(vec![parse_phase_two_expression(&mut while_loop.condition, precedence_table), parse_phase_two_compound(&mut while_loop.body, precedence_table)])
}

fn parse_phase_two_for(for_loop: &mut ForLoop, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  join_results(vec![parse_phase_two_expression(&mut for_loop.iterator, precedence_table), parse_phase_two_expression(&mut for_loop.iterable, precedence_table), parse_phase_two_compound(&mut for_loop.body, precedence_table)])
}

fn parse_phase_two_loop(loop_loop: &mut Loop, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  parse_phase_two_compound(&mut loop_loop.body, precedence_table)
}

fn parse_phase_two_branch(branch: &mut Branch, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  join_results(vec![
    parse_phase_two_expression(&mut branch.condition, precedence_table),
    parse_phase_two_compound(&mut branch.body, precedence_table),
    match &mut branch.else_branch {
      Some(else_body) => match &mut else_body.body {
        Either::Left(body) => parse_phase_two_compound(body, precedence_table),
        Either::Right(else_branch) => parse_phase_two_branch(else_branch, precedence_table),
      },
      None => Ok(()),
    },
  ])
}

fn parse_phase_two_match(_: &mut Match, _: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  todo!()
}

fn parse_phase_two_assignment(assignment: &mut Assignment, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  join_results(vec![
    match &mut assignment.left_expression {
      Either::Left(_) => Ok(()),
      Either::Right(expression) => parse_phase_two_expression(expression, precedence_table),
    },
    parse_phase_two_expression(&mut assignment.right_expression.expression_node, precedence_table),
  ])
}

fn parse_phase_two_function(function: &mut Function, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let mut new_precedence_table = precedence_table.clone();
  let mut parse_results = Vec::new();

  for result in &mut function.results {
    parse_results.push(match &mut result.expression {
      Some(expression) => parse_phase_two_expression(expression, &mut new_precedence_table),
      None => Ok(()),
    });
  }

  parse_results.push(match &mut function.compound_statement {
    Some(compound_stmt) => parse_phase_two_compound(compound_stmt, &mut new_precedence_table),
    None => Ok(()),
  });
  join_results(parse_results)
}

fn parse_phase_two_expression(expression: &mut ExpressionNode, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let expression_tokens = expression.tokens.iter().filter(|x| x.token_type != TokenType::Newline && x.token_type != TokenType::Comment).collect::<Vec<&Token<TokenType>>>();
  let (parsed_expression, _) = parse_expression(&expression_tokens, 0, precedence_table, 0)?;
  expression.parsed_expression = Some(parsed_expression);
  Ok(())
}

fn parse_expression(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable, min_precedence: usize) -> Result<(Expression, usize), Vec<Error>> {
  //println!("tokens size: {} token_index: {} min_prec {}", tokens.len(), token_index, min_precedence);
  //println!("tokens: {:?}", tokens);
  let (mut left_hand_side, next_token_index) = if is_literal_start_token(tokens[token_index]) {
    parse_literal(tokens, token_index, precedence_table)?
  } else if precedence_table.is_prefix_operator(&tokens[token_index].lexeme) {
    let prefix_precedence = precedence_table.get_prefix_precedence(&tokens[token_index].lexeme);
    let (parsed_expression, next_index) = parse_expression(tokens, token_index + 1, precedence_table, prefix_precedence)?;
    (Expression::PrefixOperation(PrefixOperator::new(tokens[token_index].clone(), Box::new(parsed_expression))), next_index)
  } else {
    return Err(vec![Error::new(Severity::Error, tokens[token_index].get_span(), "Unexpected token. Expected a literal or a prefix operator.".to_string())])
  };

  let mut current_token_index = next_token_index;
  while current_token_index < tokens.len() {
    if tokens[current_token_index].token_type == TokenType::RightParen || tokens[current_token_index].token_type == TokenType::RightSquare || tokens[current_token_index].token_type == TokenType::EndOfInput {
      break;
    }

    match tokens[current_token_index].token_type {
      TokenType::RightParen | TokenType::RightSquare | TokenType::EndOfInput | TokenType::Comma => break,
      _ => {}
    }

    // do postfix here?

    if !precedence_table.is_binary_operator(&tokens[current_token_index].lexeme) {
      match tokens[current_token_index].token_type {
        TokenType::LeftSquare => {
          let mut arguments = Vec::new();
          let left_square_index = current_token_index;
          current_token_index += 1;
          while current_token_index < tokens.len() {
            let (index_expression, next_token_index) = parse_expression(tokens, current_token_index, precedence_table, 0)?;
            if tokens[next_token_index].token_type == TokenType::Comma {
              arguments.push(Argument::new(index_expression, Some(tokens[next_token_index].clone())));
              current_token_index = next_token_index + 1;
            } else {
              arguments.push(Argument::new(index_expression, None));
              current_token_index = next_token_index;
              break;
            }
          }
          if tokens[current_token_index].token_type != TokenType::RightSquare {
            return Err(vec![Error::new(Severity::Error, tokens[current_token_index].get_span(), "Expected the end of the array index. Missing a ']' right square bracket.".to_string())]);
          }
          left_hand_side = Expression::ArrayIndex(ArrayIndex::new(Box::new(left_hand_side), tokens[left_square_index].clone(), arguments, tokens[current_token_index].clone()));
          current_token_index += 1;
          continue;
        }
        TokenType::LeftParen => {
          let mut arguments = Vec::new();
          let left_paren_index = current_token_index;
          current_token_index += 1;
          while current_token_index < tokens.len() {
            let (index_expression, next_token_index) = parse_expression(tokens, current_token_index, precedence_table, 0)?;
            if tokens[next_token_index].token_type == TokenType::Comma {
              arguments.push(Argument::new(index_expression, Some(tokens[next_token_index].clone())));
              current_token_index = next_token_index + 1;
            } else {
              arguments.push(Argument::new(index_expression, None));
              current_token_index = next_token_index;
              break;
            }
          }
          if tokens[current_token_index].token_type != TokenType::RightParen {
            return Err(vec![Error::new(Severity::Error, tokens[current_token_index].get_span(), "Expected the end of the call. Missing a ')' right parenthesis.".to_string())]);
          }
          left_hand_side = Expression::Call(Call::new(Box::new(left_hand_side), tokens[left_paren_index].clone(), arguments, tokens[current_token_index].clone()));
          current_token_index += 1;
          continue;
        }
        TokenType::As => {
          let (parsed_type, next_token_index) = parse_type(tokens, current_token_index + 1)?;
          left_hand_side = Expression::Cast(Cast::new(Box::new(left_hand_side), tokens[current_token_index].clone(), parsed_type));
          current_token_index = next_token_index;
          continue;
        }
        _ => return Err(vec![Error::new(Severity::Error, tokens[current_token_index].get_span(), "Unexpected operator. Expected a valid binary operator (TODO display binary operators from precedence table or add an extra details section to the error message maybe it is suggestions on how to fix the error?).".to_string())]),
      }
    }

    let (left_precedence, right_precedence) = precedence_table.get_binary_precedence(&tokens[current_token_index].lexeme);
    if left_precedence < min_precedence {
      break;
    }

    let (right_hand_side, next_token_index) = parse_expression(tokens, current_token_index + 1, precedence_table, right_precedence)?;
    left_hand_side = Expression::BinaryOperation(BinaryOperator::new(Box::new(left_hand_side), tokens[current_token_index].clone(), Box::new(right_hand_side)));
    current_token_index = next_token_index
  }

  Ok((left_hand_side, current_token_index))
}

fn parse_literal(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> Result<(Expression, usize), Vec<Error>> {
  let (expression, next_token_index) = match tokens[token_index].token_type {
    TokenType::String => (Expression::String(StringLiteral::new(tokens[token_index].clone())), token_index + 1),
    TokenType::InterpolatedString => {
      let parsed_expressions = parse_interpolated_string(&tokens[token_index].lexeme, precedence_table)?;
      (Expression::InterpolatedString(InterpolatedString::new(tokens[token_index].clone(), parsed_expressions)), token_index + 1)
    },
    TokenType::True | TokenType::False => (Expression::Boolean(Boolean::new(tokens[token_index].clone())), token_index + 1),
    TokenType::Number => (Expression::Number(Number::new(tokens[token_index].clone())), token_index + 1),
    TokenType::LeftParen => parse_sub_expression_or_tuple(tokens, token_index, precedence_table)?,
    TokenType::Identifier => (Expression::Variable(Variable::new(tokens[token_index].clone())), token_index + 1),
    _ => panic!("Unexpected case??? {}", tokens[token_index]),
  };

  let current_token_index = next_token_index;
  if current_token_index >= tokens.len() {
    return Ok((expression, next_token_index));
  }

  Ok((expression, current_token_index))
}

fn parse_arguments(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> Result<(Argument, usize), Vec<Error>> {
  let (index_expression, next_token_index) = parse_expression(tokens, token_index, precedence_table, 0)?;

  if tokens[next_token_index].token_type == TokenType::Comma {
    Ok((Argument::new(index_expression, Some(tokens[next_token_index].clone())), next_token_index + 1))
  } else {
    Ok((Argument::new(index_expression, None), next_token_index))
  }
}

fn parse_interpolated_string(lexeme: &String, precedence_table: &PrecedenceTable) -> Result<Vec<Expression>, Vec<Error>> {
  let chars = lexeme.chars().collect::<Vec<char>>();
  let mut errors = Vec::new();

  let mut expressions = Vec::new();
  let mut index = 0;
  let mut in_expression = false;
  let mut current_expression_chars = Vec::new();
  while index < chars.len() {
    let current_char = chars[index];
    if current_char == '{' && !in_expression {
      // && current_char == '{' || current_char == '}'
      in_expression = true;
    } else if current_char == '{' && in_expression && current_expression_chars.len() == 0 {
      in_expression = false;
    } else if current_char == '}' && in_expression {
      let tokens = match lex(&current_expression_chars.iter().collect::<String>()) {
        Ok(tokens) => tokens,
        Err(mut new_errors) => {
          errors.append(&mut new_errors);
          continue;
        }
      };
      let clean_tokens = tokens.iter().filter(|x| x.token_type != TokenType::Newline && x.token_type != TokenType::Comment).collect::<Vec<&Token<TokenType>>>();
      let (expression, _) = match parse_expression(&clean_tokens, 0, precedence_table, 0) {
        Ok(a) => a,
        Err(mut new_errors) => {
          errors.append(&mut new_errors);
          index += 1;
          continue;
        }
      };
      expressions.push(expression);
      current_expression_chars.clear();
      in_expression = false;
    } else if current_char == '}' && !in_expression {
      // TODO I think we need to make it so it required an escaped closing brace so the escaped brackets can be symmetric.
    } else if in_expression {
      current_expression_chars.push(current_char);
    }
    index += 1;
  }

  if errors.is_empty() {
    Ok(expressions)
  } else {
    Err(errors)
  }
}

fn parse_sub_expression_or_tuple(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> Result<(Expression, usize), Vec<Error>> {
  // TODO need to add parsing for tuples here
  if tokens[token_index].token_type != TokenType::LeftParen {
    panic!("Unexpected token :( {}", tokens[token_index]);
  }

  let left_paren = tokens[token_index].clone();

  if tokens[token_index + 1].token_type == TokenType::Identifier && tokens[token_index + 2].token_type == TokenType::Colon {
    let (tuple_members, next_token_index) = parse_tuple_members(tokens, token_index + 1, precedence_table)?;

    if tokens[next_token_index].token_type != TokenType::RightParen {
      Err(vec![Error::new(Severity::Error, tokens[next_token_index].get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
    } else {
      Ok((Expression::Tuple(Tuple::new(left_paren, tuple_members, tokens[next_token_index].clone())), next_token_index + 1))
    }
  } else {
    let (subexpression, next_token_index) = parse_expression(tokens, token_index + 1, precedence_table, 0)?;

    if tokens[next_token_index].token_type == TokenType::Comma {
      // also a tuple
      let (mut tuple_members, new_token_index) = parse_tuple_members(tokens, next_token_index + 1, precedence_table)?;
      tuple_members.insert(0, TupleMember::new(None, None, subexpression, Some(tokens[next_token_index].clone())));

      if tokens[new_token_index].token_type != TokenType::RightParen {
        Err(vec![Error::new(Severity::Error, tokens[new_token_index].get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
      } else {
        Ok((Expression::Tuple(Tuple::new(left_paren, tuple_members, tokens[new_token_index].clone())), new_token_index + 1))
      }
    } else {
      if tokens[next_token_index].token_type != TokenType::RightParen {
        Err(vec![Error::new(Severity::Error, tokens[next_token_index].get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
      } else {
        Ok((Expression::SubExpression(SubExpression::new(left_paren, Box::new(subexpression), tokens[next_token_index].clone())), next_token_index + 1))
      }
    }
  }
}

fn parse_tuple_members(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> Result<(Vec<TupleMember>, usize), Vec<Error>> {
  let mut errors = Vec::new();
  let mut tuple_members = Vec::new();
  let mut current_index = token_index;
  while current_index < tokens.len() {
    if tokens[current_index].token_type == TokenType::RightParen {
      break;
    }

    let (name, colon) = if tokens[current_index].token_type == TokenType::Identifier && tokens[current_index + 1].token_type == TokenType::Colon {
      let idx = current_index;
      current_index += 2;
      (Some(tokens[idx].clone()), Some(tokens[idx + 1].clone()))
    } else {
      (None, None)
    };

    let (expression, next_token_index) = match parse_expression(tokens, current_index, precedence_table, 0) {
      Ok(a) => a,
      Err(mut new_errors) => {
        errors.append(&mut new_errors);
        continue;
      }
    };

    let comma_token = if tokens[next_token_index].token_type == TokenType::Comma {
      current_index = next_token_index + 1;
      Some(tokens[next_token_index].clone())
    } else {
      current_index = next_token_index;
      None
    };

    tuple_members.push(TupleMember::new(name, colon, expression, comma_token));
  }

  if errors.is_empty() {
    Ok((tuple_members, current_index))
  } else {
    Err(errors)
  }
}

fn parse_type(tokens: &Vec<&Token<TokenType>>, token_index: usize) -> Result<(Type, usize), Vec<Error>> {
  // This is very janky going from ref vec of refs to ref vec of non-refs
  let (ast_symbol, next_token_index, mut errors) = parse_phase_one_partial(&tokens.iter().map(|x| x.clone().clone()).collect::<Vec<Token<TokenType>>>(), token_index, ParseState::Type, None);
  match (ast_symbol, errors.clone()) {
    (AstSymbol::Type(parsed_type), errors) if errors.len() == 0 => Ok((parsed_type, next_token_index)),
    _ => {
      errors.push(Error::new(Severity::Error, (tokens[token_index].get_span().0, tokens[next_token_index - 1].get_span().1), "Type parser produced an unexpected ast node :(".to_string()));
      Err(errors)
    }
  }
}

fn is_literal_start_token(token: &Token<TokenType>) -> bool {
  match token.token_type {
    TokenType::String | TokenType::InterpolatedString | TokenType::Identifier | TokenType::LeftParen | TokenType::True | TokenType::False | TokenType::Number => true,
    _ => false,
  }
}

fn join_results(results: Vec<Result<(), Vec<Error>>>) -> Result<(), Vec<Error>> {
  let mut errors = Vec::new();
  for result in results {
    if let Err(mut new_errors) = result {
      errors.append(&mut new_errors);
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}
