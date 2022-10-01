#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  get_base_type_id, ArraySymbol, FunctionSymbol, OceanType, Symbol, SymbolTable,
  SymbolTableVarEntry, TupleSymbol,
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
    Statement::If(x) => todo!(),
    Statement::ForLoop(x) => todo!(),
    Statement::WhileLoop(x) => todo!(),
    Statement::InfiniteLoop(x) => todo!(),
    Statement::Expression(x) => {
      println!("expr");
      get_expression_type(&mut x.expression, symbol_table, errors);
    }
  }
}

pub fn get_var_type(
  var: &Var,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  todo!()
}

pub fn get_untyped_var_type(
  var: &UntypedVar,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  todo!()
}

pub fn type_checker_var_dec(
  var_dec: &VarDecStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  todo!()
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
      if !symbol_table.is_iterable(target_sym_id) {
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
      println!("{:?}", t);
      0
    }
    Expression::FunctionCall(x) => {
      let t = get_function_call_type(x, symbol_table, errors);
      println!("{:?}", t);
      0
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
      let mut storage_id = get_base_type_id(Symbol::Unknown);
      for arg in &mut array_literal.args {
        let arg_id = get_expression_type(arg.as_mut(), symbol_table, errors);
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
