use itertools::Either;
use uuid::Uuid;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::ast::Statement;
use crate::ocean::frontend::symboltable::{AnalyzerContext, SymbolTable};
use crate::util::errors::{Error, Severity};
use crate::util::span::Spanned;

pub trait SemanticallyAnalyzable {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>);
}

impl SemanticallyAnalyzable for Program {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let mut results = Vec::new();

    for statement in self.statements.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Using(_) => true,
        _ => false })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.statements.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Pack(_) | Statement::Union(_) => true,
        _ => false
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.statements.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Function(_) => true,
        _ => false
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.statements.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Using(_) | Statement::Pack(_) | Statement::Union(_) | Statement::Function(_) => false,
        _ => true
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }
    (None, results)
  }
}

impl SemanticallyAnalyzable for StatementNode {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    match &self.statement {
      Some(x) => x.analyze(table, context),
      None => (None, Vec::new()),
    }
  }
}

impl SemanticallyAnalyzable for Statement {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    match self {
      Statement::WhileLoop(x) => x.analyze(table, context),
      Statement::ForLoop(x) => x.analyze(table, context),
      Statement::Loop(x) => x.analyze(table, context),
      Statement::Branch(x) => x.analyze(table, context),
      Statement::Match(x) => x.analyze(table, context),
      Statement::Assignment(x) => x.analyze(table, context),
      Statement::Function(x) => x.analyze(table, context),
      Statement::Pack(x) => x.analyze(table, context),
      Statement::Union(x) => x.analyze(table, context),
      Statement::Interface(x) => x.analyze(table, context),
      Statement::Return(x) => x.analyze(table, context),
      Statement::Break(x) => x.analyze(table, context),
      Statement::Continue(x) => x.analyze(table, context),
      Statement::Using(x) => x.analyze(table, context),
      Statement::Expression(x) => x.analyze(table, context),
    }
  }
}

impl SemanticallyAnalyzable for ExpressionStatement {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    self.expression_node.analyze(table, context)
  }
}

impl SemanticallyAnalyzable for CompoundStatement {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let mut results = Vec::new();

    for statement in self.body.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Using(_) => true,
        _ => false })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.body.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Pack(_) | Statement::Union(_) => true,
        _ => false
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.body.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Function(_) => true,
        _ => false
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }

    for statement in self.body.iter()
      .filter(|x| x.statement.as_ref().is_some_and(|y| match y {
        Statement::Using(_) | Statement::Pack(_) | Statement::Union(_) | Statement::Function(_) => false,
        _ => true
      })) {
      join_errors(&mut results, &mut statement.analyze(table, context))
    }
    (None, results)
  }
}

impl SemanticallyAnalyzable for WhileLoop {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let (_condition_type, mut results) = self.condition.analyze(table, context);

    let mut new_table = SymbolTable::soft_scope(Some(table));
    join_errors(&mut results, &mut self.body.analyze(&mut new_table, context.create_in_loop()));

    (None, results)
  }
}

impl SemanticallyAnalyzable for ForLoop {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let mut new_table = SymbolTable::soft_scope(Some(table));

    let (iterable_type, mut results) = self.iterable.analyze(&mut new_table, context);
    join_errors(&mut results, &mut self.iterator.analyze(&mut new_table, context.create_assignment_target(iterable_type)));
    join_errors(&mut results, &mut self.body.analyze(&mut new_table, context.create_in_loop()));

    (None, results)
  }
}

impl SemanticallyAnalyzable for Loop {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let mut new_table = SymbolTable::soft_scope(Some(table));
    self.body.analyze(&mut new_table, context.create_in_loop())
  }
}

impl SemanticallyAnalyzable for Branch {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let (_condition_type, mut results) = self.condition.analyze(table, context);

    let mut true_table = SymbolTable::soft_scope(Some(&mut table));
    join_errors(&mut results, &mut self.body.analyze(&mut true_table, context));

