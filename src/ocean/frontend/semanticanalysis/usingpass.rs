use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use itertools::Either;
use ocean_macros::New;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::compilationunit::CompilationUnit;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::ocean::Ocean;
use crate::util::errors::{Error, ErrorMetadata, Severity};
use crate::util::span::Spanned;

#[derive(Clone, Debug, New)]
pub struct UsingPassContext {
  #[default(HashMap::new())]
  pub path_to_symbol_table: HashMap<String, Rc<RefCell<CompilationUnit>>>,
  #[default(Vec::new())]
  pub current_dependency_chain: Vec<String>,
}

impl UsingPassContext {
  pub fn get_compilation_unit(&self, path: String) -> Option<Rc<RefCell<CompilationUnit>>> {
    match self.path_to_symbol_table.get(path.as_str()) {
      Some(x) => Some(x.clone()),
      None => None
    }
  }

  pub fn start_using(&mut self, path: String, using_span: (usize, usize)) -> Result<(), Error> {
    for (conflict_idx, p) in self.current_dependency_chain.iter().enumerate() {
      if *p == path {
        let mut metadata = ErrorMetadata::new();
        metadata.add_suggestion("Dependency cycle".to_string());
        for (idx, conflict_path) in self.current_dependency_chain.iter().enumerate().rev() {
          if conflict_idx == idx { break }
          metadata.add_suggestion(conflict_path.clone());
        }

        return Err(Error::new_with_metadata(
          Severity::Error,
          using_span,
          "Cyclic dependencies are not allowed".to_string(),
          metadata
        ))
      }
    }

    self.current_dependency_chain.push(path.clone());
    Ok(())
  }

  pub fn stop_using(&mut self) {
    self.current_dependency_chain.pop();
  }
}

pub trait UsingPass {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error>;
}

impl UsingPass for Program {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = Vec::new();

    self.table = Some(table.clone());

    for statement in &mut self.statements {
      join_errors(&mut results, &mut statement.analyze_using(table.clone(), context.clone()))
    }
    
    results
  }
}

impl UsingPass for StatementNode {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    if let Some(stmt) = &mut self.statement {
      stmt.analyze_using(table, context)
    } else {
      Vec::new()
    }
  }
}

impl UsingPass for Statement {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    match self {
      Statement::WhileLoop(x) => x.analyze_using(table, context),
      Statement::ForLoop(x) => x.analyze_using(table, context),
      Statement::Loop(x) => x.analyze_using(table, context),
      Statement::Branch(x) => x.analyze_using(table, context),
      Statement::Match(x) => x.analyze_using(table, context),
      Statement::Assignment(x) => x.analyze_using(table, context),
      Statement::Function(x) => x.analyze_using(table, context),
      Statement::Pack(_) => Vec::new(),
      Statement::Union(_) => Vec::new(),
      Statement::Interface(x) => x.analyze_using(table, context),
      Statement::Return(_) => Vec::new(),
      Statement::Break(_) => Vec::new(),
      Statement::Continue(_) => Vec::new(),
      Statement::Using(x) => x.analyze_using(table, context),
      Statement::Expression(x) => x.analyze_using(table, context),
    }
  }
}

impl UsingPass for ExpressionStatement {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    self.expression_node.analyze_using(table, context)
  }
}

impl UsingPass for CompoundStatement {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = Vec::new();

    for statement in &mut self.body {
      join_errors(&mut results, &mut statement.analyze_using(table.clone(), context.clone()))
    }

    results
  }
}

impl UsingPass for WhileLoop {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = self.condition.analyze_using(table.clone(), context.clone());

    let new_scope = SymbolTable::soft_scope(table.clone());
    self.table = Some(new_scope.clone());

    results.append(&mut self.body.analyze_using(new_scope, context));
    results
  }
}

impl UsingPass for ForLoop {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = self.iterable.analyze_using(table.clone(), context.clone());
    join_errors(&mut results, &mut self.iterator.analyze_using(table.clone(), context.clone()));

    let new_scope = SymbolTable::soft_scope(table.clone());
    self.table = Some(new_scope.clone());

    join_errors(&mut results, &mut self.body.analyze_using(new_scope, context));
    results
  }
}

impl UsingPass for Loop {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let new_scope = SymbolTable::soft_scope(table.clone());
    self.table = Some(new_scope.clone());
    self.body.analyze_using(new_scope, context)
  }
}

