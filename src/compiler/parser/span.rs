use super::ast::*;

pub trait Spanned {
  fn get_span(&self) -> (usize, usize);
}

impl Spanned for Program {
  fn get_span(&self) -> (usize, usize) {
    if self.statements.is_empty() {
      return (0, 0);
    }
    let (first_stmt_start, _) = self.statements[0].get_span();
    let (_, last_stmt_end) = self.statements[self.statements.len() - 1].get_span();
    (first_stmt_start, last_stmt_end)
  }
}

impl Spanned for Statement {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Statement::Error(x) => x.get_span(),
      Statement::Macro(x) => x.get_span(),
      Statement::Continue(x) => x.get_span(),
      Statement::Break(x) => x.get_span(),
      Statement::Return(x) => x.get_span(),
      Statement::PackDec(x) => x.get_span(),
      Statement::EnumDec(x) => x.get_span(),
      Statement::VarDec(x) => x.get_span(),
      Statement::Cast(x) => x.get_span(),
      Statement::Match(x) => x.get_span(),
      Statement::Use(x) => x.get_span(),
      Statement::If(x) => x.get_span(),
      Statement::ForLoop(x) => x.get_span(),
      Statement::WhileLoop(x) => x.get_span(),
      Statement::InfiniteLoop(x) => x.get_span(),
      Statement::Expression(x) => x.get_span(),
    }
  }
}

impl Spanned for MacroStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for ErrorStatement {
  fn get_span(&self) -> (usize, usize) {
    if self.tokens.is_empty() {
      return (0, 0);
    }
    (self.tokens[0].start, self.tokens[self.tokens.len() - 1].end)
  }
}

impl Spanned for ContinueStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for BreakStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for ReturnStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for PackDecStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.pack_token.start, self.close_brace.end)
  }
}

impl Spanned for PackDeclaration {
  fn get_span(&self) -> (usize, usize) {
    let (type_var_start, type_var_end) = self.type_var.get_span();
    match &self.expression {
      Some(x) => {
        let (_, expr_end) = x.get_span();
        (type_var_start, expr_end)
      }
      None => (type_var_start, type_var_end),
    }
  }
}

impl Spanned for EnumDecStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.enum_token.start, self.close_brace.end)
  }
}

impl Spanned for EnumDeclaration {
  fn get_span(&self) -> (usize, usize) {
    match &self.enum_storage {
      Some(x) => {
        let (_, enum_end) = x.get_span();
        (self.identifier.start, enum_end)
      }
      None => (self.identifier.start, self.identifier.end),
    }
  }
}

impl Spanned for EnumStorage {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren.start, self.right_paren.end)
  }
}

impl Spanned for VarDecStatement {
  fn get_span(&self) -> (usize, usize) {
    let let_start = self.let_token.start;
    match &self.expression {
      Some(x) => {
        let (_, expr_end) = x.get_span();
        (let_start, expr_end)
      }
      None => match &self.function {
        Some(x) => {
          let (_, func_end) = x.get_span();
          (let_start, func_end)
        }
        None => (let_start, self.assignment.end),
      },
    }
  }
}

impl Spanned for CastStatement {
  fn get_span(&self) -> (usize, usize) {
    let (_, func_end) = self.function.get_span();
    (self.cast_token.start, func_end)
  }
}

impl Spanned for MatchStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.match_token.start, self.right_curly.end)
  }
}

impl Spanned for MatchEntry {
  fn get_span(&self) -> (usize, usize) {
    let (expr_start, _) = self.match_expression.get_span();
    (expr_start, self.right_curly.end)
  }
}

impl Spanned for IfStatement {
  fn get_span(&self) -> (usize, usize) {
    match &self.else_token {
      Some(_) => match &self.else_right_curly {
        Some(x) => (self.if_token.start, x.end),
        None => {
          assert!(self.else_body.len() == 1);
          let (_, body_end) = self.else_body[0].get_span();
          (self.if_token.start, body_end)
        }
      },
      None => (self.if_token.start, self.right_curly.end),
    }
  }
}

impl Spanned for UseStatement {
  fn get_span(&self) -> (usize, usize) {
    if self.id_tokens.is_empty() {
      return (self.use_token.start, self.use_token.end);
    }
    match &self.alias_token {
      Some(x) => (self.use_token.start, x.end),
      None => (
        self.use_token.start,
        self.id_tokens[self.id_tokens.len() - 1].end,
      ),
    }
  }
}

impl Spanned for ForLoopStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.loop_token.start, self.right_curly.end)
  }
}

impl Spanned for WhileStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.loop_token.start, self.right_curly.end)
  }
}

impl Spanned for InfiniteLoopStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.loop_token.start, self.right_curly.end)
  }
}

impl Spanned for ExpressionStatement {
  fn get_span(&self) -> (usize, usize) {
    self.expression.get_span()
  }
}

impl Spanned for Function {
  fn get_span(&self) -> (usize, usize) {
    match &self.right_curly {
      Some(x) => (self.param_left_paren.start, x.end),
      None => (self.param_left_paren.start, self.return_right_paren.end),
    }
  }
}