    if let Some(else_branch) = &self.else_branch {
      join_errors(&mut results, &mut else_branch.analyze(table, context));
    }

    (None, results)
  }
}

impl SemanticallyAnalyzable for ElseBranch {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    match &self.body {
      Either::Left(compound) => {
        let mut new_table = SymbolTable::soft_scope(Some(table));
        compound.analyze(&mut new_table, context)
      }
      Either::Right(branch) => branch.analyze(table, context)
    }
  }
}

impl SemanticallyAnalyzable for Match {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Assignment {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let (rhs_type, mut results) = self.right_expression.analyze(table, context);

    let (lhs_type, mut left_results) = match &self.left_expression {
      Either::Left(let_target) => let_target.analyze(table, context.create_assignment_target(rhs_type)),
      Either::Right(expression) => expression.analyze(table, context.create_assignment_target(rhs_type)),
    };

    results.append(&mut left_results);
    (lhs_type, results)
  }
}

impl SemanticallyAnalyzable for Function {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    let mut results = Vec::new();
    let mut param_types = Vec::new();
    let mut return_types = Vec::new();

    let mut bad_signature = false;

    for param in &self.params {
      let (param_type, mut errors) = param.analyze(table, context);
      if let Some(type_id) = param_type {
        param_types.push((param.identifier.identifier.lexeme.clone(), param.identifier.identifier.get_span(), type_id));
      } else {
        bad_signature = true;
      }
      results.append(&mut errors);
    }

    for ret in &self.results {
      let (ret_type, mut errors) = ret.analyze(table, context);
      if let Some(type_id) = ret_type {
        return_types.push((ret.identifier.identifier.lexeme.clone(), ret.identifier.identifier.get_span(), type_id));
      } else {
        bad_signature = true;
      }
      results.append(&mut errors);
    }

    let function_type = if !bad_signature {
      None
      //Some(table.add_function(&self.identifier.lexeme, param_types, return_types))
    } else {
      None
    };

    if let Some(cmpd_stmt) = &self.compound_statement {
      let mut new_scope = SymbolTable::hard_scope(Some(table));
      for (param_name, param_span, param_type) in param_types {
        match new_scope.add_variable(param_name, param_span, param_type) {
          Ok(()) => (),
          Err(error) => results.push(error)
        }
      }
      for (ret_name, ret_span, ret_type) in return_types {
        match new_scope.add_variable(ret_name, ret_span, ret_type) {
          Ok(()) => (),
          Err(error) => results.push(error)
        }
      }

      let (_, mut errors) = cmpd_stmt.analyze(&mut new_scope, AnalyzerContext::default());
      results.append(&mut errors);
    }

    (function_type, results)
  }
}

impl SemanticallyAnalyzable for FunctionParam {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for FunctionReturn {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Pack {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Union {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Interface {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Using {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Return {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    (None, Vec::new()) // ?? is there anything we should check here?
  }
}

impl SemanticallyAnalyzable for Break {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    if !context.in_loop {
      (None, vec![Error::new(Severity::Error, self.break_token.get_span(), "Cannot use a 'break' statement outside of a loop.".to_string())])
    } else {
      (None, Vec::new())
    }
  }
}

impl SemanticallyAnalyzable for Continue {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    if !context.in_loop {
      (None, vec![Error::new(Severity::Error, self.continue_token.get_span(), "Cannot use a 'continue' statement outside of a loop.".to_string())])
    } else {
      (None, Vec::new())
    }
  }
}

impl SemanticallyAnalyzable for LetTarget {
  fn analyze(&self, table: &mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for ExpressionNode {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

impl SemanticallyAnalyzable for Expression {
  fn analyze(&self, table: & mut SymbolTable, context: AnalyzerContext) -> (Option<Uuid>, Vec<Error>) {
    todo!()
  }
}

fn join_errors(start: &mut Vec<Error>, new_errors: &mut (Option<Uuid>, Vec<Error>)) {
  start.append(&mut new_errors.1)
}