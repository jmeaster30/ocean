use std::cell::RefCell;
use std::rc::Rc;
use itertools::Either;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::util::errors::Error;
use crate::util::span::Spanned;

impl Program {
  pub fn analyze_object_declaration(&mut self) -> Vec<Error> {
    let mut errors = Vec::new();
    for stmt in &mut self.statements {
      let mut errs = stmt.analyze_object_declaration(self.table.clone().unwrap());
      errors.append(&mut errs);
    }
    errors
  }
}

impl StatementNode {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    if let Some(stmt) = &mut self.statement {
      stmt.analyze_object_declaration(table)
    } else {
      Vec::new()
    }
  }
}

impl Statement {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match self {
      Statement::WhileLoop(x) => x.analyze_object_declaration(table),
      Statement::ForLoop(x) => x.analyze_object_declaration(table),
      Statement::Loop(x) => x.analyze_object_declaration(table),
      Statement::Branch(x) => x.analyze_object_declaration(table),
      Statement::Match(x) => x.analyze_object_declaration(table),
      Statement::Assignment(x) => x.analyze_object_declaration(table),
      Statement::Function(x) => x.analyze_object_declaration(table),
      Statement::Pack(x) => x.analyze_object_declaration(table),
      Statement::Union(x) => x.analyze_object_declaration(table),
      Statement::Interface(x) => x.analyze_object_declaration(table),
      Statement::Return(x) => Vec::new(),
      Statement::Break(x) => Vec::new(),
      Statement::Continue(x) => Vec::new(),
      Statement::Using(x) => Vec::new(),
      Statement::Expression(x) => x.analyze_object_declaration(table),
    }
  }
}

impl WhileLoop {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.condition.analyze_object_declaration(table.clone());
    errors.append(&mut self.body.analyze_object_declaration(self.table.clone().unwrap()));
    errors
  }
}

impl ForLoop {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.iterable.analyze_object_declaration(table.clone());
    errors.append(&mut self.iterator.analyze_object_declaration(self.table.clone().unwrap()));
    errors.append(&mut self.body.analyze_object_declaration(self.table.clone().unwrap()));
    errors
  }
}

impl Loop {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.body.analyze_object_declaration(self.table.clone().unwrap())
  }
}

impl Branch {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = self.condition.analyze_object_declaration(table.clone());
    errors.append(&mut self.body.analyze_object_declaration(self.table.clone().unwrap()));

    if let Some(else_branch) = &mut self.else_branch {
      errors.append(&mut else_branch.analyze_object_declaration(table.clone()));
    }

    errors
  }
}

impl ElseBranch {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match &mut self.body {
      Either::Left(compound) => compound.analyze_object_declaration(self.table.clone().unwrap()),
      Either::Right(branch) => branch.analyze_object_declaration(self.table.clone().unwrap()),
    }
  }
}


impl Match {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    todo!()
  }
}

impl Assignment {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = match &mut self.left_expression {
      Either::Left(_) => Vec::new(),
      Either::Right(expr) => expr.analyze_object_declaration(table.clone()),
    };

    errors.append(&mut self.right_expression.analyze_object_declaration(table.clone()));
    errors
  }
}

impl Function {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let new_table = SymbolTable::hard_scope(Some(table));
    self.table = Some(new_table.clone());
    let mut errors = Vec::new();
    for result in &mut self.results {
      if let Some(result_expression) = &mut result.expression {
        let mut errs = result_expression.analyze_object_declaration(new_table.clone());
        errors.append(&mut errs);
      }
    }

    if let Some(compound) = &mut self.compound_statement {
      let mut errs = compound.analyze_object_declaration(new_table.clone());
      errors.append(&mut errs);
    }

    errors
  }
}

impl Pack {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let pack_name = self.custom_type.get_name();
    table.borrow_mut().add_pack_declaration(&pack_name, self.custom_type.get_span())
  }
}

impl Union {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let union_name = self.custom_type.get_name();
    table.borrow_mut().add_union_declaration(&union_name, self.custom_type.get_span())
  }
}

