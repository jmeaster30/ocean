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
  let (instruction_list, _) = parse_instructions(tokens, 0, HydroTokenType::EndOfInput);
  instruction_list
}

fn parse_instructions(
  tokens: &Vec<HydroToken>,
  token_index: usize,
  end: HydroTokenType,
) -> (Vec<Instruction>, usize) {
  let mut instructions_list = Vec::new();
  let mut index = token_index;

  while index < tokens.len() && tokens[index].token_type != end {
    let current_token = tokens[index].clone();
    let inst;
    println!("{}: {}", index, current_token);

    match current_token.token_type {
      HydroTokenType::Identifier => {
        (inst, index) = parse_operation(&tokens, index);
        instructions_list.push(inst);
      }
      HydroTokenType::Variable => {
        (inst, index) = parse_assignment(&tokens, index);
        instructions_list.push(inst);
      }
      HydroTokenType::Type => match current_token.lexeme.as_str() {
        "func" => {
          (inst, index) = parse_function(&tokens, index);
          instructions_list.push(inst);
        }
        _ => panic!("{:?}", current_token),
      },
      HydroTokenType::Keyword => match current_token.lexeme.as_str() {
        "if" => {
          (inst, index) = parse_if(&tokens, index);
          instructions_list.push(inst);
        }
        "while" => {
          (inst, index) = parse_while(&tokens, index);
          instructions_list.push(inst);
        }
        "return" => {
          (inst, index) = parse_return(&tokens, index);
          instructions_list.push(inst);
        }
        "break" => {
          (inst, index) = parse_break(&tokens, index);
          instructions_list.push(inst);
        }
        "continue" => {
          (inst, index) = parse_continue(&tokens, index);
          instructions_list.push(inst);
        }
        "type" => {
          (inst, index) = parse_type_def(&tokens, index);
          instructions_list.push(inst);
        }
        _ => panic!("{:?}", current_token),
      },
      HydroTokenType::Newline => {
        index = consume_newlines(&tokens, index);
      }
      HydroTokenType::EndOfInput => break,
      _ => {
        panic!("unexpected token :(   {}", current_token);
      }
    }
  }

  (instructions_list, index)
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
        panic!("{:?}", tokens[index])
      }
    } else {
      panic!("out of range {} > {}", token_index, tokens.len())
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
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
          _ => panic!("{:?}", tokens[index]),
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
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_primary(tokens: &Vec<HydroToken>, token_index: usize) -> (HydroToken, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let current_token = tokens[index].clone();
    index += 1;
    match current_token.token_type {
      HydroTokenType::Identifier
      | HydroTokenType::StringLiteral
      | HydroTokenType::CharLiteral
      | HydroTokenType::BooleanLiteral
      | HydroTokenType::NumberLiteral
      | HydroTokenType::Variable => (current_token.clone(), index),
      _ => panic!("{:?}", tokens[token_index]),
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
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
        HydroTokenType::Identifier
        | HydroTokenType::StringLiteral
        | HydroTokenType::CharLiteral
        | HydroTokenType::BooleanLiteral
        | HydroTokenType::NumberLiteral
        | HydroTokenType::Variable => {
          let primary;
          (primary, index) = parse_primary(tokens, index);
          args.push(primary);
        }
        HydroTokenType::Newline | HydroTokenType::LCurly | HydroTokenType::EndOfInput => break,
        _ => panic!("{:?}", tokens[index]),
      }
    }
    (Instruction::Operation(Operation::new(name, args)), index)
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_function(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    let func_token = tokens[index].clone();
    index += 1;
    let func_name = match tokens[index].token_type {
      HydroTokenType::Identifier => tokens[index].clone(),
      _ => panic!("{:?}", tokens[index]),
    };
    index += 1;
    let (func_params, new_index) = parse_function_parameters(tokens, index);
    let (func_return_type, new_new_index) = parse_optional_type(tokens, new_index);
    let (func_body, new_new_new_index) = parse_compound(tokens, new_new_index);
    index = new_new_new_index;
    (
      Instruction::Function(Function::new(
        func_token,
        func_name,
        func_params,
        func_return_type,
        func_body,
      )),
      index,
    )
  } else {
    panic!("out of range {} > {}", token_index, tokens.len());
  }
}

