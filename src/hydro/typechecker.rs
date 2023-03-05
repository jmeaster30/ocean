use super::{
  instruction::{
    Assignment, Function, If, Instruction, Loop, Operation, OperationOrPrimary, Primary, Return,
    Type, TypeDefinition, TypeVar,
  },
  lexer::{HydroToken, HydroTokenType},
  symboltable::HydroSymbol,
  symboltable::HydroSymbolTable,
  symboltable::HydroType,
};
use crate::util::{
  errors::{OceanError, Severity},
  span::Spanned,
};
use std::collections::HashMap;

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
    Instruction::Operation(operation) => {
      typecheck_operation(operation, symbol_table, errors);
    }
    Instruction::Assignment(assignment) => typecheck_assignment(assignment, symbol_table, errors),
    Instruction::If(if_statement) => typecheck_if(if_statement, symbol_table, errors),
    Instruction::Loop(loop_statement) => typecheck_loop(loop_statement, symbol_table, errors),
    Instruction::TypeDefinition(type_def) => {
      typecheck_typedefinition(type_def, symbol_table, errors)
    }
    Instruction::Function(function) => typecheck_function(function, symbol_table, errors),
    Instruction::Return(ret) => typecheck_return(ret, symbol_table, errors),
    Instruction::Break(_) => (),
    Instruction::Continue(_) => (),
  };
}

fn typecheck_function(
  function: &mut Function,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  // typecheck args
  let args = typecheck_typevars(function.parameter_list.clone(), symbol_table, errors);
  // typecheck return type
  let ret = match &function.return_type {
    Some(ret_type) => {
      match symbol_table.get_symbol_from_type(ret_type.clone()) {
        Some(symbol) => symbol_table.add_symbol(symbol),
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            ret_type.get_span(), 
            "Unknown type :(".to_string()
          ));
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown))
        } 
      }
    }
    None => symbol_table.add_symbol(HydroSymbol::Base(HydroType::Void)),
  };

  // add function to the symbol table
  match symbol_table.add_function(function.identifier.lexeme.clone(), args.clone(), ret) {
    Some(_) => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (function.identifier.start, function.identifier.end),
        "Function already defined".to_string()
      ));
    }
    None => {}
  }

  // create sub scope
  let mut sub_scope = HydroSymbolTable::new(Some(Box::new(symbol_table.clone())));
  // add args to sub scope
  for (type_var, type_var_id) in function.parameter_list.iter().zip(args.iter()) {
    sub_scope.add_variable(type_var.identifier.lexeme.clone(), *type_var_id)
  }
  // set return type id
  sub_scope.return_type_id = Some(ret);

  // type check function body
  for inst in &mut function.body {
    typecheck_instruction(inst, &mut sub_scope, errors)
  }
}

fn typecheck_typevars(
  typevars: Vec<TypeVar>,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) -> Vec<u64> {
  let mut results = Vec::new();
  for typevar in typevars {
    match symbol_table.get_symbol_from_type(typevar.type_def.clone()) {
      Some(symbol) => {
        results.push(symbol_table.add_symbol(symbol));
      }
      None => {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          typevar.type_def.get_span(),
          "Unknown type :(".to_string(),
        ));
        results.push(symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown)));
      }
    }
  }
  results
}

fn typecheck_operation(
  operation: &mut Operation,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  let mut args = Vec::new();
  for arg in &mut operation.arguments {
    args.push(typecheck_primary(arg, symbol_table, errors))
  }

  match symbol_table.get_function_return_type_id(operation.identifier.lexeme.clone(), args) {
    Some(x) => x,
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        operation.get_span(),
        "Function not found with argument types :(".to_string(),
      ));
      symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown))
    }
  }
}

fn typecheck_operation_primary(
  operation_primary: &mut OperationOrPrimary,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  match operation_primary {
    OperationOrPrimary::Operation(op) => typecheck_operation(op, symbol_table, errors),
    OperationOrPrimary::Primary(prim) => typecheck_primary(prim, symbol_table, errors),
  }
}

