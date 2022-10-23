#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::lexer::TokenType;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  get_base_type_id, get_base_type_symbol_from_lexeme, ArraySymbol, AssignableSymbol, AutoSymbol,
  CustomSymbol, FunctionSymbol, OceanType, Symbol, SymbolTable, SymbolTableVarEntry, TupleSymbol,
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
    Statement::PackDec(pack_dec) => type_checker_pack_dec(pack_dec, symbol_table, errors),
    Statement::UnionDec(x) => todo!("Unions are currently not supported sorry :("),
    Statement::VarDec(var_dec) => type_checker_var_dec(var_dec, symbol_table, errors),
    Statement::Cast(cast_stmt) => type_checker_cast_statement(cast_stmt, symbol_table, errors),
    Statement::Match(x) => panic!(),
    Statement::Use(x) => todo!(),
    Statement::If(if_stmt) => type_checker_if(if_stmt, symbol_table, errors),
    Statement::ForLoop(for_stmt) => type_checker_for_loop(for_stmt, symbol_table, errors),
    Statement::WhileLoop(while_stmt) => type_checker_while_loop(while_stmt, symbol_table, errors),
    Statement::InfiniteLoop(loop_stmt) => {
      type_checker_infinite_loop(loop_stmt, symbol_table, errors)
    }
    Statement::Expression(x) => {
      get_expression_type(&mut x.expression, symbol_table, errors);
    }
  }
}

pub fn type_checker_pack_dec(
  pack_dec: &mut PackDecStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  if symbol_table
    .find_type(&pack_dec.name_token.lexeme)
    .is_some()
  {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      pack_dec.get_span(),
      "This type already exists (TODO add a marker here to highlight where it was declared before)"
        .to_string(),
    ));
    return;
  }

  let mut custom_symbol = CustomSymbol::new(pack_dec.name_token.lexeme.clone());

  for pack_mem in &mut pack_dec.pack_declarations {
    if custom_symbol
      .members
      .contains_key(&pack_mem.type_var.var_name.lexeme)
    {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        pack_mem.get_span(),
        "Duplicate member variable name (todo add a marker here to highlight where it was declared before)".to_string()
      ));
      continue;
    }

    let pack_mem_type_id = get_type_symbol(&pack_mem.type_var.var_type, symbol_table, errors);
    if symbol_table.is_function(pack_mem_type_id) {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        pack_mem.get_span(),
        "Member variable of a pack cannot be a function type".to_string(), // TODO rethink this potentially
      ));
      continue;
    }

    // compare to expression
    match &mut pack_mem.expression {
      Some(expr) => {
        let pack_mem_expr_id = get_expression_type(expr, symbol_table, errors);
        match symbol_table.match_types(pack_mem_type_id, pack_mem_expr_id) {
          Some(_) => {}
          None => {
            errors.push(OceanError::SemanticError(
              Severity::Error,
              expr.get_span(),
              format!(
                "Unexpected type ;( Expected '{:?}' but got '{:?}'",
                symbol_table.get_symbol(pack_mem_type_id),
                symbol_table.get_symbol(pack_mem_expr_id)
              ),
            ));
          }
        }
      }
      None => {}
    }

    custom_symbol.add_member(pack_mem.type_var.var_name.lexeme.clone(), pack_mem_type_id);
  }

  let pack_dec_symbol_id = symbol_table.add_symbol(Symbol::Custom(custom_symbol));
  symbol_table.add_type(pack_dec.name_token.lexeme.clone(), pack_dec_symbol_id);
}