impl UsingPass for Branch {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = self.condition.analyze_using(table.clone(), context.clone());

    let new_scope = SymbolTable::soft_scope(table.clone());
    self.table = Some(new_scope.clone());

    join_errors(&mut results, &mut self.body.analyze_using(new_scope, context.clone()));

    if let Some(else_branch) = &mut self.else_branch {
      let else_scope = SymbolTable::soft_scope(table.clone());
      join_errors(&mut results, &mut else_branch.analyze_using(else_scope, context));
    }

    results
  }
}

impl UsingPass for ElseBranch {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    self.table = Some(table.clone());

    match &mut self.body {
      Either::Left(compound) => compound.analyze_using(table.clone(), context.clone()),
      Either::Right(branch) => branch.analyze_using(table.clone(), context.clone()),
    }
  }
}

impl UsingPass for Match {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    todo!()
  }
}

impl UsingPass for Assignment {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = match &mut self.left_expression {
      Either::Left(_) => Vec::new(),
      Either::Right(expr) => expr.analyze_using(table.clone(), context.clone()),
    };

    join_errors(&mut results, &mut self.right_expression.analyze_using(table, context));
    results
  }
}

impl UsingPass for Function {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let new_scope = SymbolTable::hard_scope(Some(table));
    let mut results = Vec::new();
    for res in &mut self.results {
      join_errors(&mut results, &mut res.analyze_using(new_scope.clone(), context.clone()));
    }

    if let Some(compound) = &mut self.compound_statement {
      join_errors(&mut results, &mut compound.analyze_using(new_scope.clone(), context.clone()))
    }
    results
  }
}

impl UsingPass for FunctionReturn {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    if let Some(expr) = &mut self.expression {
      expr.analyze_using(table, context)
    } else {
      Vec::new()
    }
  }
}

impl UsingPass for Interface {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = Vec::new();

    for entry in &mut self.entries {
      join_errors(&mut results, &mut entry.function.analyze_using(table.clone(), context.clone()));
    }

    results
  }
}

impl UsingPass for Using {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    // start the using thingy, if we have the symbol table already then we can just pull it from there
    // if success then we want to call the compiler again but with the current using pass context
    // hmmm compiler may have to return an ast and we might have to attach it somewhere here
    // then stop the using thingy so we can remove it from the dependency chain and move on
    let file_path = self.get_file_path();
    match context.borrow_mut().start_using(file_path.clone(), self.get_span()) {
      Ok(_) => { },
      Err(err) => return vec![err],
    };

    let compilation_unit = Rc::new(RefCell::new(Ocean::compile_using(file_path.as_str(), context.clone())));
    match &compilation_unit.borrow().program {
      Some(program) => match &program.table {
        Some(using_table) => table.borrow_mut().add_using_table(using_table.clone()),
        None => return vec![Error::new(Severity::Warning, self.get_span(), "There was an issue with this import".to_string())]
      }
      None => return vec![Error::new(Severity::Warning, self.get_span(), "There was an issue with this import".to_string())]
    }

    context.borrow_mut().path_to_symbol_table.insert(compilation_unit.borrow().filepath.clone(), compilation_unit.clone());
    context.borrow_mut().stop_using();
    Vec::new()
  }
}

impl UsingPass for ExpressionNode {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    let mut results = Vec::new();

    for token in &mut self.tokens {
      match token {
        Either::Left(_) => {}
        Either::Right(expr) => join_errors(&mut results, &mut expr.analyze_using(table.clone(), context.clone())),
      }
    }

    results
  }
}

impl UsingPass for AstNodeExpression {
  fn analyze_using(&mut self, table: Rc<RefCell<SymbolTable>>, context: Rc<RefCell<UsingPassContext>>) -> Vec<Error> {
    match self {
      AstNodeExpression::Match(x) => x.analyze_using(table, context),
      AstNodeExpression::Loop(x) => x.analyze_using(table, context),
      AstNodeExpression::ForLoop(x) => x.analyze_using(table, context),
      AstNodeExpression::WhileLoop(x) => x.analyze_using(table, context),
      AstNodeExpression::Branch(x) => x.analyze_using(table, context),
      AstNodeExpression::Function(x) => x.analyze_using(table, context),
    }
  }
}

fn join_errors(start: &mut Vec<Error>, new_errors: &mut Vec<Error>) {
  start.append(new_errors)
}