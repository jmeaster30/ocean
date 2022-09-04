#![allow(unused_variables, dead_code)]

use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  get_index_type, get_iterator_type, resolve_binary_operator, resolve_postfix_operator,
  resolve_prefix_operator, ArraySymbol, OceanType, Symbol, SymbolTable, SymbolTableVarEntry,
  TupleSymbol,
};

pub fn type_checker(program: &Program) -> (SymbolTable, Vec<OceanError>) {
  let mut symbol_table = SymbolTable::hard_scope(None);
  let mut errors = Vec::new();

  for statement in &program.statements {
    type_checker_stmt(statement, &mut symbol_table, &mut errors)
  }

  (symbol_table, errors)
}

pub fn type_checker_stmt(
  statement: &Statement,
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
    Statement::If(x) => {
      let condition_sym = get_expression_type(&x.condition, symbol_table, errors);
      if let Some(condition) = condition_sym {
        let is_bool = symbol_table.match_types(&condition, &Symbol::Base(OceanType::Bool));
        if is_bool.is_none() {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            x.condition.get_span(),
            format!(
              "Condition of an if statement must evaluate to a boolean but got {:?}",
              condition
            ),
          ));
        }
      }

      // TODO I think because we clone the parent scope we won't be able to modify the parent scope
      let soft_symbol_table = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
      for stmt in &x.true_body {
        type_checker_stmt(stmt, symbol_table, errors);
      }

      match x.else_token {
        Some(_) => {
          let soft_symbol_table = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
          for stmt in &x.else_body {
            type_checker_stmt(stmt, symbol_table, errors);
          }
        }
        None => {}
      }
    }
    Statement::ForLoop(x) => {
      let mut soft_symbol_table = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
      let iterable_sym = get_expression_type(&x.iterable, symbol_table, errors);
      match iterable_sym {
        Some(iterable) => {
          let iterator_type_sym = get_iterator_type(&iterable);
          let index_type_sym = get_index_type(&iterable);
          match (iterator_type_sym, index_type_sym) {
            (Some(iterator_type), Some(index_type)) => {
              let matched_type =
                symbol_table.match_types(&index_type, &Symbol::Base(OceanType::Unsigned(64)));
              if matched_type.is_some() {
                soft_symbol_table.add_var(
                  x.iterator.lexeme.clone(),
                  (x.iterator.start, x.iterator.end),
                  iterator_type,
                );
              } else {
                let mut tuple_sym = TupleSymbol::new();
                tuple_sym.add_named("key".to_string(), index_type);
                tuple_sym.add_named("value".to_string(), iterator_type);
                soft_symbol_table.add_var(
                  x.iterator.lexeme.clone(),
                  (x.iterator.start, x.iterator.end),
                  Symbol::Tuple(tuple_sym),
                );
              }
            }
            _ => {
              errors.push(OceanError::SemanticError(
                Severity::Error,
                x.iterable.get_span(),
                format!("This expression of type {:?} is not iterable", iterable),
              ));
              soft_symbol_table.add_var(
                x.iterator.lexeme.clone(),
                (x.iterator.start, x.iterator.end),
                Symbol::Unknown,
              );
            }
          }
        }
        None => soft_symbol_table.add_var(
          x.iterator.lexeme.clone(),
          (x.iterator.start, x.iterator.end),
          Symbol::Unknown,
        ),
      };

      println!("{:?}", soft_symbol_table);

      for stmt in &x.loop_body {
        type_checker_stmt(stmt, &mut soft_symbol_table, errors);
      }
    }
    Statement::WhileLoop(x) => {
      let condition_sym = get_expression_type(&x.condition, symbol_table, errors);
      match condition_sym {
        Some(condition) => {
          let is_bool = symbol_table.match_types(&condition, &Symbol::Base(OceanType::Bool));
          if is_bool.is_none() {
            errors.push(OceanError::SemanticError(
              Severity::Error,
              x.condition.get_span(),
              format!(
                "Condition of while loop must evaluate to a boolean but got {:?}",
                condition
              ),
            ))
          }
        }
        None => {}
      }

      let soft_symbol_table = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
      for stmt in &x.loop_body {
        type_checker_stmt(stmt, symbol_table, errors);
      }
    }
    Statement::InfiniteLoop(x) => {
      let mut soft_symbol_table = SymbolTable::soft_scope(Some(Box::new(symbol_table.clone())));
      for stmt in &x.loop_body {
        type_checker_stmt(stmt, &mut soft_symbol_table, errors);
      }
    }
    Statement::Expression(x) => {
      get_expression_type(&x.expression, symbol_table, errors);
    }
  }
}