pub fn type_checker_cast_statement(
  cast_statement: &mut CastStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let function_type_id = get_expression_type(&mut cast_statement.function, symbol_table, errors);

  let parameters = symbol_table.get_function_params(function_type_id);
  let returns = symbol_table.get_function_returns(function_type_id);

  if parameters.len() != 1 || returns.len() != 1 {
    errors.push(OceanError::SemanticError(
      Severity::Error,
      cast_statement.function.get_span(),
      format!(
        "Expected a function that has 1 parameter and 1 return but got '{:?}'",
        symbol_table.get_symbol(function_type_id)
      ),
    ));
    return;
  }

  symbol_table.add_cast(parameters[0].1, returns[0].1);
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

pub fn get_untyped_var_type(
  var: &mut UntypedVar,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  // look up var from symbol_table and return the type id
  match symbol_table.find_variable(&var.id.lexeme) {
    Some(var_entry) => {
      let base_type_id = var_entry.type_id;
      let assignable = AssignableSymbol::new(base_type_id);
      let result = symbol_table.add_symbol(Symbol::Assignable(assignable));
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
) -> u64 {
  match type_node {
    Type::Auto(auto_type) => {
      let unk_type_id = symbol_table.add_symbol(Symbol::Unknown);
      let auto_symbol = match &auto_type.auto_name {
        Some(auto_name_token) => AutoSymbol::new(Some(auto_name_token.lexeme.clone()), unk_type_id),
        None => AutoSymbol::new(None, unk_type_id),
      };

      // add type info to the symbol table
      let auto_type_id = symbol_table.add_symbol(Symbol::Auto(auto_symbol));
      match &auto_type.auto_name {
        Some(auto_name_token) => {
          symbol_table.add_type(auto_name_token.lexeme.clone(), auto_type_id)
        }
        None => {}
      };

      auto_type_id
    }
    Type::Comp(comp_type) => panic!("comp"),
    Type::Sub(sub_type) => get_type_symbol(sub_type.sub_type.as_ref(), symbol_table, errors),
    Type::Func(func_type) => {
      panic!("func")
    }
    Type::Base(base_type) => match &base_type.base_token.token_type {
      TokenType::Type => get_base_type_id(get_base_type_symbol_from_lexeme(
        &base_type.base_token.lexeme,
      )),
      TokenType::Identifier => match symbol_table.find_type(&base_type.base_token.lexeme) {
        Some(found_type_id) => found_type_id,
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            base_type.get_span(),
            "Unknown type!!!!!!!!!!!!!!!!".to_string(),
          ));
          symbol_table.add_symbol(Symbol::Unknown)
        }
      },
      _ => panic!("unhandled token type in base type :("),
    },
    Type::Lazy(lazy_type) => panic!("lazy"),
    Type::Ref(ref_type) => panic!("ref"),
    Type::Mutable(mutable_type) => panic!("mut"),
    Type::Array(array_type) => {
      let storage_id = get_type_symbol(array_type.base.as_ref(), symbol_table, errors);
      let index_id = match array_type.sub_type.as_ref() {
        Some(sub_type) => get_type_symbol(sub_type, symbol_table, errors),
        None => get_base_type_id(Symbol::Base(OceanType::Unsigned(64))),
      };
      symbol_table.add_symbol(Symbol::Array(ArraySymbol::new(storage_id, index_id)))
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
  let var_type_id = match &var_dec.var_type {
    Some(type_node) => get_type_symbol(type_node, symbol_table, errors),
    None => symbol_table.add_symbol(Symbol::Unknown),
  };

  // add variable to symbol_table
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
      match symbol_table.match_types(expression_type_id, var_type_id) {
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
}

pub fn get_expression_type(
  expr: &mut Expression,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  match expr {
    Expression::Binary(binary) => {
      let lhs_type_id = get_expression_type(&mut binary.lhs, symbol_table, errors);
      let rhs_type_id = get_expression_type(&mut binary.rhs, symbol_table, errors);
      match symbol_table.get_infix_operator_type(
        binary.operator.lexeme.clone(),
        lhs_type_id,
        rhs_type_id,
      ) {
        Some(result_id) => {
          binary.type_id = Some(result_id);
          result_id
        }
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            binary.get_span(),
            format!(
              "The operator '{}' does not work with type {:?} and {:?}",
              binary.operator.lexeme,
              symbol_table.get_symbol(lhs_type_id),
              symbol_table.get_symbol(rhs_type_id)
            ),
          ));
          let unktype = symbol_table.add_symbol(Symbol::Unknown);
          binary.type_id = Some(unktype);
          unktype
        }
      }
    }
    Expression::Prefix(prefix) => {
      let target_type_id = get_expression_type(&mut prefix.rhs, symbol_table, errors);
      match symbol_table.get_prefix_operator_type(prefix.operator.lexeme.clone(), target_type_id) {
        Some(result_id) => {
          prefix.type_id = Some(result_id);
          result_id
        }
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            prefix.get_span(),
            format!(
              "The operator '{}' does not work with type {:?}",
              prefix.operator.lexeme,
              symbol_table.get_symbol(target_type_id)
            ),
          ));
          let unktype = symbol_table.add_symbol(Symbol::Unknown);
          prefix.type_id = Some(unktype);
          unktype
        }
      }
    }
    Expression::Postfix(postfix) => {
      let target_type_id = get_expression_type(&mut postfix.lhs, symbol_table, errors);
      match symbol_table.get_postfix_operator_type(postfix.operator.lexeme.clone(), target_type_id)
      {
        Some(result_id) => {
          postfix.type_id = Some(result_id);
          result_id
        }
        None => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            postfix.get_span(),
            format!(
              "The operator '{}' does not work with type {:?}",
              postfix.operator.lexeme,
              symbol_table.get_symbol(target_type_id)
            ),
          ));
          let unktype = symbol_table.add_symbol(Symbol::Unknown);
          postfix.type_id = Some(unktype);
          unktype
        }
      }
    }
    Expression::Member(member) => {
      let target_type_id = get_expression_type(member.lhs.as_mut(), symbol_table, errors);
      match symbol_table.get_member_type(target_type_id, member.id.lexeme.clone()) {
        Ok(member_type) => {
          member.type_id = Some(member_type);
          member_type
        }
        Err(_) => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            (member.id.start, member.id.end),
            format!(
              "Type {:?} does not contain a member named '{}'",
              symbol_table.get_symbol(target_type_id),
              member.id.lexeme
            ),
          ));
          let unk_type = symbol_table.add_symbol(Symbol::Unknown);
          member.type_id = Some(unk_type);
          unk_type
        }
      }
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
    Expression::Cast(cast_expression) => {
      let from_type_id = get_expression_type(&mut cast_expression.lhs, symbol_table, errors);
      let to_type_id = get_type_symbol(&mut cast_expression.cast_type, symbol_table, errors);
      if !symbol_table.find_cast(from_type_id, to_type_id) {
        match symbol_table.match_types(from_type_id, to_type_id) {
          Some(_) => {}
          None => {
            errors.push(OceanError::SemanticError(
              Severity::Error,
              cast_expression.get_span(),
              format!(
                "A cast from '{:?}' to '{:?}' does not exist :(",
                symbol_table.get_symbol(from_type_id),
                symbol_table.get_symbol(to_type_id)
              ),
            ));
          }
        }
      }

      cast_expression.type_id = Some(to_type_id);
      to_type_id
    }
    Expression::Literal(x) => get_literal_type(x, symbol_table, errors),
    Expression::Var(var) => get_untyped_var_type(var, symbol_table, errors),
    Expression::FunctionCall(x) => get_function_call_type(x, symbol_table, errors),
    Expression::Error(_) => todo!(),
  }
}

