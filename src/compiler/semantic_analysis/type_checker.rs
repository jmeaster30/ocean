#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  get_base_type_id, get_base_type_symbol_from_lexeme, ArraySymbol, FunctionSymbol, OceanType,
  Symbol, SymbolTable, SymbolTableVarEntry, TupleSymbol,
};

pub fn type_checker(program: &mut Program) -> (SymbolTable, Vec<OceanError>) {
  let mut symbol_table = SymbolTable::init();

  let mut errors = Vec::new();

  for statement in &mut program.statements {
    type_checker_stmt(statement, &mut symbol_table, &mut errors)
  }

  (symbol_table, errors)
}

pub fn type_checker_stmt(
  statement: &mut Statement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  match statement {
    Statement::Error(x) => { /* I don't think I need to do anything here */ }
    Statement::Macro(x) => todo!(),
    Statement::Continue(x) => {}
    Statement::Break(x) => {}
    Statement::Return(x) => {}
    Statement::PackDec(x) => todo!(),
    Statement::UnionDec(x) => todo!(),
    Statement::VarDec(var_dec) => type_checker_var_dec(var_dec, symbol_table, errors),
    Statement::Cast(x) => todo!(),
    Statement::Match(x) => panic!(),
    Statement::Use(x) => todo!(),
    Statement::If(if_stmt) => type_checker_if(if_stmt, symbol_table, errors),
    Statement::ForLoop(for_stmt) => type_checker_for_loop(for_stmt, symbol_table, errors),
    Statement::WhileLoop(while_stmt) => type_checker_while_loop(while_stmt, symbol_table, errors),
    Statement::InfiniteLoop(loop_stmt) => {
      type_checker_infinite_loop(loop_stmt, symbol_table, errors)
    }
    Statement::Expression(x) => {
      println!("expr");
      get_expression_type(&mut x.expression, symbol_table, errors);
    }
  }
}

pub fn type_checker_for_loop(
  for_stmt: &mut ForLoopStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let for_expression_id = get_expression_type(&mut for_stmt.iterable, symbol_table, errors);
  if !symbol_table.is_indexable(for_expression_id) {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      for_stmt.iterable.get_span(),
      format!(
        "{:?} is not iterable :(",
        symbol_table.get_symbol(for_expression_id)
      ),
    ));
  }

  // create scope
  let mut sub_scope = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
  // add iterator to scope (even if the iterable expression is not iterable)
  let iterator_type_id = match sub_scope.get_iterator_type_from_indexable(for_expression_id) {
    Ok(iterator_id) => iterator_id,
    Err(_) => sub_scope.add_symbol(Symbol::Unknown),
  };
  sub_scope.add_var(
    for_stmt.iterator.lexeme.clone(),
    (for_stmt.iterator.start, for_stmt.iterator.end),
    iterator_type_id,
  );
  // type check body
  for statement in &mut for_stmt.loop_body {
    type_checker_stmt(statement, &mut sub_scope, errors)
  }
}

pub fn type_checker_while_loop(
  while_stmt: &mut WhileStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let condition_id = get_expression_type(&mut while_stmt.condition, symbol_table, errors);
  match symbol_table.match_types(
    condition_id,
    get_base_type_id(Symbol::Base(OceanType::Bool)),
  ) {
    Some(result_type_id) => {}
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        while_stmt.condition.get_span(),
        format!(
          "Loop condition must evaluate to a boolean value. Found {:?}",
          symbol_table.get_symbol(condition_id)
        ),
      ));
    }
  }

  let mut sub_scope = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
  for statement in &mut while_stmt.loop_body {
    type_checker_stmt(statement, &mut sub_scope, errors)
  }
}

pub fn type_checker_infinite_loop(
  loop_stmt: &mut InfiniteLoopStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let mut sub_scope = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
  for statement in &mut loop_stmt.loop_body {
    type_checker_stmt(statement, &mut sub_scope, errors)
  }
}

pub fn type_checker_if(
  if_stmt: &mut IfStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  // check condition
  let condition_id = get_expression_type(&mut if_stmt.condition, symbol_table, errors);
  match symbol_table.match_types(
    condition_id,
    get_base_type_id(Symbol::Base(OceanType::Bool)),
  ) {
    Some(result_type_id) => {}
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        if_stmt.condition.get_span(),
        format!(
          "If condition must evaluate to a boolean value. Found {:?}",
          symbol_table.get_symbol(condition_id)
        ),
      ));
    }
  }

  //check true body
  // TODO fix the symbol table to be a reference so we can potentially mutate symbols properly
  let mut true_sub_scope = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
  for statement in &mut if_stmt.true_body {
    type_checker_stmt(statement, &mut true_sub_scope, errors)
  }

  // check else body
  // TODO fix the symbol table to be a reference so we can potentially mutate symbols properly
  let mut else_sub_scope = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
  for statement in &mut if_stmt.else_body {
    type_checker_stmt(statement, &mut else_sub_scope, errors)
  }
}

