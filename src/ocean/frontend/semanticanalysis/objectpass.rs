use std::cell::RefCell;
use std::rc::Rc;
use itertools::{Either, Itertools};
use uuid::Uuid;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::util::errors::{Error, ErrorMetadata, Severity};
use crate::util::span::Spanned;

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
    let mut errors = match &mut self.left_expression {
      Either::Left(_) => Vec::new(),
      Either::Right(expr) => expr.analyze_object(table.clone()),
    };

    errors.append(&mut self.right_expression.analyze_object(table.clone()));
    errors
  }
}

impl Function {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    Vec::new()
  }
}

impl Pack {
  pub fn analyze_object(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
    let mut errors = Vec::new();

    let pack_name = self.custom_type.get_name();
    //let pack_type_arguments = self.custom_type.get_type_arguments();
    let interfaces = match &self.interface_declaration {
      Some(interface_declaraion) => interface_declaraion.implemented_interfaces.iter()
        .map(|x| (x.interface_type.get_name(), x.interface_type.get_type_arguments(), x.interface_type.get_span()))
        .collect::<Vec<(String, Vec<Type>, (usize, usize))>>(),
      None => Vec::new(),
    };
    let mut interface_uuids = Vec::new();
    for (interface_name, _, interface_span) in interfaces {
      match table.borrow().find_interface(&interface_name) {
        Some(interface_uuid) => interface_uuids.push(interface_uuid),
        None => errors.push(Error::new(Severity::Error, interface_span, "Interface not found in scope.".to_string())),
      }
    }

    let members = self.members.iter().map(|x| (x.identifier.identifier.lexeme.clone(), x.identifier.optional_type.clone().unwrap(), x.identifier.get_span())).collect::<Vec<(String, Type, (usize, usize))>>();
    let mut member_uuids: Vec<(String, Uuid, (usize, usize))> = Vec::new();
    for (member_name, member_type, member_span) in members {
      let member_type_id = table.borrow_mut().add_type(member_type);
      let conflict = member_uuids.iter().find(|x| x.0 == member_name);
      if let Some((_, _, conflicted_span)) = conflict {
        errors.push(Error::new_with_metadata(
          Severity::Error,
          member_span,
          "Pack already has member with the same name".to_string(),
          ErrorMetadata::new().extra_highlighted_info(
            conflicted_span.clone(),
            "Member previously defined here".to_string()), ))
      } else {
        member_uuids.push((member_name.clone(), member_type_id, member_span));
      }
    }

    errors
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

