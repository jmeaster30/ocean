use itertools::Either;
use crate::ocean::frontend::compilationunit::ast::nodestructs::*;
use crate::util::span::Spanned;

impl Spanned for Statement {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Annotation {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for CompoundStatement {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for WhileLoop {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ForLoop {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Loop {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Branch {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ElseBranch {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Match {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Assignment {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for LetTarget {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Identifier {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Function {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Pack {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Union {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Interface {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Return {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}
impl Spanned for Break {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Continue {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Using {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ExpressionStatement {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ExpressionNode {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for BaseType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for CustomType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for TypeParameters {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for AutoType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for LazyType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for RefType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for MutType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for FunctionType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for SubType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ArrayType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for VariableType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for TupleType {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ArrayLiteral {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for InterpolatedString {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Variable {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Tuple {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for TupleMember {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Call {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for ArrayIndex {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for SubExpression {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for Cast {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for PrefixOperator {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for PostfixOperator {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}

impl Spanned for BinaryOperator {
  fn get_span(&self) -> (usize, usize) {
    todo!()
  }
}