use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parser::astsymbolstack::AstSymbol;
use crate::ocean::frontend::parser::parsestatestack::ParseState;
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

fn parse_phase_two_match(match_body: &mut Match, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let mut results = vec![ parse_phase_two_expression(&mut match_body.expression, precedence_table) ];

  for match_case in &mut match_body.cases {
    results.push(parse_phase_two_expression(&mut match_case.pattern, precedence_table));
    results.push(match &mut match_case.body {
      Either::Left(statement) => parse_phase_two_statement(statement, precedence_table),
      Either::Right(compound_statement ) => parse_phase_two_compound(compound_statement, precedence_table),
    });
  }

  join_results(results)
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

fn parse_phase_two_expression_ast_node(node: &mut AstNodeExpression, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  match node {
    AstNodeExpression::Match(match_node) => parse_phase_two_match(match_node, precedence_table),
    AstNodeExpression::Loop(loop_node) => parse_phase_two_loop(loop_node, precedence_table),
    AstNodeExpression::ForLoop(loop_node) => parse_phase_two_for(loop_node, precedence_table),
    AstNodeExpression::WhileLoop(loop_node) => parse_phase_two_while(loop_node, precedence_table),
    AstNodeExpression::Branch(branch_node) => parse_phase_two_branch(branch_node, precedence_table),
    AstNodeExpression::Function(function) => parse_phase_two_function(function, precedence_table),
    _ => todo!()
  }
}

fn parse_phase_two_expression(expression: &mut ExpressionNode, precedence_table: &mut PrecedenceTable) -> Result<(), Vec<Error>> {
  let expression_tokens = expression.tokens.iter().filter(|x| match x {
    Either::Left(token) => token.token_type != TokenType::Newline && token.token_type != TokenType::Comment,
    Either::Right(_) => true,
  }).collect::<Vec<&Either<Token<TokenType>, AstNodeExpression>>>();
  let (parsed_expression, _) = parse_expression(&expression_tokens, 0, precedence_table, 0)?;
  expression.parsed_expression = Some(parsed_expression);
  Ok(())
}

fn parse_expression(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize, precedence_table: &mut PrecedenceTable, min_precedence: usize) -> Result<(Expression, usize), Vec<Error>> {
  let (mut left_hand_side, next_token_index) = match tokens[token_index].clone() {
    Either::Right(mut expression) => {
      parse_phase_two_expression_ast_node(&mut expression, precedence_table)?;
      (Expression::AstNode(expression.clone()), token_index + 1)
    },
    Either::Left(token) => if is_literal_start_token(&token) {
      parse_literal(tokens, token_index, precedence_table)?
    } else if precedence_table.is_prefix_operator(&token.lexeme) {
      let prefix_precedence = precedence_table.get_prefix_precedence(&token.lexeme);
      let (parsed_expression, next_index) = parse_expression(tokens, token_index + 1, precedence_table, prefix_precedence)?;
      (Expression::PrefixOperation(PrefixOperator::new(token.clone(), Box::new(parsed_expression))), next_index)
    } else {
      return Err(vec![Error::new(Severity::Error, token.get_span(), "Unexpected token. Expected a literal or a prefix operator.".to_string())])
    }
  };

  let mut current_token_index = next_token_index;
  while current_token_index < tokens.len() {
    let current_token = match tokens[current_token_index] {
      Either::Left(token) => token,
      // TODO add expression span
      Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected ')', ']', ',', or an operator. OR you may have missed a semicolon.".to_string())])
    };

    match current_token.token_type {
      TokenType::RightParen | TokenType::RightSquare | TokenType::EndOfInput | TokenType::Comma => break,
      _ => {}
    }

    if !precedence_table.is_binary_operator(&current_token.lexeme) {
      match current_token.token_type {
        TokenType::LeftSquare => {
          current_token_index += 1;
          let (arguments, next_token_index) = parse_arguments(tokens, current_token_index, precedence_table)?;
          current_token_index = next_token_index;
          let right_square_token = match tokens[current_token_index] {
            Either::Left(token) => token,
            // TODO add expression span
            Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Expected the end of the array index. Missing a ']' right square bracket.".to_string())])
          };
          if right_square_token.token_type != TokenType::RightSquare {
            return Err(vec![Error::new(Severity::Error, right_square_token.get_span(), "Expected the end of the array index. Missing a ']' right square bracket.".to_string())]);
          }
          left_hand_side = Expression::ArrayIndex(ArrayIndex::new(Box::new(left_hand_side), current_token.clone(), arguments, right_square_token.clone()));
          current_token_index += 1;
          continue;
        }
        TokenType::LeftParen => {
          current_token_index += 1;
          let (arguments, next_token_index) = parse_arguments(tokens, current_token_index, precedence_table)?;
          current_token_index = next_token_index;
          let right_paren_token = match tokens[current_token_index] {
            Either::Left(token) => token,
            // TODO add expression span
            Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Expected the end of the call. Missing a ')' right parenthesis.".to_string())])
          };
          if right_paren_token.token_type != TokenType::RightParen {
            return Err(vec![Error::new(Severity::Error, right_paren_token.get_span(), "Expected the end of the call. Missing a ')' right parenthesis.".to_string())]);
          }
          left_hand_side = Expression::Call(Call::new(Box::new(left_hand_side), current_token.clone(), arguments, right_paren_token.clone()));
          current_token_index += 1;
          continue;
        }
        TokenType::As => {
          let (parsed_type, next_token_index) = match tokens[current_token_index + 1] {
            Either::Left(_) => parse_type(tokens, current_token_index + 1)?,
            Either::Right(expression) => match expression {
              AstNodeExpression::Type(parsed_type) => (parsed_type.clone(), current_token_index + 2),
              // TODO add expression span
              _ => return Err(vec![Error::new(Severity::Error, (0, 0), "Expected a type for the cast expression.".to_string())]),
            }
          };
          left_hand_side = Expression::Cast(Cast::new(Box::new(left_hand_side), current_token.clone(), parsed_type.clone()));
          current_token_index = next_token_index;
          continue;
        }
        _ if precedence_table.is_postfix_operator(&current_token.lexeme) => {
          let postfix_power = precedence_table.get_postfix_precedence(&current_token.lexeme);
          if postfix_power < min_precedence {
            break;
          }

          left_hand_side = Expression::PostfixOperation(PostfixOperator::new(Box::new(left_hand_side), current_token.clone()));
          current_token_index += 1;
          continue;
        }
        _ => return Err(vec![Error::new(Severity::Error, current_token.get_span(), "Unexpected operator. Expected a valid binary operator (TODO display binary operators from precedence table or add an extra details section to the error message maybe it is suggestions on how to fix the error?).".to_string())]),
      }
    }

    let operator_token = match tokens[current_token_index] {
      Either::Left(token) => token,
      // TODO add expression span
      Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected operator :(".to_string())])
    };

    let (left_precedence, right_precedence) = precedence_table.get_binary_precedence(&operator_token.lexeme);
    if left_precedence < min_precedence {
      break;
    }

    let (right_hand_side, next_token_index) = parse_expression(tokens, current_token_index + 1, precedence_table, right_precedence)?;
    left_hand_side = Expression::BinaryOperation(BinaryOperator::new(Box::new(left_hand_side), operator_token.clone(), Box::new(right_hand_side)));
    current_token_index = next_token_index
  }

  Ok((left_hand_side, current_token_index))
}

fn parse_literal(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize, precedence_table: &mut PrecedenceTable) -> Result<(Expression, usize), Vec<Error>> {
  let token = tokens[token_index].clone().expect_left("Uh oh found an expression :(");
  let (expression, next_token_index) = match token.token_type {
    TokenType::String => (Expression::String(StringLiteral::new(token.clone())), token_index + 1),
    TokenType::InterpolatedString => {
      let parsed_expressions = parse_interpolated_string(&token.lexeme, precedence_table)?;
      (Expression::InterpolatedString(InterpolatedString::new(token.clone(), parsed_expressions)), token_index + 1)
    },
    TokenType::True | TokenType::False => (Expression::Boolean(Boolean::new(token.clone())), token_index + 1),
    TokenType::Number => (Expression::Number(Number::new(token.clone())), token_index + 1),
    TokenType::LeftParen => parse_sub_expression_or_tuple(tokens, token_index, precedence_table)?,
    TokenType::Identifier => (Expression::Variable(Variable::new(token.clone())), token_index + 1),
    TokenType::LeftSquare => {
      let mut current_token_index = token_index;
      current_token_index += 1;
      let (arguments, next_token_index) = parse_arguments(tokens, current_token_index, precedence_table)?;
      current_token_index = next_token_index;
      let current_token = tokens[current_token_index].clone().expect_left("Uh oh found an expression here :(");
      if current_token.token_type != TokenType::RightSquare {
        return Err(vec![Error::new(Severity::Error, current_token.get_span(), "Expected the end of the array literal. Missing a ']' right square bracket.".to_string())]);
      }
      (Expression::ArrayLiteral(ArrayLiteral::new(token.clone(), arguments, current_token.clone())), current_token_index + 1)
    }
    _ => panic!("Unexpected case??? {:?}", tokens[token_index]),
  };

  let current_token_index = next_token_index;
  if current_token_index >= tokens.len() {
    return Ok((expression, next_token_index));
  }

  Ok((expression, current_token_index))
}

fn parse_arguments(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize, precedence_table: &mut PrecedenceTable) -> Result<(Vec<Argument>, usize), Vec<Error>> {
  let mut current_token_index = token_index;
  let mut arguments = Vec::new();
  if let Either::Left(token) = tokens[current_token_index] {
    if token.token_type == TokenType::RightParen || token.token_type == TokenType::RightSquare {
      return Ok((arguments, current_token_index));
    }
  }

  while current_token_index < tokens.len() {
    let (index_expression, next_token_index) = parse_expression(tokens, current_token_index, precedence_table, 0)?;
    if let Either::Left(token) = tokens[next_token_index] {
      if token.token_type == TokenType::Comma {
        arguments.push(Argument::new(index_expression, Some(token.clone())));
        current_token_index = next_token_index + 1;
        continue;
      }
    }
    arguments.push(Argument::new(index_expression, None));
    current_token_index = next_token_index;
    break;
  }
  Ok((arguments, current_token_index))
}

fn parse_interpolated_string(lexeme: &String, precedence_table: &mut PrecedenceTable) -> Result<Vec<Expression>, Vec<Error>> {
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
        Ok(tokens) => tokens.iter()
          .filter(|x| x.token_type != TokenType::Newline && x.token_type != TokenType::Comment)
          .map(|x| Either::Left(x.clone()))
          .collect::<Vec<Either<Token<TokenType>, AstNodeExpression>>>(),
        Err(mut new_errors) => {
          errors.append(&mut new_errors);
          continue;
        }
      };
      // This is weird
      let clean_tokens = tokens.iter().collect::<Vec<&Either<Token<TokenType>, AstNodeExpression>>>();
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

fn parse_sub_expression_or_tuple(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize, precedence_table: &mut PrecedenceTable) -> Result<(Expression, usize), Vec<Error>> {
  let start_token = match tokens[token_index] {
    Either::Left(token) => token,
    // TODO add expression span
    Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected left parenthesis :(".to_string())])
  };
  if start_token.token_type != TokenType::LeftParen {
    return Err(vec![Error::new(Severity::Error, start_token.get_span(), "Unexpected token. Expected left parenthesis :(".to_string())])
  }

  match (tokens[token_index + 1], tokens[token_index + 2]) {
    (Either::Left(plus_one), Either::Left(plus_two))
    if plus_one.token_type == TokenType::Identifier && plus_two.token_type == TokenType::Colon => {
      let (tuple_members, next_token_index) = parse_tuple_members(tokens, token_index + 1, precedence_table)?;

      let right_paren_token = match tokens[next_token_index] {
        Either::Left(token) => token,
        // TODO add expression span
        Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected right paren :(".to_string())])
      };

      if right_paren_token.token_type != TokenType::RightParen {
        Err(vec![Error::new(Severity::Error, right_paren_token.get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
      } else {
        Ok((Expression::Tuple(Tuple::new(start_token.clone(), tuple_members, right_paren_token.clone())), next_token_index + 1))
      }
    }
    _ => {
      let (subexpression, next_token_index) = parse_expression(tokens, token_index + 1, precedence_table, 0)?;

      let next_token = match tokens[next_token_index] {
        Either::Left(token) => token,
        // TODO add expression span
        Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected comma or right parenthesis :(".to_string())])
      };

      if next_token.token_type == TokenType::Comma {
        // also a tuple
        let (mut tuple_members, new_token_index) = parse_tuple_members(tokens, next_token_index + 1, precedence_table)?;
        tuple_members.insert(0, TupleMember::new(None, None, subexpression, Some(next_token.clone())));

        let end_token = match tokens[new_token_index] {
          Either::Left(token) => token,
          // TODO add expression span
          Either::Right(expression) => return Err(vec![Error::new(Severity::Error, (0, 0), "Unexpected expression. Expected right parenthesis :(".to_string())])
        };

        if end_token.token_type != TokenType::RightParen {
          Err(vec![Error::new(Severity::Error, end_token.get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
        } else {
          Ok((Expression::Tuple(Tuple::new(start_token.clone(), tuple_members, end_token.clone())), new_token_index + 1))
        }
      } else {
        if next_token.token_type != TokenType::RightParen {
          Err(vec![Error::new(Severity::Error, next_token.get_span(), "Unexpected token. Expected ')' right parenthesis.".to_string())])
        } else {
          Ok((Expression::SubExpression(SubExpression::new(start_token.clone(), Box::new(subexpression), next_token.clone())), next_token_index + 1))
        }
      }
    }
  }
}

fn parse_tuple_members(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize, precedence_table: &mut PrecedenceTable) -> Result<(Vec<TupleMember>, usize), Vec<Error>> {
  let mut errors = Vec::new();
  let mut tuple_members = Vec::new();
  let mut current_index = token_index;
  while current_index < tokens.len() {
    if let Either::Left(current_token) = tokens[current_index]{
      if current_token.token_type == TokenType::RightParen {
        break;
      }
    }

    let (name, colon) = match (tokens[current_index], tokens[current_index + 1]) {
      (Either::Left(id_token), Either::Left(colon_token)) if id_token.token_type == TokenType::Identifier && colon_token.token_type == TokenType::Colon => {
        current_index += 2;
        (Some(id_token.clone()), Some(colon_token.clone()))
      }
      _ => (None, None)
    };

    let (expression, next_token_index) = match parse_expression(tokens, current_index, precedence_table, 0) {
      Ok(a) => a,
      Err(mut new_errors) => {
        errors.append(&mut new_errors);
        continue;
      }
    };

    let comma_token = match tokens[next_token_index] {
      Either::Left(comma_token) if comma_token.token_type == TokenType::Comma => {
        current_index = next_token_index + 1;
        Some(comma_token.clone())
      }
      _ => {
        current_index = next_token_index;
        None
      }
    };

    tuple_members.push(TupleMember::new(name, colon, expression, comma_token));
  }

  if errors.is_empty() {
    Ok((tuple_members, current_index))
  } else {
    Err(errors)
  }
}

fn parse_type(tokens: &Vec<&Either<Token<TokenType>, AstNodeExpression>>, token_index: usize) -> Result<(Type, usize), Vec<Error>> {
  // This is very janky going from ref vec of refs to ref vec of non-refs
  let mut token_copy = tokens.iter().filter(|x| x.is_left()).map(|x| (*x).clone().expect_left("woah")).collect::<Vec<Token<TokenType>>>();
  token_copy.push(Token::new("".to_string(), TokenType::EndOfInput, (0, 0), (0, 0), (0, 0)));
  let (ast_symbol, next_token_index, mut errors) = parse_phase_one_partial(&token_copy, token_index, ParseState::Type, None);
  match (ast_symbol.clone(), errors.clone()) {
    (AstSymbol::Type(parsed_type), errors) if errors.len() == 0 => Ok((parsed_type, next_token_index)),
    _ => {
      errors.push(Error::new(Severity::Error, (token_copy[token_index].get_span().0, token_copy[next_token_index - 1].get_span().1), format!("Type parser produced an unexpected ast node :( {:?}", ast_symbol).to_string()));
      Err(errors)
    }
  }
}

fn is_literal_start_token(token: &Token<TokenType>) -> bool {
  match token.token_type {
    TokenType::String | TokenType::InterpolatedString | TokenType::Identifier | TokenType::LeftParen | TokenType::LeftSquare | TokenType::True | TokenType::False | TokenType::Number => true,
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
