use super::{
  instruction::{
    Assignment, Function, If, Instruction, Loop, Operation, OperationOrPrimary, Primary, Return,
    Type, TypeDefinition,
  },
  lexer::{HydroToken, HydroTokenType},
  symboltable::HydroSymbolTable,
  symboltable::HydroSymbol,
  symboltable::HydroType,
};
use std::collections::HashMap;
use crate::util::errors::{OceanError, Severity};

pub fn hydro_semantic_check(
  instructions: &Vec<Instruction>,
) -> (Vec<Instruction>, Option<HydroSymbolTable>, Vec<OceanError>) {
  let mut symbol_table = HydroSymbolTable::new(None);
  let mut errors = Vec::new();
  let mut typed_instructions = instructions.clone();

  for inst in &mut typed_instructions {
    typecheck_instruction(inst, &mut symbol_table, &mut errors);
  }

  (typed_instructions, Some(symbol_table), errors)
}

fn typecheck_instruction(
  instruction: &mut Instruction,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  match instruction {
    Instruction::Operation(operation) => {typecheck_operation(operation, symbol_table, errors);}
    Instruction::Assignment(assignment) => typecheck_assignment(assignment, symbol_table, errors),
    Instruction::If(if_statement) => typecheck_if(if_statement, symbol_table, errors),
    Instruction::Loop(loop_statement) => typecheck_loop(loop_statement, symbol_table, errors),
    Instruction::TypeDefinition(type_def) => {
      typecheck_typedefinition(type_def, symbol_table, errors)
    }
    Instruction::Function(function) => typecheck_function(function, symbol_table, errors),
    Instruction::Return(ret) => typecheck_return(ret, symbol_table, errors),
    Instruction::Break => (),
    Instruction::Continue => (),
  };
}

fn typecheck_operation(operation: &mut Operation, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  let mut args = Vec::new();
  for arg in operation.arguments {
    args.push(typecheck_primary(&mut arg, symbol_table, errors))
  }

  match symbol_table.get_function_return_type_id(operation.identifier.lexeme, args) {
    Some(x) => x,
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (operation.identifier.start, operation.identifier.end), 
        "Function not found with argument types :(".to_string()
      ));
      symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown))
    }
  }
}

fn typecheck_operation_primary(operation_primary: &mut OperationOrPrimary, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  match operation_primary {
    OperationOrPrimary::Operation(op) => typecheck_operation(op, symbol_table, errors),
    OperationOrPrimary::Primary(prim) => typecheck_primary(prim, symbol_table, errors),
  }
}

fn typecheck_assignment(assignment: &mut Assignment, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) {
  let exp_id = typecheck_operation_primary(&mut assignment.operation, symbol_table, errors);
  match symbol_table.get_variable_type_id(assignment.identifier.lexeme) {
    Some(x) => {
      errors.push(OceanError::SemanticError(
        Severity::Error, 
        (assignment.identifier.start, assignment.identifier.end),
        "Variable cannot be assigned twice :(".to_string()
      ));
    }
    None => {
      symbol_table.add_variable(assignment.identifier.lexeme, exp_id)
    }
  }
}

fn typecheck_if(if_stmt: &mut If, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) {
  let condition_id = typecheck_operation_primary(&mut if_stmt.condition, symbol_table, errors);
  if !symbol_table.matches_bool(condition_id) {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      (0, 0), // TODO
      "If condition must evaluate to a boolean".to_string()
    ));
  }

  for inst in if_stmt.true_body {
    typecheck_instruction(&mut inst, symbol_table, errors)
  }

  for inst in if_stmt.else_body {
    typecheck_instruction(&mut inst, symbol_table, errors)
  }
}

fn typecheck_loop(loop_stmt: &mut Loop, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) {
  for inst in loop_stmt.body {
    typecheck_instruction(&mut inst, symbol_table, errors)
  }
}

fn typecheck_return(ret: &mut Return, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) {
  let ret_id = typecheck_operation_primary(&mut ret.value, symbol_table, errors);
  match symbol_table.return_type_id {
    Some(x) => if ret_id != x {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (0, 0), // TODO
        "Return type doesn't match expected return type".to_string()
      ));
    }
    None => {
      symbol_table.return_type_id = Some(ret_id)
    }
  }
}

fn typecheck_typedefinition(type_def: &mut TypeDefinition, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) {
  let add_type_def = match symbol_table.get_type_id(type_def.identifier.lexeme) {
    Some(x) => true,
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error, 
        (type_def.identifier.start, type_def.identifier.end), 
        "Type already defined :(".to_string()
      ));
      false
    }
  };
  
  let custom_type = HashMap::new();
  for typevar in type_def.entries {
    match custom_type.get(&typevar.identifier.lexeme) {
      Some(_) => {
        errors.push(OceanError::SemanticError(
          Severity::Error, 
          (typevar.identifier.start, typevar.identifier.end),
          "Type member already defined for this type".to_string()
        ));
      }
      None => {
        let tid = symbol_table.add_symbol(symbol_table.get_symbol_from_type(typevar.type_def));
        custom_type.insert(typevar.identifier.lexeme, tid);
      }
    }
  }
  if add_type_def {
    symbol_table.add_type(type_def.identifier.lexeme, custom_type);
  }
}

fn typecheck_primary(primary: &mut Primary, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  match primary.token.token_type {
    HydroTokenType::Identifier => todo!(),
    HydroTokenType::StringLiteral => if primary.token.lexeme.len() == 1 {
      symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned8))
    } else {
      symbol_table.add_symbol(HydroSymbol::Base(HydroType::String))
    },
    HydroTokenType::BooleanLiteral => {
      symbol_table.add_symbol(HydroSymbol::Base(HydroType::Bool))
    },
    HydroTokenType::NumberLiteral => {
      let lex = primary.token.lexeme.as_str();
      let value = match lex.parse::<i128>() {
        Ok(x) => Some(x),
        Err(_) => None,
      };
      match value {
        Some(x) if x >= 0 && x < 256 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned8))
        }
        Some(x) if x >= 0 && x < 65536 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned16))
        }
        Some(x) if x >= 0 && x < 4294967296 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned32))
        }
        Some(x) if x >= 0 && x < 18446744073709551616 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned64))
        }
        Some(x) if x >= -128 && x < 128 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Signed8))
        }
        Some(x) if x >= -32768 && x < 32768 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Signed16))
        }
        Some(x) if x >= -2147483648 && x < 2147483648 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Signed32))
        }
        Some(x) if x >= -9223372036854775808 && x < 9223372036854775808 => {
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Signed64))
        }
        None if lex.contains('.') => {
          //TODO make this better
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Float64))
        }
        _ => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            (primary.token.start, primary.token.end),
            "This number is not representable by any of our number types :(.".to_string(),
          ));
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown))
        }
      }
    },
    HydroTokenType::Variable => todo!(),
    _ => {
      panic!("Unexpected primary")
    }
  }
}