pub fn get_var_type(
  var: &Var,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  let name = match var {
    Var::Typed(typed) => typed.var.id.lexeme.clone(),
    Var::Untyped(untyped) => untyped.id.lexeme.clone(),
  };
  let found = symbol_table.find_variable(&name);
  match found {
    Some(symbol_table_var_entry) => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        var.get_span(),
        format!(
          "Redeclaration. This variable was declared previously on line '{}' and column '{}'",
          symbol_table_var_entry.span.0, symbol_table_var_entry.span.1
        ),
      ));
      None
    }
    None => match var {
      Var::Typed(typed) => Some(symbol_table.resolve_type_ast(typed.var_type.as_ref())),
      Var::Untyped(untyped) => Some(Symbol::Unknown),
    },
  }
}

pub fn get_untyped_var_type(
  var: &UntypedVar,
  symbol_table: &SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  let name = var.id.lexeme.clone();
  let found = symbol_table.find_variable(&name);
  match found {
    Some(symbol_table_var_entry) => Some(symbol_table_var_entry.symbol.clone()),
    None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        var.get_span(),
        format!("Variable '{}' not declared", name),
      ));
      None
    }
  }
}

pub fn type_checker_var_dec(
  var_dec: &VarDecStatement,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) {
  let var_type = get_var_type(&var_dec.var, symbol_table, errors);
  let expr_type = match &var_dec.expression {
    Some(expr) => get_expression_type(expr, symbol_table, errors),
    None => Some(Symbol::Unknown),
  };

  match (var_type, expr_type) {
    (Some(var_sym), Some(expr_sym)) => {
      // must match
    }
    (Some(var_sym), None) => {
      // error in expresstion BUT
      // add to sym table
    }
    (None, Some(_)) | (None, None) => { /* There were errors so no need to do anything */ }
  }
}

