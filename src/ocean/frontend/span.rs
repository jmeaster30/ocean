use itertools::Either;
use crate::ocean::frontend::ast::node::*;
use crate::ocean::frontend::ast::typenode::*;
use crate::util::span::Spanned;

impl Spanned for Program {
  fn get_span(&self) -> (usize, usize) {
    let first = self.statements.first();
    let last = self.statements.last();
    if let (Some(first), Some(last)) = (first, last) {
      (first.get_span().0, last.get_span().1)
    } else {
      (0, 0)
    }
  }
}

impl Spanned for StatementNode {
  fn get_span(&self) -> (usize, usize) {
    match (self.data.first(), &self.statement) {
      (Some(first), Some(statement)) => (first.get_span().0, statement.get_span().1),
      (Some(first), None) => (first.get_span().0, self.data.last().unwrap().get_span().1),
      (None, Some(statement)) => statement.get_span(),
      (None, None) => (0, 0)
    }
  }
}

impl Spanned for StatementNodeData {
  fn get_span(&self) -> (usize, usize) {
    match self {
      StatementNodeData::Annotation(annotation) => annotation.get_span(),
      StatementNodeData::Comment(comment) => comment.get_span()
    }
  }
}

impl Spanned for Comment {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for Annotation {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for CompoundStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.left_curly.get_span().0, self.right_curly.get_span().1)
  }
}

impl Spanned for Statement {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Statement::WhileLoop(x) => x.get_span(),
      Statement::ForLoop(x) => x.get_span(),
      Statement::Loop(x) => x.get_span(),
      Statement::Branch(x) => x.get_span(),
      Statement::Match(x) => x.get_span(),
      Statement::Assignment(x) => x.get_span(),
      Statement::Function(x) => x.get_span(),
      Statement::Pack(x) => x.get_span(),
      Statement::Union(x) => x.get_span(),
      Statement::Interface(x) => x.get_span(),
      Statement::Return(x) => x.get_span(),
      Statement::Break(x) => x.get_span(),
      Statement::Continue(x) => x.get_span(),
      Statement::Using(x) => x.get_span(),
      Statement::Expression(x) => x.get_span(),
    }
  }
}

impl Spanned for WhileLoop {
  fn get_span(&self) -> (usize, usize) {
    (self.while_token.get_span().0, self.body.get_span().1)
  }
}

impl Spanned for ForLoop {
  fn get_span(&self) -> (usize, usize) {
    (self.for_token.get_span().0, self.body.get_span().1)
  }
}

impl Spanned for Loop {
  fn get_span(&self) -> (usize, usize) {
    (self.loop_token.get_span().0, self.body.get_span().1)
  }
}

impl Spanned for Branch {
  fn get_span(&self) -> (usize, usize) {
    if let Some(else_branch) = &self.else_branch {
      (self.if_token.get_span().0, else_branch.get_span().1)
    } else {
      (self.if_token.get_span().0, self.body.get_span().1)
    }
  }
}

impl Spanned for ElseBranch {
  fn get_span(&self) -> (usize, usize) {
    (self.else_token.get_span().0, match &self.body {
      Either::Left(compound) => compound.get_span(),
      Either::Right(branch) => branch.get_span()
    }.1)
  }
}

impl Spanned for Match {
  fn get_span(&self) -> (usize, usize) {
    (self.match_token.get_span().0, self.right_curly.get_span().1)
  }
}

impl Spanned for Assignment {
  fn get_span(&self) -> (usize, usize) {
    (match &self.left_expression {
      Either::Left(let_target) => let_target.get_span().0,
      Either::Right(expression) => expression.get_span().0,
    }, self.right_expression.get_span().1)
  }
}

impl Spanned for LetTarget {
  fn get_span(&self) -> (usize, usize) {
    (self.let_token.get_span().0, self.identifier.get_span().1)
  }
}

impl Spanned for Identifier {
  fn get_span(&self) -> (usize, usize) {
    if let Some(assignment_type) = &self.optional_type {
      (self.identifier.get_span().0, assignment_type.get_span().1)
    } else {
      self.identifier.get_span()
    }
  }
}