pub fn get_var_type(var: &Var, symbol_table: &SymbolTable, errors: &mut Vec<OceanError>) -> i32 {
  todo!()
}

pub fn get_untyped_var_type(
  var: &mut UntypedVar,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> i32 {
  // look up var from symbol_table and return the type id
  match symbol_table.find_variable(&var.id.lexeme) {
    Some(var_entry) => {
      let result = var_entry.type_id;
      var.type_id = Some(result);
      result
    }
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        var.get_span(),
        format!("Variable not defined '{}'", var.id.lexeme),
      ));
      symbol_table.add_symbol(Symbol::Unknown)
    }
  }
}

pub fn get_type_symbol(
  type_node: &Type,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Symbol {
  match type_node {
    Type::Auto(auto_type) => {
      panic!("auto")
    }
    Type::Comp(comp_type) => panic!("comp"),
    Type::Sub(sub_type) => get_type_symbol(sub_type.sub_type.as_ref(), symbol_table, errors),
    Type::Func(func_type) => {
      panic!("func")
    }
    Type::Base(base_type) => get_base_type_symbol_from_lexeme(&base_type.base_token.lexeme),
    Type::Lazy(lazy_type) => panic!("lazy"),
    Type::Ref(ref_type) => {
      panic!("ref")
    }
    Type::Mutable(mutable_type) => panic!("mut"),
    Type::Array(array_type) => {
      let storage_sym = get_type_symbol(array_type.base.as_ref(), symbol_table, errors);
      let storage_id = symbol_table.add_symbol(storage_sym);
      let index_id = match array_type.sub_type.as_ref() {
        Some(sub_type) => {
          let index_sym = get_type_symbol(sub_type, symbol_table, errors);
          symbol_table.add_symbol(index_sym)
        }
        None => get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
      };
      Symbol::Array(ArraySymbol::new(storage_id, index_id))
    }
    Type::VarType(variable_type) => panic!("variadic type"),
  }
}

pub fn type_checker_var_dec(
  var_dec: &mut VarDecStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  // check if we have this variable already
  if let Some(original_declaration) = symbol_table.find_variable_in_scope(&var_dec.var_name.lexeme)
  {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      (var_dec.var_name.start, var_dec.var_name.end),
      format!(
        "Variable already declared at {:?}.",
        original_declaration.span
      ), // TODO show line number and column number here
    ));
    return;
  }

  // get type information
  let var_type_symbol = match &var_dec.var_type {
    Some(type_node) => get_type_symbol(type_node, symbol_table, errors),
    None => Symbol::Unknown,
  };

  // add variable and symbol to symbol_table
  println!("{:#?}", symbol_table);
  println!("next id: {}", symbol_table.get_new_symbol_id());
  let var_type_id = symbol_table.add_symbol(var_type_symbol);
  symbol_table.add_var(
    var_dec.var_name.lexeme.clone(),
    (var_dec.var_name.start, var_dec.var_name.end),
    var_type_id,
  );

  match &mut var_dec.expression {
    Some(expr) => {
      // type check expression
      let expression_type_id = get_expression_type(expr, symbol_table, errors);

      // match expression type to variable type
      // TODO maybe need a different match function or add additional parameters because I want final_type_id to return var_type_id if the types match
      match symbol_table.match_types(var_type_id, expression_type_id) {
        Some(final_type_id) => { /* Shouldn't need to do anything here */ }
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            expr.get_span(),
            format!(
              "Unexpected type in variable declaration expression. Expected {:?} but found {:?}",
              symbol_table.get_symbol(var_type_id),
              symbol_table.get_symbol(expression_type_id)
            ),
          ));
        }
      }
    }
    None => {}
  }

  println!("{:#?}", symbol_table);
}

