use super::{instruction::{Instruction, Operation, Assignment, If, Loop, Function, Type, Return, OperationOrPrimary, Primary, TypeDefinition}, symboltable::HydroSymbolTable, lexer::{HydroToken, HydroTokenType}};
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

fn typecheck_instruction(instruction: &mut Instruction, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  match instruction {
    Instruction::Operation(operation) => typecheck_operation(operation, symbol_table, errors),
    Instruction::Assignment(assignment) => typecheck_assignment(assignment, symbol_table, errors),
    Instruction::If(if_statement) => typecheck_if(if_statement, symbol_table, errors),
    Instruction::Loop(loop_statement) => typecheck_loop(loop_statement, symbol_table, errors),
    Instruction::TypeDefinition(type_def) => typecheck_typedefinition(type_def, symbol_table, errors),
    Instruction::Function(function) => typecheck_function(function, symbol_table, errors),
    Instruction::Return(ret) => typecheck_return(ret, symbol_table, errors),
    Instruction::Break => 1,
    Instruction::Continue => 1,
  }
}

fn typecheck_operation(operation: &mut Operation, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  let mut op = (operation.identifier.lexeme.clone(), Vec::new());

  for arg in &mut operation.arguments {
    let arg_type_id = typecheck_primary(arg, symbol_table, errors);
    op.1.push(arg_type_id);
  }

  match symbol_table.find_function(op) {
    Some(x) => x,
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        (operation.identifier.start, operation.identifier.end),
      "Unrecognized operation :(".to_string()));
      0
    }
  }
}

fn typecheck_primary(primary: &mut Primary, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  match primary.token.token_type {
    HydroTokenType::Identifier => {
      // TODO search for variable and return type id
      0
    },
    HydroTokenType::Variable => {
      // TODO search for variable and return type id
      0
    },
    HydroTokenType::StringLiteral => todo!(),
    HydroTokenType::CharLiteral => todo!(),
    HydroTokenType::BooleanLiteral => todo!(),
    HydroTokenType::NumberLiteral => todo!(),
    _ => panic!("unexpected primary token"),
}
}

fn typecheck_operation_or_primary(operation_primary: &mut OperationOrPrimary, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  match operation_primary {
    OperationOrPrimary::Operation(operation) => typecheck_operation(operation, symbol_table, errors),
    OperationOrPrimary::Primary(primary) => typecheck_primary(primary, symbol_table, errors),
  }
}

fn typecheck_assignment(assignment: &mut Assignment, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  let expr_type = typecheck_operation_or_primary(&mut assignment.operation, symbol_table, errors);
  if symbol_table.add_variable(assignment.identifier.lexeme.clone(), expr_type) {
    return expr_type
  } else {
    // TODO make it so we check if the variable has a type
    errors.push(OceanError::SemanticError(
      Severity::Error,
      (assignment.identifier.start, assignment.identifier.end), 
      "Cannot assign to variables that have already been declared".to_string()));
    return 0
  }
}

fn typecheck_if(if_statement: &mut If, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  let condition_type = typecheck_operation_or_primary(&mut if_statement.condition, symbol_table, errors);

  if !symbol_table.is_boolean(condition_type) {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      (0, 0), // TODO
      "Condition must evaluate to a boolean".to_string()
    ));
  }

  for inst in &mut if_statement.true_body {
    typecheck_instruction(inst, symbol_table, errors);
  }

  for inst in &mut if_statement.else_body {
    typecheck_instruction(inst, symbol_table, errors);
  }

  condition_type
}

fn typecheck_loop(loop_statement: &mut Loop, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  let condition_type = typecheck_operation_or_primary(&mut loop_statement.condition, symbol_table, errors);

  if !symbol_table.is_boolean(condition_type) {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      (0, 0), // TODO
      "Condition must evaluate to a boolean".to_string()
    ));
  }

  for inst in &mut loop_statement.body {
    typecheck_instruction(inst, symbol_table, errors);
  }

  condition_type
}

fn typecheck_function(function: &mut Function, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  0
}

fn typecheck_typedefinition(type_def: &mut TypeDefinition, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  0
}

fn typecheck_return(ret: &mut Return, symbol_table: &mut HydroSymbolTable, errors: &mut Vec<OceanError>) -> u64 {
  0
}