impl Spanned for Function {
  fn get_span(&self) -> (usize, usize) {
    (self.function_token.get_span().0, match &self.compound_statement {
      Some(compound) => compound.get_span(),
      None => match &self.result_right_paren {
        Some(result_right_paren) => result_right_paren.get_span(),
        None => self.param_right_paren.get_span()
      }
    }.0)
  }
}

impl Spanned for Pack {
  fn get_span(&self) -> (usize, usize) {
    (self.pack_token.get_span().0, self.right_curly.get_span().1)
  }
}

impl Spanned for Union {
  fn get_span(&self) -> (usize, usize) {
    (self.union_token.get_span().0, self.right_curly.get_span().1)
  }
}

impl Spanned for Interface {
  fn get_span(&self) -> (usize, usize) {
    (self.interface_token.get_span().0, self.right_curly_token.get_span().1)
  }
}

impl Spanned for Return {
  fn get_span(&self) -> (usize, usize) {
    self.return_token.get_span()
  }
}
impl Spanned for Break {
  fn get_span(&self) -> (usize, usize) {
    self.break_token.get_span()
  }
}

impl Spanned for Continue {
  fn get_span(&self) -> (usize, usize) {
    self.continue_token.get_span()
  }
}

impl Spanned for Using {
  fn get_span(&self) -> (usize, usize) {
    if let Some(last) = self.path.last() {
      (self.using_token.get_span().0, last.identifier.get_span().1)
    } else {
      self.using_token.get_span()
    }
  }
}

impl Spanned for ExpressionStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.expression_node.get_span().0, self.semicolon_token.get_span().1)
  }
}

impl Spanned for ExpressionNode {
  fn get_span(&self) -> (usize, usize) {
    let first = self.tokens.first();
    let last = self.tokens.last();
    (match first {
      Some(Either::Left(token)) => token.get_span(),
      Some(Either::Right(exp)) => exp.get_span(),
      _ => (0, 0),
    }.0, match last {
      Some(Either::Left(token)) => token.get_span(),
      Some(Either::Right(exp)) => exp.get_span(),
      _ => (0, 0),
    }.0)
  }
}

impl Spanned for AstNodeExpression {
  fn get_span(&self) -> (usize, usize) {
    match self {
      AstNodeExpression::Branch(branch) => branch.get_span(),
      AstNodeExpression::Match(match_exp) => match_exp.get_span(),
      AstNodeExpression::Loop(loop_exp) => loop_exp.get_span(),
      AstNodeExpression::ForLoop(for_exp) => for_exp.get_span(),
      AstNodeExpression::WhileLoop(while_exp) => while_exp.get_span(),
      AstNodeExpression::Function(func) => func.get_span(),
    }
  }
}


impl Spanned for Type {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Type::Unknown => panic!("Should never get the span of an unknown type"),
      Type::Base(base) => base.get_span(),
      Type::Custom(custom) => custom.get_span(),
      Type::Auto(auto_type) => auto_type.get_span(),
      Type::Lazy(lazy_type) => lazy_type.get_span(),
      Type::Ref(ref_type) => ref_type.get_span(),
      Type::Mutable(mutable_type) => mutable_type.get_span(),
      Type::Function(function_type) => function_type.get_span(),
      Type::Array(array_type) => array_type.get_span(),
      Type::VariableType(var_type) => var_type.get_span(),
      Type::TupleType(tuple_type) => tuple_type.get_span(),
    }
  }
}

impl Spanned for BaseType {
  fn get_span(&self) -> (usize, usize) {
    self.base_type.get_span()
  }
}

impl Spanned for CustomType {
  fn get_span(&self) -> (usize, usize) {
    let id = self.identifier.get_span();
    (id.0, match &self.type_parameters {
      Some(type_parameters) => type_parameters.get_span().1,
      None => id.1,
    })
  }
}

impl Spanned for TypeParameters {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren_token.get_span().0, self.right_paren_token.get_span().1)
  }
}

impl Spanned for AutoType {
  fn get_span(&self) -> (usize, usize) {
    (self.auto_token.get_span().0, self.identifier.get_span().1)
  }
}

impl Spanned for LazyType {
  fn get_span(&self) -> (usize, usize) {
    (self.lazy_token.get_span().0, self.base_type.get_span().1)
  }
}