impl Interface {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let interface_name = self.custom_type.get_name();
    table.borrow_mut().add_interface_declaration(&interface_name, self.custom_type.get_span())
  }
}


impl ExpressionStatement {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.expression_node.analyze_object_declaration(table)
  }
}

impl ExpressionNode {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match &mut self.parsed_expression {
      Some(expr) => expr.analyze_object_declaration(table),
      None => Vec::new()
    }
  }
}

impl CompoundStatement {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = Vec::new();
    for stmt in &mut self.body {
      let mut errs = stmt.analyze_object_declaration(table.clone());
      errors.append(&mut errs);
    }
    errors
  }
}

impl Expression {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match self {
      Expression::String(_) => Vec::new(),
      Expression::ArrayLiteral(a) => a.analyze_object_declaration(table),
      Expression::Number(_) => Vec::new(),
      Expression::Boolean(_) => Vec::new(),
      Expression::InterpolatedString(i) => i.analyze_object_declaration(table),
      Expression::Variable(_) => Vec::new(),
      Expression::Tuple(t) => t.analyze_object_declaration(table),
      Expression::Call(c) => c.analyze_object_declaration(table),
      Expression::ArrayIndex(a) => a.analyze_object_declaration(table),
      Expression::SubExpression(s) => s.analyze_object_declaration(table),
      Expression::Cast(c) => c.analyze_object_declaration(table),
      Expression::PrefixOperation(p) => p.analyze_object_declaration(table),
      Expression::PostfixOperation(p) => p.analyze_object_declaration(table),
      Expression::BinaryOperation(b) => b.analyze_object_declaration(table),
      Expression::AstNode(a) => a.analyze_object_declaration(table),
    }
  }
}

impl AstNodeExpression {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    match self {
      AstNodeExpression::Match(m) => m.analyze_object_declaration(table),
      AstNodeExpression::Loop(l) => l.analyze_object_declaration(table),
      AstNodeExpression::ForLoop(f) => f.analyze_object_declaration(table),
      AstNodeExpression::WhileLoop(w) => w.analyze_object_declaration(table),
      AstNodeExpression::Branch(b) => b.analyze_object_declaration(table),
      AstNodeExpression::Function(f) => f.analyze_object_declaration(table),
    }
  }
}

impl ArrayLiteral {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = Vec::new();
    for arg in &mut self.arguments {
      errors.append(&mut arg.analyze_object_declaration(table.clone()));
    }
    errors
  }
}

impl Argument {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.value.analyze_object_declaration(table)
  }
}

impl InterpolatedString {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errs = Vec::new();
    for expr in &mut self.subexpressions {
      errs.append(&mut expr.analyze_object_declaration(table.clone()));
    }
    errs
  }
}

impl Tuple {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errs = Vec::new();
    for member in &mut self.tuple_members {
      errs.append(&mut member.analyze_object_declaration(table.clone()));
    }
    errs
  }
}

impl TupleMember {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.value.analyze_object_declaration(table)
  }
}

impl Call {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errs = self.target.analyze_object_declaration(table.clone());
    for arg in &mut self.arguments {
      errs.append(&mut arg.value.analyze_object_declaration(table.clone()));
    }
    errs
  }
}

impl ArrayIndex {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errs = self.target.analyze_object_declaration(table.clone());
    for arg in &mut self.arguments {
      errs.append(&mut arg.value.analyze_object_declaration(table.clone()));
    }
    errs
  }
}

impl SubExpression {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.expression.analyze_object_declaration(table)
  }
}

impl Cast {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.expression.analyze_object_declaration(table)
  }
}

impl PrefixOperator {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.expression.analyze_object_declaration(table)
  }
}

impl PostfixOperator {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    self.expression.analyze_object_declaration(table)
  }
}

impl BinaryOperator {
  pub fn analyze_object_declaration(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errs = self.left_expression.analyze_object_declaration(table.clone());
    errs.append(&mut self.right_expression.analyze_object_declaration(table));
    errs
  }
}
