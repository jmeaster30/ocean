use std::cmp::Ordering;

use crate::compiler::errors::*;
use crate::compiler::parser::ast::*;
use crate::compiler::{errors::OceanError, parser::span::Spanned};

use super::symboltable::{
  ArraySymbol, OceanType, Symbol, SymbolTable, SymbolTableVarEntry, TupleSymbol,
};

pub fn type_checker(program: &Program) -> (SymbolTable, Vec<OceanError>) {
  let mut symbol_table = SymbolTable::new();
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
    Statement::Error(x) => todo!(),
    Statement::Macro(x) => todo!(),
    Statement::Continue(x) => todo!(),
    Statement::Break(x) => todo!(),
    Statement::Return(x) => todo!(),
    Statement::PackDec(x) => todo!(),
    Statement::UnionDec(x) => todo!(),
    Statement::VarDec(var_dec) => type_checker_var_dec(var_dec, symbol_table, errors),
    Statement::Cast(x) => todo!(),
    Statement::Match(x) => todo!(),
    Statement::Use(x) => todo!(),
    Statement::If(x) => todo!(),
    Statement::ForLoop(x) => todo!(),
    Statement::WhileLoop(x) => todo!(),
    Statement::InfiniteLoop(x) => todo!(),
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
    Expression::Binary(_) => todo!(),
    Expression::Prefix(_) => todo!(),
    Expression::Postfix(_) => todo!(),
    Expression::Member(_) => todo!(),
    Expression::ArrayAccess(_) => todo!(),
    Expression::Cast(_) => todo!(),
    Expression::Literal(x) => {
      let t = get_literal_type(x, symbol_table, errors);
      println!("{:?}", t);
      t
    }
    Expression::Var(_) => todo!(),
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
        val if val.len() <= 3 && val.parse::<u64>().unwrap() < 256 => {
          Some(Symbol::Base(OceanType::U8))
        }
        val if val.len() <= 5 && val.parse::<u64>().unwrap() < 65536 => {
          Some(Symbol::Base(OceanType::U16))
        }
        val if val.len() <= 10 && val.parse::<u64>().unwrap() < 4294967296 => {
          Some(Symbol::Base(OceanType::U32))
        }
        val if val.len() < 19 => Some(Symbol::Base(OceanType::U64)),
        val if val.len() == 20 && val.cmp("18446744073709551616") == Ordering::Less => {
          Some(Symbol::Base(OceanType::U64))
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
        Box::new(Symbol::Base(OceanType::U64)),
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
          // if arg_symbols[i] doesn't match the x.parameter[i] symbol then
          //   give error
          // else
          //   yay!!!!
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