pub fn get_expression_type(
  expr: &Expression,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  match expr {
    Expression::Binary(binary) => {
      let lhs_expr_sym = get_expression_type(binary.lhs.as_ref(), symbol_table, errors);
      let rhs_expr_sym = get_expression_type(binary.rhs.as_ref(), symbol_table, errors);
      match (lhs_expr_sym, rhs_expr_sym) {
        (Some(lhs_expr), Some(rhs_expr)) => {
          let result = resolve_binary_operator(
            binary.operator.lexeme.to_string(),
            &lhs_expr,
            &rhs_expr,
            &symbol_table,
          );
          match result {
            Some(x) => {
              println!("binary result: {:?}", x);
              Some(x)
            }
            None => {
              errors.push(OceanError::SemanticError(
                Severity::Error,
                binary.get_span(),
                format!(
                  "Unknown operator {} for lhs type of {:?} and rhs type of {:?}",
                  binary.operator.lexeme, lhs_expr, rhs_expr
                ),
              ));
              None
            }
          }
        }
        _ => None,
      }
    }
    Expression::Prefix(prefix) => {
      let pre_expr_sym = get_expression_type(prefix.rhs.as_ref(), symbol_table, errors);
      match pre_expr_sym {
        Some(pre_expr) => {
          let result =
            resolve_prefix_operator(prefix.operator.lexeme.to_string(), &pre_expr, &symbol_table);
          match result {
            Some(x) => {
              println!("prefix result: {:?}", x);
              Some(x)
            }
            None => {
              errors.push(OceanError::SemanticError(
                Severity::Error,
                prefix.get_span(),
                format!(
                  "Unknown operator {} for expression of type {:?}",
                  prefix.operator.lexeme, pre_expr
                ),
              ));
              None
            }
          }
        }
        None => None,
      }
    }
    Expression::Postfix(postfix) => {
      let post_expr_sym = get_expression_type(postfix.lhs.as_ref(), symbol_table, errors);
      match post_expr_sym {
        Some(post_expr) => {
          let result = resolve_postfix_operator(
            postfix.operator.lexeme.to_string(),
            &post_expr,
            &symbol_table,
          );
          match result {
            Some(x) => {
              println!("postfix result: {:?}", x);
              Some(x)
            }
            None => {
              errors.push(OceanError::SemanticError(
                Severity::Error,
                postfix.get_span(),
                format!(
                  "Unknown operator {} for expression of type {:?}",
                  postfix.operator.lexeme, post_expr
                ),
              ));
              None
            }
          }
        }
        None => None,
      }
    }
    Expression::Member(_) => {
      todo!("should I think about doing the functions as member variables at this point?")
    }
    Expression::ArrayAccess(access) => {
      let target_sym = get_expression_type(access.lhs.as_ref(), symbol_table, errors);
      let index_sym = get_expression_type(access.expr.as_ref(), symbol_table, errors);
      match (target_sym, index_sym) {
        (Some(target), Some(index)) => match get_iterator_type(&target) {
          Some(iterator) => match get_index_type(&target) {
            Some(target_index) => {
              let matched_index = symbol_table.match_types(&index, &target_index);
              if matched_index.is_some() {
                println!("iterator: {:?}", iterator);
                Some(iterator)
              } else {
                errors.push(OceanError::SemanticError(
                  Severity::Error,
                  access.expr.as_ref().get_span(),
                  format!(
                    "Cannot index the {:?} type using an {:?} it must be indexed by an {:?}",
                    target, index, target_index
                  ),
                ));
                None
              }
            }
            None => {
              errors.push(OceanError::SemanticError(Severity::Error, access.expr.as_ref().get_span(), "Cannot index the [add type] type using an [add index] it must be index by an '[add correct index]'".to_string()));
              None
            }
          },
          None => {
            errors.push(OceanError::SemanticError(
              Severity::Error,
              access.lhs.as_ref().get_span(),
              "Target expression is not iterable".to_string(),
            ));
            None
          }
        },
        _ => None,
      }
    }
    Expression::Cast(_) => todo!(),
    Expression::Literal(x) => {
      let t = get_literal_type(x, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::Var(var) => {
      let t = get_untyped_var_type(var, symbol_table, errors);
      println!("{:?}", t);
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
  literal: &Literal,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  match literal {
    Literal::Boolean(_) => Some(Symbol::Base(OceanType::Bool)),
    Literal::Number(x) => {
      // these numbers are always positive
      let val = x.lexeme.as_str();
      match val {
        val if val.contains(".") => {
          // TODO make this way better
          Some(Symbol::Base(OceanType::Float(64)))
        }
        val if val.len() <= 3 && val.parse::<u64>().unwrap() < 256 => {
          Some(Symbol::Base(OceanType::Unsigned(8)))
        }
        val if val.len() <= 5 && val.parse::<u64>().unwrap() < 65536 => {
          Some(Symbol::Base(OceanType::Unsigned(16)))
        }
        val if val.len() <= 10 && val.parse::<u64>().unwrap() < 4294967296 => {
          Some(Symbol::Base(OceanType::Unsigned(32)))
        }
        val if val.len() < 19 => Some(Symbol::Base(OceanType::Unsigned(64))),
        val if val.len() == 20 && val.cmp("18446744073709551616") == Ordering::Less => {
          Some(Symbol::Base(OceanType::Unsigned(64)))
        }
        _ => {
          errors.push(OceanError::SemanticError(
            Severity::Error,
            (x.start, x.end),
            "This number is too big and cannot fit into an unsigned 64 bit number.".to_string(),
          ));
          None
        }
      }
    }
    Literal::String(x) => {
      if x.lexeme.len() == 3 {
        Some(Symbol::Base(OceanType::Char))
      } else {
        Some(Symbol::Base(OceanType::String))
      }
    }
    Literal::Array(x) => {
      let mut storage_symbol = Symbol::Unknown;
      for arg in &x.args {
        let arg_sym = get_expression_type(arg.as_ref(), symbol_table, errors);
        match arg_sym {
          Some(sym) => match symbol_table.match_types(&storage_symbol, &sym) {
            Some(result_symbol) => storage_symbol = result_symbol,
            None => errors.push(OceanError::SemanticError(
              Severity::Error,
              arg.get_span(),
              "Unexpected type for array entry. TODO ADD TYPE REPORT HERE".to_string(),
            )),
          },
          None => { /* idk?? */ }
        }
      }
      Some(Symbol::Array(ArraySymbol::new(
        Box::new(storage_symbol),
        Box::new(Symbol::Base(OceanType::Unsigned(64))),
      )))
    }
    Literal::Tuple(x) => {
      let mut symbol = TupleSymbol::new();
      for content in &x.contents {
        let exp_sym = get_expression_type(&content.expression, symbol_table, errors);
        match (&content.name, exp_sym) {
          (Some(name_val), Some(exp)) => {
            symbol.add_named(name_val.lexeme.clone(), exp);
          }
          (None, Some(exp)) => {
            symbol.add_unnamed(exp);
          }
          _ => { /* uhoh :( */ }
        }
      }
      Some(Symbol::Tuple(symbol))
    }
    Literal::Function(x) => todo!(),
  }
}

pub fn get_function_call_type(
  call: &FunctionCall,
  symbol_table: &mut SymbolTable,
  errors: &mut Vec<OceanError>,
) -> Option<Symbol> {
  let call_target = get_expression_type(call.target.as_ref(), symbol_table, errors);
  match call_target {
    Some(Symbol::Function(x)) => {
      if x.parameters.len() == call.arguments.len() {
        let mut arg_symbols = Vec::new();
        for arg in &call.arguments {
          arg_symbols.push(get_expression_type(arg.as_ref(), symbol_table, errors));
        }
        for i in 0..call.arguments.len() {
          let arg_type = get_expression_type(call.arguments[i].as_ref(), symbol_table, errors);
          match arg_type {
            Some(arg) => {
              let matched_param_type = symbol_table.match_types(&arg, &x.parameters[i].1);
              match matched_param_type {
                Some(_) => {}
                None => {
                  errors.push(OceanError::SemanticError(
                    Severity::Error,
                    call.arguments[i].get_span(),
                    format!(
                      "Mismatch type for function call argument '{}'. Expected {:?} but got {:?}",
                      x.parameters[i].0, x.parameters[i].1, arg
                    ),
                  ));
                }
              }
            }
            None => {}
          }
        }
      } else {
        errors.push(OceanError::SemanticError(
          Severity::Error,
          call.get_span(),
          format!(
            "Unexpected number of arguments to function call... Expected: '{}' Found: '{}'",
            x.parameters.len(),
            call.arguments.len()
          ),
        ));
      }

      if x.returns.len() == 0 {
        Some(Symbol::Base(OceanType::Void))
      } else if x.returns.len() == 1 {
        Some(x.returns[0].1.clone())
      } else {
        let mut return_symbol = TupleSymbol::new();
        for ret_type in x.returns {
          return_symbol.add_named(ret_type.0, ret_type.1);
        }
        Some(Symbol::Tuple(return_symbol))
      }
    }
    Some(_) | None => {
      errors.push(OceanError::SemanticError(
        Severity::Error,
        call.target.get_span(),
        "Call target not callable :(".to_string(),
      ));
      None
    }
  }
}