fn typecheck_assignment(
  assignment: &mut Assignment,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let exp_id = typecheck_operation_primary(&mut assignment.operation, symbol_table, errors);
  match symbol_table.get_variable_type_id(assignment.identifier.lexeme.clone()) {
    Some(x) => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (assignment.identifier.start, assignment.identifier.end),
        "Variable cannot be assigned twice :(".to_string(),
      ));
    }
    None => symbol_table.add_variable(assignment.identifier.lexeme.clone(), exp_id),
  }
}

fn typecheck_if(
  if_stmt: &mut If,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let condition_id = typecheck_operation_primary(&mut if_stmt.condition, symbol_table, errors);
  if !symbol_table.matches_bool(condition_id) {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      if_stmt.condition.get_span(),
      "If condition must evaluate to a boolean".to_string(),
    ));
  }

  for inst in &mut if_stmt.true_body {
    typecheck_instruction(inst, symbol_table, errors)
  }

  for inst in &mut if_stmt.else_body {
    typecheck_instruction(inst, symbol_table, errors)
  }
}

fn typecheck_loop(
  loop_stmt: &mut Loop,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  for inst in &mut loop_stmt.body {
    typecheck_instruction(inst, symbol_table, errors)
  }
}

fn typecheck_return(
  ret: &mut Return,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let ret_id = typecheck_operation_primary(&mut ret.value, symbol_table, errors);
  match symbol_table.return_type_id {
    Some(x) => {
      if ret_id != x {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          ret.value.get_span(),
          "Return type doesn't match expected return type".to_string(),
        ));
      }
    }
    None => symbol_table.return_type_id = Some(ret_id),
  }
}

fn typecheck_typedefinition(
  type_def: &mut TypeDefinition,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let add_type_def = match symbol_table.get_type_id(type_def.identifier.lexeme.clone()) {
    Some(x) => true,
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (type_def.identifier.start, type_def.identifier.end),
        "Type already defined :(".to_string(),
      ));
      false
    }
  };

  let mut custom_type = HashMap::new();
  for typevar in &type_def.entries {
    match custom_type.get(&typevar.identifier.lexeme) {
      Some(_) => {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          (typevar.identifier.start, typevar.identifier.end),
          "Type member already defined for this type".to_string(),
        ));
      }
      None => match symbol_table.get_symbol_from_type(typevar.type_def.clone()) {
        Some(symbol) => {
          let tid = symbol_table.add_symbol(symbol);
          custom_type.insert(typevar.identifier.lexeme.clone(), tid);
        }
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            typevar.type_def.get_span(),
            "Unknown type :(".to_string(),
          ));
        }
      },
    }
  }
  if add_type_def {
    symbol_table.add_type(type_def.identifier.lexeme.clone(), custom_type);
  }
}

fn typecheck_primary(
  primary: &mut Primary,
  symbol_table: &mut HydroSymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  match primary.token.token_type {
    HydroTokenType::Identifier => todo!(),
    HydroTokenType::StringLiteral => {
      if primary.token.lexeme.len() == 1 {
        symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unsigned8))
      } else {
        symbol_table.add_symbol(HydroSymbol::Base(HydroType::String))
      }
    }
    HydroTokenType::BooleanLiteral => symbol_table.add_symbol(HydroSymbol::Base(HydroType::Bool)),
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
          //TODO make this choose between f32 and f64 instead of just defaulting to f64
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
    }
    HydroTokenType::Variable => {
      match symbol_table.get_variable_type_id(primary.token.lexeme.clone()) {
        Some(type_id) => type_id,
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            primary.get_span(),
            "Unknown variable :(".to_string(),
          ));
          symbol_table.add_symbol(HydroSymbol::Base(HydroType::Unknown))
        }
      }
    }
    _ => {
      panic!("Unexpected primary")
    }
  }
}