fn parse_function_parameters(
  tokens: &Vec<HydroToken>,
  token_index: usize,
) -> (Vec<TypeVar>, usize) {
  let mut index = token_index;
  if tokens[index].token_type == HydroTokenType::LParen {
    let left_paren = tokens[index].clone();
    index += 1;
    let mut params = Vec::new();
    while index < tokens.len() {
      let (param, new_index) = parse_type_var(tokens, index);
      params.push(param);
      index = new_index;
      if tokens[index].token_type == HydroTokenType::RParen {
        index += 1;
        break;
      } else if tokens[index].token_type == HydroTokenType::Comma {
        index += 1;
      } else {
        panic!("{:?}", tokens[index]);
      }
    }
    (params, index)
  } else {
    (Vec::new(), token_index)
  }
}

fn parse_optional_type(tokens: &Vec<HydroToken>, token_index: usize) -> (Option<Type>, usize) {
  if token_index < tokens.len() {
    match tokens[token_index].token_type {
      HydroTokenType::LCurly => (None, token_index),
      _ => {
        let (t, idx) = parse_type(tokens, token_index);
        println!("{:?}", t);
        (Some(t), idx)
      }
    }
  } else {
    println!("empty return type");
    (None, token_index)
  }
}

fn parse_type(tokens: &Vec<HydroToken>, token_index: usize) -> (Type, usize) {
  if token_index < tokens.len() {
    match tokens[token_index].token_type {
      HydroTokenType::Identifier | HydroTokenType::Type => (
        Type::BaseType(BaseType::new(tokens[token_index].clone())),
        token_index + 1,
      ),
      _ => panic!("{:?}", tokens[token_index]),
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_compound(tokens: &Vec<HydroToken>, token_index: usize) -> (Vec<Instruction>, usize) {
  let mut index = token_index;
  if index < tokens.len() {
    match tokens[index].token_type {
      HydroTokenType::LCurly => {
        index += 1;
        let (instructions, new_index) = parse_instructions(tokens, index, HydroTokenType::RCurly);
        index = new_index;
        match tokens[index].token_type {
          HydroTokenType::RCurly => {
            index += 1;
            println!("here {}", index);
            (instructions, index)
          }
          _ => panic!("{:?}", tokens[new_index]),
        }
      }
      _ => panic!("{:?}", tokens[index]),
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_while(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  if token_index < tokens.len() {
    let (operation, index) = parse_operation_or_primary(tokens, token_index + 1);
    let (insts, new_index) = parse_compound(tokens, index);
    (Instruction::Loop(Loop::new(operation, insts)), new_index)
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_if(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  if token_index < tokens.len() {
    let (operation, index) = parse_operation_or_primary(tokens, token_index + 1);
    let (true_insts, new_index) = parse_compound(tokens, index);
    match tokens[new_index].token_type {
      HydroTokenType::Keyword if tokens[new_index].lexeme == "else" => {
        // TODO add else if
        let (false_insts, new_new_index) = parse_compound(tokens, new_index + 1);
        (
          Instruction::If(If::new(operation, true_insts, false_insts)),
          new_new_index,
        )
      }
      _ => (
        Instruction::If(If::new(operation, true_insts, Vec::new())),
        new_index,
      ),
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_return(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  if token_index < tokens.len() {
    let (operation, index) = parse_operation_or_primary(tokens, token_index + 1);
    (Instruction::Return(Return::new(operation)), index)
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_continue(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  if token_index < tokens.len() {
    (Instruction::Continue, token_index + 1)
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_break(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  if token_index < tokens.len() {
    (Instruction::Break, token_index + 1)
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}

fn parse_type_def(tokens: &Vec<HydroToken>, token_index: usize) -> (Instruction, usize) {
  todo!()
}

fn parse_type_var(tokens: &Vec<HydroToken>, token_index: usize) -> (TypeVar, usize) {
  if token_index < tokens.len() {
    let mut index = token_index;
    match tokens[index].token_type {
      HydroTokenType::Variable => {
        let var_name = tokens[index].clone();
        index += 1;
        match tokens[index].token_type {
          HydroTokenType::Colon => {
            index += 1;
            let (var_type, new_index) = parse_type(tokens, index);
            (TypeVar::new(var_name, var_type), new_index)
          }
          _ => panic!("{:?}", tokens[index].token_type),
        }
      }
      _ => panic!("{:?}", tokens[index].token_type),
    }
  } else {
    panic!("out of range {} > {}", token_index, tokens.len())
  }
}
