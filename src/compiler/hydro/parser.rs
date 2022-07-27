use super::instruction::*;
use super::lexer::*;

fn consume_newlines(tokens: &Vec<HydroToken>, token_index: usize) -> usize {
  let mut index = token_index;
  while index < tokens.len() && tokens[index].token_type == HydroTokenType::Newline {
    index += 1;
  }
  index
}

pub fn hydro_parse(tokens: &Vec<HydroToken>) -> Vec<Instruction> {
  let mut instructions_list = Vec::new();
  let mut token_index = 0;

  while token_index < tokens.len() {
    let current_token = tokens[token_index].clone();
    let inst;
    println!("{}: {}", token_index, current_token);

    match current_token.token_type {
      HydroTokenType::Identifier => {
        (inst, token_index) = parse_operation(&tokens, token_index);
        instructions_list.push(inst);
      }
      HydroTokenType::Variable => {
        (inst, token_index) = parse_assignment(&tokens, token_index);
        instructions_list.push(inst);
      }
      HydroTokenType::Keyword => match current_token.lexeme.as_str() {
        "if" => {
          (inst, token_index) = parse_if(&tokens, token_index);
          instructions_list.push(inst);
        }
        "while" => {
          (inst, token_index) = parse_while(&tokens, token_index);
          instructions_list.push(inst);
        }
        "return" => {
          (inst, token_index) = parse_return(&tokens, token_index);
          instructions_list.push(inst);
        }
        "break" => {
          (inst, token_index) = parse_break(&tokens, token_index);
          instructions_list.push(inst);
        }
        "continue" => {
          (inst, token_index) = parse_continue(&tokens, token_index);
          instructions_list.push(inst);
        }
        "func" => {
          (inst, token_index) = parse_function(&tokens, token_index);
          instructions_list.push(inst);
        }
        "type" => {
          (inst, token_index) = parse_type(&tokens, token_index);
          instructions_list.push(inst);
        }
        _ => panic!(),
      },
      HydroTokenType::Newline => {
        token_index = consume_newlines(&tokens, token_index);
      }
      HydroTokenType::EndOfInput => break,
      _ => {
        panic!("unexpected token :(");
      }
    }
  }

  instructions_list
}

fn parse_assignment(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let var = tokens[index].clone();
    index += 1;
    if index < tokens.len() {
      let equal = tokens[index].clone();
      index += 1;
      if equal.token_type == HydroTokenType::Equal {
        let (operation_or_primary, index) = parse_operation_or_primary(tokens, index);
        (
          Instruction::Assignment(Assignment::new(var, equal, operation_or_primary)),
          index,
        )
      } else {
        panic!()
      }
    } else {
      panic!()
    }
  } else {
    panic!()
  }
}

fn parse_operation_or_primary(
  tokens: &Vec<HydroToken>,
  token_index: usize,
) -> (OperationOrPrimary, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let current_token = tokens[index].clone();
    match current_token.token_type {
      HydroTokenType::Identifier => {
        let (operation, index) = parse_operation(tokens, index);
        match operation {
          Instruction::Operation(op) => (OperationOrPrimary::Operation(op), index),
          _ => panic!(),
        }
      }
      HydroTokenType::StringLiteral
      | HydroTokenType::CharLiteral
      | HydroTokenType::BooleanLiteral
      | HydroTokenType::NumberLiteral
      | HydroTokenType::Variable => {
        index += 1;
        (OperationOrPrimary::Primary(current_token.clone()), index)
      }
      _ => panic!("{:?}", current_token),
    }
  } else {
    panic!()
  }
}

fn parse_primary(tokens: &Vec<HydroToken>, token_index: usize) -> (HydroToken, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let current_token = tokens[index].clone();
    index += 1;
    match current_token.token_type {
      HydroTokenType::StringLiteral
      | HydroTokenType::CharLiteral
      | HydroTokenType::BooleanLiteral
      | HydroTokenType::NumberLiteral
      | HydroTokenType::Variable => (current_token.clone(), index),
      _ => panic!(),
    }
  } else {
    panic!()
  }
}

fn parse_operation(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let name = tokens[index].clone();
    index += 1;
    let mut args = Vec::new();
    while index < tokens.len() {
      let current_token = tokens[index].clone();
      match current_token.token_type {
        HydroTokenType::StringLiteral
        | HydroTokenType::CharLiteral
        | HydroTokenType::BooleanLiteral
        | HydroTokenType::NumberLiteral
        | HydroTokenType::Variable => {
          let primary;
          (primary, index) = parse_primary(tokens, index);
          args.push(primary);
          println!("{}", index);
        }
        HydroTokenType::Newline => break,
        _ => panic!(),
      }
    }
    (Instruction::Operation(Operation::new(name, args)), index)
  } else {
    panic!()
  }
}

fn parse_function(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_while(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_for(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_if(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_return(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_continue(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_break(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_type(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}