impl Spanned for Expression {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Expression::Binary(x) => x.get_span(),
      Expression::Prefix(x) => x.get_span(),
      Expression::Postfix(x) => x.get_span(),
      Expression::Member(x) => x.get_span(),
      Expression::ArrayAccess(x) => x.get_span(),
      Expression::Cast(x) => x.get_span(),
      Expression::Literal(x) => x.get_span(),
      Expression::Var(x) => x.get_span(),
      Expression::FunctionCall(x) => x.get_span(),
    }
  }
}

impl Spanned for BinaryExpression {
  fn get_span(&self) -> (usize, usize) {
    let (lhs_start, _) = self.lhs.get_span();
    let (_, rhs_end) = self.rhs.get_span();
    (lhs_start, rhs_end)
  }
}

impl Spanned for PrefixExpression {
  fn get_span(&self) -> (usize, usize) {
    let (_, rhs_end) = self.rhs.get_span();
    (self.operator.start, rhs_end)
  }
}

impl Spanned for PostfixExpression {
  fn get_span(&self) -> (usize, usize) {
    let (lhs_start, _) = self.lhs.get_span();
    (lhs_start, self.operator.end)
  }
}

impl Spanned for MemberAccess {
  fn get_span(&self) -> (usize, usize) {
    let (lhs_start, _) = self.lhs.get_span();
    (lhs_start, self.id.end)
  }
}

impl Spanned for ArrayAccess {
  fn get_span(&self) -> (usize, usize) {
    let (lhs_start, _) = self.lhs.get_span();
    (lhs_start, self.right_square.end)
  }
}

impl Spanned for CastExpression {
  fn get_span(&self) -> (usize, usize) {
    let (lhs_start, _) = self.lhs.get_span();
    let (_, type_end) = self.cast_type.get_span();
    (lhs_start, type_end)
  }
}

impl Spanned for FunctionCall {
  fn get_span(&self) -> (usize, usize) {
    let (target_start, _) = self.target.get_span();
    (target_start, self.right_paren.end)
  }
}

impl Spanned for Literal {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Literal::Boolean(x) => (x.start, x.end),
      Literal::Number(x) => (x.start, x.end),
      Literal::String(x) => (x.start, x.end),
      Literal::Array(x) => x.get_span(),
      Literal::Tuple(x) => x.get_span(),
    }
  }
}

impl Spanned for Tuple {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren.start, self.right_paren.end)
  }
}

impl Spanned for ArrayLiteral {
  fn get_span(&self) -> (usize, usize) {
    (self.left_square.start, self.right_square.end)
  }
}

impl Spanned for Var {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Var::Typed(x) => x.get_span(),
      Var::Untyped(x) => x.get_span(),
    }
  }
}

impl Spanned for TypeVar {
  fn get_span(&self) -> (usize, usize) {
    let (var_start, _) = self.var.get_span();
    let (_, var_type_end) = (*self.var_type).get_span();
    (var_start, var_type_end)
  }
}

impl Spanned for UntypedVar {
  fn get_span(&self) -> (usize, usize) {
    (self.id.start, self.id.end)
  }
}

impl Spanned for Type {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Type::Auto(x) => x.get_span(),
      Type::Comp(x) => x.get_span(),
      Type::Sub(x) => x.get_span(),
      Type::Func(x) => x.get_span(),
      Type::Base(x) => x.get_span(),
      Type::Lazy(x) => x.get_span(),
      Type::Ref(x) => x.get_span(),
      Type::Optional(x) => x.get_span(),
    }
  }
}

impl Spanned for AutoType {
  fn get_span(&self) -> (usize, usize) {
    match &self.auto_name {
      Some(x) => (self.auto_token.start, x.end),
      None => (self.auto_token.start, self.auto_token.end),
    }
  }
}

impl Spanned for CompType {
  fn get_span(&self) -> (usize, usize) {
    let (_, sub_type_end) = (*self.sub_type).get_span();
    (self.comp_token.start, sub_type_end)
  }
}

impl Spanned for SubType {
  fn get_span(&self) -> (usize, usize) {
    (self.left_paren.start, self.right_paren.end)
  }
}

impl Spanned for FuncType {
  fn get_span(&self) -> (usize, usize) {
    match &self.right_paren {
      Some(x) => (self.func_token.start, x.end),
      None => (self.func_token.start, self.func_token.end),
    }
  }
}

impl Spanned for BaseType {
  fn get_span(&self) -> (usize, usize) {
    (self.base_token.start, self.base_token.end)
  }
}

impl Spanned for LazyType {
  fn get_span(&self) -> (usize, usize) {
    let (_, sub_type_end) = (*self.sub_type).get_span();
    (self.lazy_token.start, sub_type_end)
  }
}

impl Spanned for RefType {
  fn get_span(&self) -> (usize, usize) {
    let (_, sub_type_end) = (*self.sub_type).get_span();
    (self.ref_token.start, sub_type_end)
  }
}

impl Spanned for OptionalType {
  fn get_span(&self) -> (usize, usize) {
    let (_, sub_type_end) = (*self.sub_type).get_span();
    (self.optional_token.start, sub_type_end)
  }
}
