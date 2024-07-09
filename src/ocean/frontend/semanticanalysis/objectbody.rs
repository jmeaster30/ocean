use std::cell::RefCell;
use std::rc::Rc;
use itertools::Either;
use uuid::Uuid;
use crate::ocean::frontend::ast::*;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;
use crate::util::errors::{Error, Severity};
use crate::util::hashablemap::HashableMap;
use crate::util::span::Spanned;

impl Program {
    pub fn analyze_object_body(&mut self) -> Vec<Error> {
        self.statements.iter_mut()
            .map(|x| x.analyze_object_body(self.table.clone().unwrap()))
            .reduce(|mut a, mut b| { a.append(&mut b); a })
            .unwrap()
    }
}

impl StatementNode {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        self.statement.as_mut()
            .map(|x| x.analyze_object_body(table))
            .unwrap_or(Vec::new())
    }
}

impl Statement {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        match self {
            Statement::WhileLoop(x) => x.analyze_object_body(table),
            Statement::ForLoop(x) => x.analyze_object_body(table),
            Statement::Loop(x) => x.analyze_object_body(table),
            Statement::Branch(x) => x.analyze_object_body(table),
            Statement::Match(x) => x.analyze_object_body(table),
            Statement::Assignment(x) => x.analyze_object_body(table),
            Statement::Function(x) => x.analyze_object_body(table),
            Statement::Pack(x) => x.analyze_object_body(table),
            Statement::Union(x) => x.analyze_object_body(table),
            Statement::Interface(x) => x.analyze_object_body(table),
            Statement::Return(x) => Vec::new(),
            Statement::Break(x) => Vec::new(),
            Statement::Continue(x) => Vec::new(),
            Statement::Using(x) => Vec::new(),
            Statement::Expression(x) => x.analyze_object_body(table),
        }
    }
}


impl WhileLoop {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let mut errors = self.condition.analyze_object_body(table.clone());
        errors.append(&mut self.body.analyze_object_body(self.table.clone().unwrap()));
        errors
    }
}

impl ForLoop {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let mut errors = self.iterable.analyze_object_body(table.clone());
        errors.append(&mut self.iterator.analyze_object_body(self.table.clone().unwrap()));
        errors.append(&mut self.body.analyze_object_body(self.table.clone().unwrap()));
        errors
    }
}

impl Loop {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        self.body.analyze_object_body(self.table.clone().unwrap())
    }
}

impl Branch {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let mut errors = self.condition.analyze_object_body(table.clone());
        errors.append(&mut self.body.analyze_object_body(self.table.clone().unwrap()));

        if let Some(else_branch) = &mut self.else_branch {
            errors.append(&mut else_branch.analyze_object_body(table.clone()));
        }

        errors
    }
}

impl ElseBranch {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        match &mut self.body {
            Either::Left(compound) => compound.analyze_object_body(self.table.clone().unwrap()),
            Either::Right(branch) => branch.analyze_object_body(self.table.clone().unwrap()),
        }
    }
}


impl Match {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        todo!()
    }
}

impl Assignment {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let mut errors = match &mut self.left_expression {
            Either::Left(_) => Vec::new(),
            Either::Right(expr) => expr.analyze_object_body(table.clone()),
        };

        errors.append(&mut self.right_expression.analyze_object_body(table.clone()));
        errors
    }
}

impl Function {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let new_table = SymbolTable::hard_scope(Some(table));
        self.table = Some(new_table.clone());
        let mut errors = Vec::new();
        for result in &mut self.results {
            if let Some(result_expression) = &mut result.expression {
                let mut errs = result_expression.analyze_object_body(new_table.clone());
                errors.append(&mut errs);
            }
        }

        if let Some(compound) = &mut self.compound_statement {
            let mut errs = compound.analyze_object_body(new_table.clone());
            errors.append(&mut errs);
        }

        errors
    }
}

impl Pack {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        let mut errors = Vec::new();
        let pack_name = self.custom_type.get_name();

        let type_args = self.custom_type.get_type_arguments().iter_mut()
            .map(|x| (x.get_span(), table.borrow_mut().find_type(x)))
            .map(|(a, b)| if let Some(type_uuid) = b {
                Some(type_uuid)
            } else {
                errors.push(Error::new(Severity::Error, a, "Undeclared type".to_string()));
                None
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<Uuid>>();

        let interfaces = self.interface_declaration.as_mut()
            .map_or(Vec::new(), |x| x.implemented_interfaces.iter_mut().map(|y| {
                if let Some(interface_uuid) = table.borrow_mut().find_interface(&y.interface_type.get_name(), false) {
                    Some(interface_uuid)
                } else {
                    errors.push(Error::new(Severity::Error, y.interface_type.get_span(), "Undeclared interface".to_string()));
                    None
                }}).collect::<Vec<Option<Uuid>>>())
            .iter().filter(|x| x.is_some()).map(|x| x.unwrap())
            .collect::<Vec<Uuid>>();

        let mut members = HashableMap::new();
        for member in &self.members {
            if member.identifier.clone().optional_type.is_none() {
                errors.push(Error::new(Severity::Error, member.identifier.clone().get_span(), "Must declare type for pack member.".to_string()));
                continue
            }

            if let Some(type_uuid) = table.borrow_mut().find_type(member.identifier.clone().optional_type.unwrap()) {
                members.insert(member.clone().identifier.identifier.lexeme, type_uuid);
            } else {
                errors.push(Error::new(Severity::Error, member.identifier.clone().optional_type.unwrap().get_span(), "Undeclared type".to_string())).clone();
            }
        }

        errors.append(&mut table.borrow_mut().add_pack_type_args(&pack_name, type_args));
        errors.append(&mut table.borrow_mut().add_pack_interfaces(&pack_name, interfaces));
        errors.append(&mut table.borrow_mut().add_pack_members(&pack_name, members));

        errors
    }
}

impl Union {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        Vec::new()
    }
}

impl Interface {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        Vec::new()
    }
}

impl CompoundStatement {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        self.body.iter_mut()
            .map(|x| x.analyze_object_body(table.clone()))
            .reduce(|mut a, mut b| { a.append(&mut b); a })
            .unwrap()
    }
}

impl ExpressionStatement {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        Vec::new()
    }
}

impl ExpressionNode {
    pub fn analyze_object_body(&mut self, table: Rc<RefCell<SymbolTable>>) -> Vec<Error> {
        Vec::new()
    }
}