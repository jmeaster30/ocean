use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::precedencetable::PrecedenceTable;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;
use itertools::Either;
use crate::ocean::frontend::astsymbolstack::{AstSymbol, AstSymbolStack};
use crate::ocean::frontend::lexer::lex;
use crate::ocean::frontend::parsestatestack::{ParseState, ParseStateStack};

pub fn parse_phase_two(ast: &mut Program, precedence_table: &mut PrecedenceTable) {
  for statement_node in &mut ast.statements {
    match &mut statement_node.statement {
      Some(statement) => {
        parse_phase_two_statement(statement, precedence_table);
      }
      None => {}
    }
  }
}

fn parse_phase_two_statement(statement: &mut Statement, precedence_table: &mut PrecedenceTable) {
  match statement {
    Statement::WhileLoop(while_loop) => parse_phase_two_while(while_loop, precedence_table),
    Statement::ForLoop(for_loop) => parse_phase_two_for(for_loop, precedence_table),
    Statement::Loop(loop_stmt) => parse_phase_two_loop(loop_stmt, precedence_table),
    Statement::Branch(branch) => parse_phase_two_branch(branch, precedence_table),
    Statement::Match(match_stmt) => parse_phase_two_match(match_stmt, precedence_table),
    Statement::Assignment(assignment) => parse_phase_two_assignment(assignment, precedence_table),
    Statement::Function(function) => parse_phase_two_function(function, precedence_table),
    Statement::Expression(expression) => parse_phase_two_expression(&mut expression.expression_node, precedence_table),
    _ => {}
  }
}

fn parse_phase_two_compound(compound_statement: &mut CompoundStatement, precedence_table: &mut PrecedenceTable) {
  for statement_node in &mut compound_statement.body {
    match &mut statement_node.statement {
      Some(statement) => {
        parse_phase_two_statement(statement, precedence_table);
      }
      None => {}
    }
  }
}

fn parse_phase_two_while(while_loop: &mut WhileLoop, precedence_table: &mut PrecedenceTable) {
  parse_phase_two_expression(&mut while_loop.condition, precedence_table);
  parse_phase_two_compound(&mut while_loop.body, precedence_table);
}

fn parse_phase_two_for(for_loop: &mut ForLoop, precedence_table: &mut PrecedenceTable) {
  parse_phase_two_expression(&mut for_loop.iterator, precedence_table);
  parse_phase_two_expression(&mut for_loop.iterable, precedence_table);
  parse_phase_two_compound(&mut for_loop.body, precedence_table);
}

fn parse_phase_two_loop(loop_loop: &mut Loop, precedence_table: &mut PrecedenceTable) {
  parse_phase_two_compound(&mut loop_loop.body, precedence_table);
}

fn parse_phase_two_branch(branch: &mut Branch, precedence_table: &mut PrecedenceTable) {
  parse_phase_two_expression(&mut branch.condition, precedence_table);
  parse_phase_two_compound(&mut branch.body, precedence_table);
  match &mut branch.else_branch {
    Some(else_body) => match &mut else_body.body {
      Either::Left(body) => parse_phase_two_compound(body, precedence_table),
      Either::Right(else_branch) => parse_phase_two_branch(else_branch, precedence_table),
    },
    None => {}
  }
}

fn parse_phase_two_match(match_stmt: &mut Match, precedence_table: &mut PrecedenceTable) {
  todo!()
}

fn parse_phase_two_assignment(assignment: &mut Assignment, precedence_table: &mut PrecedenceTable) {
  match &mut assignment.left_expression {
    Either::Left(_) => {}
    Either::Right(expression) => parse_phase_two_expression(expression, precedence_table),
  }
  parse_phase_two_expression(&mut assignment.right_expression.expression_node, precedence_table);
}

fn parse_phase_two_function(function: &mut Function, precedence_table: &mut PrecedenceTable) {
  for result in &mut function.results {
    match &mut result.expression {
      Some(expression) => parse_phase_two_expression(expression, precedence_table),
      None => {}
    }
  }

  match &mut function.compound_statement {
    Some(compound_stmt) => parse_phase_two_compound(compound_stmt, precedence_table),
    None => {}
  }
}