pub fn get_literal_type(
  literal: &mut Literal,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
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
      let mut storage_id = 0;
      for arg in &mut array_literal.args {
        let arg_id = get_expression_type(arg.as_mut(), symbol_table, errors);
        if storage_id == 0 {
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
      let mut sub_scope = SymbolTable::hard_scope(Some(Box::new(symbol_table.clone())));
      let mut func_symbol = FunctionSymbol::new();

      // add params to sub scope and func symbol
      for param in &mut function_literal.param_list.params {
        let param_type_id = get_type_symbol(param.type_var.var_type.as_ref(), symbol_table, errors);
        func_symbol.add_parameter(param.type_var.var_name.lexeme.clone(), param_type_id);
        sub_scope.add_var(
          param.type_var.var_name.lexeme.clone(),
          param.type_var.get_span(),
          param_type_id,
        );
      }

      // add returns to sub scope and fumc symbol
      for ret in &mut function_literal.return_list.returns {
        let ret_type_id = get_type_symbol(ret.type_var.var_type.as_ref(), symbol_table, errors);
        // TODO typecheck optional expression
        func_symbol.add_return(ret.type_var.var_name.lexeme.clone(), ret_type_id);
        sub_scope.add_var(
          ret.type_var.var_name.lexeme.clone(),
          ret.type_var.get_span(),
          ret_type_id,
        );
      }

      // add func symbol to both sub_scope and main scope
      let parent_func_id = symbol_table.add_symbol(Symbol::Function(func_symbol.clone()));
      function_literal.type_id = Some(parent_func_id);
      
      match &function_literal.optional_name_token {
        Some(name_token) => {
          symbol_table.add_var(name_token.lexeme.clone(), (name_token.start, name_token.end), parent_func_id);
          let sub_func_id = sub_scope.add_symbol(Symbol::Function(func_symbol));
          sub_scope.add_var(name_token.lexeme.clone(), (name_token.start, name_token.end), sub_func_id);
        }
        None => {}
      };

      for stmt in &mut function_literal.function_body {
        type_checker_stmt(stmt, &mut sub_scope, errors);
      }

      parent_func_id
    }
  }
}

pub fn get_function_call_type(
  call: &mut FunctionCall,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> u64 {
  let call_target_id = get_expression_type(call.target.as_mut(), symbol_table, errors);
  let mut argument_type_ids = Vec::new();
  for arg in &mut call.arguments {
    let arg_type_id = get_expression_type(arg.as_mut(), symbol_table, errors);
    argument_type_ids.push(arg_type_id);
  }

  match symbol_table.check_function_parameter_lengths(call_target_id, argument_type_ids.len()) {
    Ok(_) => {}
    Err((is_function, expected_length)) => {
      if is_function {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          call.get_span(),
          format!(
            "Found {} arguments but expected {}.",
            argument_type_ids.len(),
            expected_length
          ),
        ));
      } else {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          call.target.get_span(),
          format!(
            "{:?} does not evaluate to a callable type :(",
            symbol_table.get_symbol(call_target_id)
          ),
        ));
      }

      let unk_id = symbol_table.add_symbol(Symbol::Unknown);
      call.type_id = Some(unk_id);
      return unk_id;
    }
  }

  match symbol_table.get_function_return_types(call_target_id, &argument_type_ids) {
    Ok(return_symbol_id) => {
      call.type_id = Some(return_symbol_id);
      return_symbol_id
    }
    Err(bad_parameters) => {
      for (arg_index, param_name, param_type_id) in bad_parameters {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          call.arguments[arg_index].get_span(),
          format!(
            "Expected argument type {:?} for parameter '{}' but got {:?}",
            symbol_table.get_symbol(param_type_id),
            param_name,
            symbol_table.get_symbol(argument_type_ids[arg_index])
          ),
        ))
      }

      let unk_id = symbol_table.add_symbol(Symbol::Unknown);
      call.type_id = Some(unk_id);
      unk_id
    }
  }
}