impl Spanned for RefType {
  fn get_span(&self) -> (usize, usize) {
    (self.ref_token.get_span().0, self.base_type.get_span().1)
  }
}

impl Spanned for MutType {
  fn get_span(&self) -> (usize, usize) {
    (self.mut_token.get_span().0, self.base_type.get_span().1)
  }
}

impl Spanned for FunctionType {
  fn get_span(&self) -> (usize, usize) {
    (self.function_token.get_span().0, match &self.result_right_paren {
      Some(right_paren) => right_paren.get_span().1,
      None => self.param_right_paren.get_span().1
    })
  }
}

impl Spanned for SubType {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren_token.get_span().0, self.right_paren_token.get_span().1)
  }
}

impl Spanned for ArrayType {
  fn get_span(&self) -> (usize, usize) {
    (self.base_type.get_span().0, self.right_square.get_span().1)
  }
}

impl Spanned for VariableType {
  fn get_span(&self) -> (usize, usize) {
    (self.base_type.get_span().0, self.spread_token.get_span().1)
  }
}

impl Spanned for TupleType {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren_token.get_span().0, self.right_paren_token.get_span().1)
   }
}

impl Spanned for Expression {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Expression::String(x) => x.get_span(),
      Expression::ArrayLiteral(x) => x.get_span(),
      Expression::Number(x) => x.get_span(),
      Expression::Boolean(x) => x.get_span(),
      Expression::InterpolatedString(x) => x.get_span(),
      Expression::Variable(x) => x.get_span(),
      Expression::Tuple(x) => x.get_span(),
      Expression::Call(x) => x.get_span(),
      Expression::ArrayIndex(x) => x.get_span(),
      Expression::SubExpression(x) => x.get_span(),
      Expression::Cast(x) => x.get_span(),
      Expression::PrefixOperation(x) => x.get_span(),
      Expression::PostfixOperation(x) => x.get_span(),
      Expression::BinaryOperation(x) => x.get_span(),
      Expression::AstNode(x) => x.get_span(),
    }
  }
}

impl Spanned for StringLiteral {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for ArrayLiteral {
  fn get_span(&self) -> (usize, usize) {
    (self.left_square_token.get_span().0, self.right_square_token.get_span().1)
  }
}

impl Spanned for Number {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for Boolean {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for InterpolatedString {
  fn get_span(&self) -> (usize, usize) {
    self.token.get_span()
  }
}

impl Spanned for Variable {
  fn get_span(&self) -> (usize, usize) {
    self.identifier.get_span()
  }
}

impl Spanned for Tuple {
  fn get_span(&self) -> (usize, usize) {
    (self.left_token.get_span().0, self.right_token.get_span().1)
  }
}

impl Spanned for TupleMember {
  fn get_span(&self) -> (usize, usize) {
    let left = match self.identifier.clone() {
      Some(id) => id.get_span().0,
      None => self.value.get_span().0,
    };

    let right = match self.comma_token.clone() {
      Some(comma) => comma.get_span().1,
      None => self.value.get_span().1
    };
    (left, right)
  }
}

impl Spanned for Call {
  fn get_span(&self) -> (usize, usize) {
    (self.target.get_span().0, self.right_paren.get_span().1)
  }
}

impl Spanned for ArrayIndex {
  fn get_span(&self) -> (usize, usize) {
    (self.target.get_span().0, self.right_square.get_span().1)
  }
}

impl Spanned for SubExpression {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren.get_span().0, self.right_paren.get_span().1)
  }
}

impl Spanned for Cast {
  fn get_span(&self) -> (usize, usize) {
    (self.expression.get_span().0, self.casted_type.get_span().1)
  }
}

impl Spanned for PrefixOperator {
  fn get_span(&self) -> (usize, usize) {
    (self.operator.get_span().0, self.expression.get_span().1)
  }
}

impl Spanned for PostfixOperator {
  fn get_span(&self) -> (usize, usize) {
    (self.expression.get_span().0, self.operator.get_span().1)
  }
}

impl Spanned for BinaryOperator {
  fn get_span(&self) -> (usize, usize) {
    (self.left_expression.get_span().0, self.right_expression.get_span().1)
  }
}