fn parse_phase_two_expression(expression: &mut ExpressionNode, precedence_table: &mut PrecedenceTable) {
  let expression_tokens = expression.tokens.iter().filter(|x| x.token_type != TokenType::Newline && x.token_type != TokenType::Comment).collect::<Vec<&Token<TokenType>>>();
  let (parsed_expression, _) = parse_expression(&expression_tokens, 0, precedence_table, 0);
  expression.parsed_expression = Some(parsed_expression);
}

fn parse_expression(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable, min_precedence: usize) -> (Expression, usize) {
  //println!("tokens size: {} token_index: {} min_prec {}", tokens.len(), token_index, min_precedence);
  //println!("tokens: {:?}", tokens);
  let (mut left_hand_side, next_token_index) = if is_literal_start_token(tokens[token_index]) {
    parse_literal(tokens, token_index, precedence_table)
  } else if precedence_table.is_prefix_operator(&tokens[token_index].lexeme) {
    let prefix_precedence = precedence_table.get_prefix_precedence(&tokens[token_index].lexeme);
    let (expr, next_index) = parse_expression(tokens, token_index + 1, precedence_table, prefix_precedence);
    (Expression::PrefixOperation(PrefixOperator::new(tokens[token_index].clone(), Box::new(expr))), next_index)
  } else {
    panic!("Unexpected token in expression {}", tokens[token_index])
  };

  let mut current_token_index = next_token_index;
  while current_token_index < tokens.len() {
    if tokens[current_token_index].token_type == TokenType::RightParen || tokens[current_token_index].token_type == TokenType::RightSquare || tokens[current_token_index].token_type == TokenType::EndOfInput {
      break;
    }

    // do postfix here?

    if !precedence_table.is_binary_operator(&tokens[current_token_index].lexeme) {
      match tokens[current_token_index].token_type {
        TokenType::LeftSquare => {
          let mut arguments = Vec::new();
          let left_square_index = current_token_index;
          current_token_index += 1;
          while current_token_index < tokens.len() {
            let (index_expression, next_token_index) = parse_expression(tokens, current_token_index, precedence_table, 0);
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
            panic!("Expected end of array index but got {}", tokens[current_token_index]);
          }
          left_hand_side = Expression::ArrayIndex(ArrayIndex::new(Box::new(left_hand_side), tokens[left_square_index].clone(), arguments, tokens[current_token_index].clone()));
          current_token_index += 1;
          continue
        }
        TokenType::LeftParen => {
          let mut arguments = Vec::new();
          let left_paren_index = current_token_index;
          current_token_index += 1;
          while current_token_index < tokens.len() {
            let (index_expression, next_token_index) = parse_expression(tokens, current_token_index, precedence_table, 0);
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
            panic!("Expected end of call but got {}", tokens[current_token_index]);
          }
          left_hand_side = Expression::Call(Call::new(Box::new(left_hand_side), tokens[left_paren_index].clone(), arguments, tokens[current_token_index].clone()));
          current_token_index += 1;
          continue
        }
        _ => panic!("Unexpected token expected binary operator {}", tokens[current_token_index]),
      }
    }

    let (left_precedence, right_precedence) = precedence_table.get_binary_precedence(&tokens[current_token_index].lexeme);
    if left_precedence < min_precedence {
      break;
    }

    let (right_hand_side, next_token_index) = parse_expression(tokens, current_token_index + 1, precedence_table, right_precedence);
    left_hand_side = Expression::BinaryOperation(BinaryOperator::new(Box::new(left_hand_side), tokens[current_token_index].clone(), Box::new(right_hand_side)));
    current_token_index = next_token_index
  }

  (left_hand_side, current_token_index)
}

