use std::cell::RefCell;
use std::rc::Rc;
use itertools::Either;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::util::errors::Error;

impl Program {
  pub fn analyze_object(&mut self) -> Vec<Error> {
    let mut errors = Vec::new();
    for stmt in &mut self.statements {
      let mut errs = stmt.analyze_object(self.table.clone().unwrap());
      errors.append(&mut errs);
    }
    errors
  }
}

impl StatementNode {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    if let Some(stmt) = &mut self.statement {
      stmt.analyze_object(table)
    } else {
      Vec::new()
    }
  }
}

impl Statement {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match self {
      Statement::WhileLoop(x) => x.analyze_object(table),
      Statement::ForLoop(x) => x.analyze_object(table),
      Statement::Loop(x) => x.analyze_object(table),
      Statement::Branch(x) => x.analyze_object(table),
      Statement::Match(x) => x.analyze_object(table),
      Statement::Assignment(x) => x.analyze_object(table),
      Statement::Function(x) => x.analyze_object(table),
      Statement::Pack(x) => x.analyze_object(table),
      Statement::Union(x) => x.analyze_object(table),
      Statement::Interface(x) => x.analyze_object(table),
      Statement::Return(x) => Vec::new(),
      Statement::Break(x) => Vec::new(),
      Statement::Continue(x) => Vec::new(),
      Statement::Using(x) => Vec::new(),
      Statement::Expression(x) => x.analyze_object(table),
    }
  }
}

impl WhileLoop {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.condition.analyze_object(table.clone());
    errors.append(&mut self.body.analyze_object(self.table.clone().unwrap()));
    errors
  }
}

impl ForLoop {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.iterable.analyze_object(table.clone());
    errors.append(&mut self.iterator.analyze_object(self.table.clone().unwrap()));
    errors.append(&mut self.body.analyze_object(self.table.clone().unwrap()));
    errors
  }
}

impl Loop {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.body.analyze_object(self.table.clone().unwrap())
  }
}

impl Branch {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.condition.analyze_object(table.clone());
    errors.append(&mut self.body.analyze_object(self.table.clone().unwrap()));

    if let Some(else_branch) = &mut self.else_branch {
      errors.append(&mut else_branch.analyze_object(table.clone()));
    }

    errors
  }
}

impl ElseBranch {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match &mut self.body {
      Either::Left(compound) => compound.analyze_object(self.table.clone().unwrap()),
      Either::Right(branch) => branch.analyze_object(self.table.clone().unwrap()),
    }
  }
}


impl Match {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    todo!()
  }
}

impl Assignment {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl Function {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl Pack {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl Union {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl Interface {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}


impl ExpressionStatement {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl ExpressionNode {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl CompoundStatement {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