pub fn get_expression_type(
  expr: &mut Expression,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> i32 {
  match expr {
    Expression::Binary(binary) => todo!(),
    Expression::Prefix(prefix) => todo!(),
    Expression::Postfix(postfix) => todo!(),
    Expression::Member(_) => {
      todo!("should I think about doing the functions as member variables at this point?")
    }
    Expression::ArrayAccess(access) => {
      let target_sym_id = get_expression_type(access.lhs.as_mut(), symbol_table, errors);
      let index_sym_id = get_expression_type(access.expr.as_mut(), symbol_table, errors);
      let mut result_id = get_base_type_id(Symbol::Unknown);
      if !symbol_table.is_indexable(target_sym_id) {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          access.lhs.get_span(),
          format!(
            "{:?} is not iterable",
            symbol_table.get_symbol(target_sym_id)
          ),
        ));
        access.type_id = Some(result_id);
        return result_id;
      }

      match symbol_table.get_storage_type_from_indexable(target_sym_id, index_sym_id) {
        Ok(storage_type_id) => result_id = storage_type_id,
        Err(_) => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            access.get_span(),
            format!(
              "{:?} is not indexable by {:?}",
              symbol_table.get_symbol(target_sym_id),
              symbol_table.get_symbol(index_sym_id)
            ),
          ));
        }
      }

      access.type_id = Some(result_id);
      result_id
    }
    Expression::Cast(_) => todo!(),
    Expression::Literal(x) => {
      println!("literal");
      let t = get_literal_type(x, symbol_table, errors);
      println!("{:?} {:?}", t, symbol_table.get_symbol(t));
      t
    }
    Expression::Var(var) => {
      let t = get_untyped_var_type(var, symbol_table, errors);
      println!("var '{}' {:?}", var.id.lexeme, t);
      t
    }
    Expression::FunctionCall(x) => {
      let t = get_function_call_type(x, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::Error(_) => todo!(),
  }
}

pub fn get_literal_type(
  literal: &mut Literal,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> i32 {
  match literal {
    Literal::Boolean(bool_literal) => {
      let id = get_base_type_id(Symbol::Base(OceanType::Bool));
      bool_literal.type_id = Some(id);
      id
    }
    Literal::Number(num_literal) => {
      let val = num_literal.token.lexeme.as_str();
      let mut type_id = get_base_type_id(Symbol::Unknown);
      match val {
        val if val.contains(".") => {
          // TODO make this way better
          type_id = get_base_type_id(Symbol::Base(OceanType::Float(64)))
        }
        val if val.len() <= 3 && val.parse::<u64>().unwrap() < 256 => {
          type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(8)))
        }
        val if val.len() <= 5 && val.parse::<u64>().unwrap() < 65536 => {
          type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(16)))
        }
        val if val.len() <= 10 && val.parse::<u64>().unwrap() < 4294967296 => {
          type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(32)))
        }
        val if val.len() < 19 => type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
        val if val.len() == 20 && val.cmp("18446744073709551616") == Ordering::Less => {
          type_id = get_base_type_id(Symbol::Base(OceanType::Unsigned(64)))
        }
        _ => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            num_literal.get_span(),
            "This number is too big and cannot fit into an unsigned 64 bit number.".to_string(),
          ));
        }
      };
      num_literal.type_id = Some(type_id);
      type_id
    }
    Literal::String(str_literal) => {
      if str_literal.token.lexeme.len() == 3 {
        let id = get_base_type_id(Symbol::Base(OceanType::Char));
        str_literal.type_id = Some(id);
        id
      } else {
        let id = get_base_type_id(Symbol::Base(OceanType::String));
        str_literal.type_id = Some(id);
        id
      }
    }
    Literal::Array(array_literal) => {
      let mut storage_id = -1;
      for arg in &mut array_literal.args {
        let arg_id = get_expression_type(arg.as_mut(), symbol_table, errors);
        if storage_id == -1 {
          storage_id = arg_id;
          continue;
        }
        match symbol_table.match_types(arg_id, storage_id) {
          Some(x) => storage_id = x,
          None => errors.push(OceanError::SemanticError(
            Severity::Error,
            arg.get_span(),
            format!(
              "Unexpected type for array entry. Found {:?} but expected {:?}",
              symbol_table.get_symbol(arg_id),
              symbol_table.get_symbol(storage_id)
            ),
          )),
        }
      }
      let id = symbol_table.add_symbol(Symbol::Array(ArraySymbol::new(
        storage_id,
        get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
      )));
      array_literal.type_id = Some(id);
      id
    }
    Literal::Tuple(tuple_literal) => {
      let mut tuple_symbol = TupleSymbol::new();
      for member in &mut tuple_literal.contents {
        let member_id = get_expression_type(&mut member.expression, symbol_table, errors);
        match member.name.clone() {
          Some(member_name) => tuple_symbol.add_named(member_name.lexeme, member_id),
          None => tuple_symbol.add_unnamed(member_id),
        }
      }
      let id = symbol_table.add_symbol(Symbol::Tuple(tuple_symbol));
      tuple_literal.type_id = Some(id);
      id
    }
    Literal::Function(function_literal) => {
      //let mut func_symbol = FunctionSymbol::new();
      //for params in &mut function_literal.param_list.params {
      //}
      todo!("functions need to have the type var typechecking to work")
    }
  }
}

pub fn get_function_call_type(
  call: &mut FunctionCall,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> i32 {
  let call_target = get_expression_type(call.target.as_mut(), symbol_table, errors);
  0
}