fn parse_literal(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> (Expression, usize) {
  let (expression, next_token_index) = match tokens[token_index].token_type {
    TokenType::String => (Expression::String(StringLiteral::new(tokens[token_index].clone())), token_index + 1),
    TokenType::InterpolatedString => {
      let expressions = parse_interpolated_string(&tokens[token_index].lexeme, precedence_table);
      (Expression::InterpolatedString(InterpolatedString::new(tokens[token_index].clone(), expressions)), token_index + 1)
    }
    TokenType::True | TokenType::False => (Expression::Boolean(Boolean::new(tokens[token_index].clone())), token_index + 1),
    TokenType::Number => (Expression::Number(Number::new(tokens[token_index].clone())), token_index + 1),
    TokenType::LeftParen => parse_sub_expression_or_tuple(tokens, token_index, precedence_table),
    TokenType::Identifier => (Expression::Variable(Variable::new(tokens[token_index].clone())), token_index + 1),
    _ => panic!("Unexpected case??? {}", tokens[token_index]),
  };

  let current_token_index = next_token_index;
  if current_token_index >= tokens.len() {
    return (expression, next_token_index);
  }

  if tokens[current_token_index].token_type == TokenType::As {
    let (parsed_type, next_token_index) = parse_type(tokens, current_token_index + 1);
    (Expression::Cast(Cast::new(Box::new(expression), tokens[current_token_index].clone(), parsed_type)), next_token_index)
  } else {
    (expression, current_token_index)
  }
}

fn parse_arguments(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> (Argument, usize) {
  let (index_expression, next_token_index) = parse_expression(tokens, token_index, precedence_table, 0);
  if tokens[next_token_index].token_type == TokenType::Comma {
    (Argument::new(index_expression, Some(tokens[next_token_index].clone())), next_token_index + 1)
  } else {
    (Argument::new(index_expression, None), next_token_index)
  }
}

fn parse_interpolated_string(lexeme: &String, precedence_table: &PrecedenceTable) -> Vec<Expression> {
  println!("parsing interpolated string :)");
  let chars = lexeme.chars().collect::<Vec<char>>();

  let mut expressions = Vec::new();
  let mut index = 0;
  let mut in_expression = false;
  let mut current_expression_chars = Vec::new();
  while index < chars.len() {
    let current_char = chars[index];
    if current_char != '{' && current_char != '}' && !in_expression {
    } else if current_char == '{' && !in_expression { // && current_char == '{' || current_char == '}'
      in_expression = true;
    } else if current_char == '{' && in_expression && current_expression_chars.len() == 0 {
      in_expression = false;
    } else if current_char == '}' && in_expression {
      let tokens = lex(current_expression_chars.iter().collect::<String>());
      let clean_tokens = tokens.iter().filter(|x| x.token_type != TokenType::Newline && x.token_type != TokenType::Comment).collect::<Vec<&Token<TokenType>>>();
      let (expression, _) = parse_expression(&clean_tokens, 0, precedence_table, 0);
      expressions.push(expression);
      current_expression_chars.clear();
      in_expression = false;
    } else if current_char == '}' && !in_expression {
      // TODO I think we need to make it so it required an escaped closing brace so the escaped brackets can be symmetric.
    } else if in_expression {
      current_expression_chars.push(current_char);
    } else {
      panic!("char {}, in_expression {}", current_char, in_expression);
    }
    index += 1;
  }


  println!("done");
  expressions
}

fn parse_sub_expression_or_tuple(tokens: &Vec<&Token<TokenType>>, token_index: usize, precedence_table: &PrecedenceTable) -> (Expression, usize) {
  // TODO need to add parsing for tuples here
  if tokens[token_index].token_type != TokenType::LeftParen {
    panic!("Unexpected token :( {}", tokens[token_index]);
  }

  let left_paren = tokens[token_index].clone();
  let (subexpression, next_token_index) = parse_expression(tokens, token_index + 1, precedence_table, 0);

  if tokens[next_token_index].token_type != TokenType::RightParen {
    panic!("Unexpected token :( {}", tokens[next_token_index]);
  }

  (Expression::SubExpression(SubExpression::new(left_paren, Box::new(subexpression), tokens[next_token_index].clone())), next_token_index)
}

fn parse_type(tokens: &Vec<&Token<TokenType>>, token_index: usize) -> (Type, usize) {
  // TODO this is copied from the type section of the phase 1 parser. Need to make it so we can call this part of the phase 1 parse directly instead of copying it
  let mut parser_state_stack = ParseStateStack::new();
  let mut ast_stack = AstSymbolStack::new();
  let mut token_index = token_index;

  parser_state_stack.push(ParseState::Type);

  loop {
    if token_index >= tokens.len() {
      break;
    }

    let current_token = tokens[token_index];
    let current_state = parser_state_stack.current_state();
    let current_ast_symbol = ast_stack.peek();

    //println!("{} > {} - {:?}", token_index, current_token, current_ast_symbol);

    match (current_state, current_ast_symbol, &current_token.token_type) {
      (Some(ParseState::Type), None, TokenType::Identifier) => {
        parser_state_stack.goto(ParseState::TypeArray);
        ast_stack.push(AstSymbol::Type(Type::Custom(CustomType::new(current_token.clone()))));
        token_index += 1;
      }
      (Some(ParseState::Type), None, TokenType::Type) => {
        parser_state_stack.goto(ParseState::TypeArray);
        ast_stack.push(AstSymbol::Type(Type::Base(BaseType::new(current_token.clone()))));
        token_index += 1;
      }
      (Some(ParseState::Type), Some(_), TokenType::RightSquare) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::Type), None, TokenType::TypePrefix) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeArray);
        match current_token.lexeme.as_str() {
          "auto" => {
            parser_state_stack.push(ParseState::TypeAuto);
            parser_state_stack.push(ParseState::TypeIdentifier);
          }
          "lazy" => {
            parser_state_stack.push(ParseState::TypeLazy);
            parser_state_stack.push(ParseState::Type);
          }
          "ref" => {
            parser_state_stack.push(ParseState::TypeRef);
            parser_state_stack.push(ParseState::Type);
          }
          "mut" => {
            parser_state_stack.push(ParseState::TypeMut);
            parser_state_stack.push(ParseState::Type);
          }
          _ => panic!("Unexpected type prefix"),
        }
      }
      (Some(ParseState::Type), None, TokenType::Function) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionParams);
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) | (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) | (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Newline) | (Some(ParseState::TypeFunctionParams), Some(_), TokenType::Comment) => {
        token_index += 1;
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index += 1;
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        ast_stack.push(AstSymbol::FunctionTypeArguments(Vec::new()));
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::Type(param_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, None));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
            ast_stack.push(AstSymbol::Token(current_token.clone()));
            token_index += 1;
            parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionParams), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::RightParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionOptArrow);
      }
      (Some(ParseState::TypeFunctionOptArrow), Some(_), TokenType::Arrow) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeFunctionReturns);
      }
      (Some(ParseState::TypeFunctionOptArrow), Some(AstSymbol::Token(right_paren)), _) => {
        ast_stack.pop();
        let param_types_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let function_token_sym = ast_stack.pop_panic();
        match (function_token_sym, left_paren_sym, param_types_sym) {
          (AstSymbol::Token(function_token), AstSymbol::Token(left_paren), AstSymbol::FunctionTypeArguments(param_types)) => {
            ast_stack.push(AstSymbol::Type(Type::Function(FunctionType::new(function_token, left_paren, param_types, right_paren, None, None, Vec::new(), None))));
            parser_state_stack.goto(ParseState::TypeArray);
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Identifier) | (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::Type) | (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(_)), TokenType::TypePrefix) => {
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Newline) | (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::Comment) => {
        token_index += 1;
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::Comma) => {
        token_index += 1;
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, Some(current_token.clone())));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(_), TokenType::LeftParen) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        ast_stack.push(AstSymbol::FunctionTypeArguments(Vec::new()));
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::Type(param_type)), TokenType::RightParen) => {
        ast_stack.pop();
        let args_sym = ast_stack.pop_panic();
        match args_sym {
          AstSymbol::FunctionTypeArguments(mut args) => {
            args.push(FunctionTypeArgument::new(param_type, None));
            ast_stack.push(AstSymbol::FunctionTypeArguments(args));
          }
          _ => panic!("invalid state :("),
        }
      }
      (Some(ParseState::TypeFunctionReturns), Some(AstSymbol::FunctionTypeArguments(return_args)), TokenType::RightParen) => {
        ast_stack.pop();
        let return_left_paren_sym = ast_stack.pop_panic();
        let arrow_sym = ast_stack.pop_panic();
        let right_paren_sym = ast_stack.pop_panic();
        let params_sym = ast_stack.pop_panic();
        let left_paren_sym = ast_stack.pop_panic();
        let function_sym = ast_stack.pop_panic();
        match (function_sym.clone(), left_paren_sym.clone(), params_sym.clone(), right_paren_sym.clone(), arrow_sym.clone(), return_left_paren_sym.clone()) {
          (AstSymbol::Token(function_token), AstSymbol::Token(left_paren), AstSymbol::FunctionTypeArguments(params), AstSymbol::Token(right_paren), AstSymbol::Token(arrow_token), AstSymbol::Token(return_left_paren)) => {
            ast_stack.push(AstSymbol::Type(Type::Function(FunctionType::new(function_token, left_paren, params, right_paren, Some(arrow_token), Some(return_left_paren), return_args, Some(current_token.clone())))));
            token_index += 1;
            parser_state_stack.goto(ParseState::TypeArray);
          }
          _ => panic!("invalid state :(\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}", function_sym, left_paren_sym, params_sym, right_paren_sym, arrow_sym, return_left_paren_sym),
        }
      }
      (Some(ParseState::TypeIdentifier), Some(_), TokenType::Identifier) => {
        token_index += 1;
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeAuto), Some(AstSymbol::Token(identifier_token)), _) => {
        ast_stack.pop();
        let auto_token_sym = ast_stack.pop_panic();
        match auto_token_sym {
          AstSymbol::Token(auto_token) => {
            ast_stack.push(AstSymbol::Type(Type::Auto(AutoType::new(auto_token, identifier_token))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeLazy), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let lazy_token_sym = ast_stack.pop_panic();
        match lazy_token_sym {
          AstSymbol::Token(lazy_token) => {
            ast_stack.push(AstSymbol::Type(Type::Lazy(LazyType::new(lazy_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeRef), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let ref_token_sym = ast_stack.pop_panic();
        match ref_token_sym {
          AstSymbol::Token(ref_token) => {
            ast_stack.push(AstSymbol::Type(Type::Ref(RefType::new(ref_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeMut), Some(AstSymbol::Type(subtype)), _) => {
        ast_stack.pop();
        let mut_token_sym = ast_stack.pop_panic();
        match mut_token_sym {
          AstSymbol::Token(mut_token) => {
            ast_stack.push(AstSymbol::Type(Type::Mutable(MutType::new(mut_token, Box::new(subtype)))));
          }
          _ => panic!("Unexpected state :("),
        }
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArray), Some(_), TokenType::LeftSquare) => {
        ast_stack.push(AstSymbol::Token(current_token.clone()));
        token_index += 1;
        parser_state_stack.goto(ParseState::TypeArrayEnd);
        parser_state_stack.push(ParseState::Type);
      }
      (Some(ParseState::TypeArray), Some(AstSymbol::Type(base_type)), TokenType::Spread) => {
        ast_stack.pop();
        ast_stack.push(AstSymbol::Type(Type::VariableType(VariableType::new(Box::new(base_type), current_token.clone()))));
        token_index += 1;
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArray), Some(_), _) => {
        parser_state_stack.pop();
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Type(subtype)), TokenType::RightSquare) => {
        ast_stack.pop();
        let left_square_sym = ast_stack.pop_panic();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index += 1;
        match (main_type_sym, left_square_sym) {
          (AstSymbol::Type(main_type), AstSymbol::Token(left_square)) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, Some(Box::new(subtype)), current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :("),
        }
      }
      (Some(ParseState::TypeArrayEnd), Some(AstSymbol::Token(left_square)), TokenType::RightSquare) => {
        ast_stack.pop();
        let main_type_sym = ast_stack.pop_panic();
        parser_state_stack.pop();
        token_index += 1;
        match main_type_sym {
          AstSymbol::Type(main_type) => {
            ast_stack.push(AstSymbol::Type(Type::Array(ArrayType::new(Box::new(main_type), left_square, None, current_token.clone()))));
          }
          _ => panic!("Unexpected stack state :("),
        }
      }
      (a, b, c) => {
        ast_stack.print();
        parser_state_stack.print();
        println!("TOKEN:::: {}", current_token);
        panic!("Unexpected state {:?} {:?} {:?}", a, b, c)
      }
    }
  }

  if ast_stack.size() != 1 {
    panic!("Too many things in the ast stack");
  }

  match ast_stack.pop_panic() {
    AstSymbol::Type(parsed_type) => (parsed_type, token_index),
    _ => panic!("Unexpected ast stack symbol"),
  }
}

fn is_literal_start_token(token: &Token<TokenType>) -> bool {
  match token.token_type {
    TokenType::String | TokenType::InterpolatedString | TokenType::Identifier | TokenType::LeftParen | TokenType::True | TokenType::False | TokenType::Number => true,
    _ => false,
  }
}
