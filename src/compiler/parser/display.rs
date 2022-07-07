use super::ast::*;
use std::fmt;

impl fmt::Display for Program {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(Program")?;
    for stmt in &self.statements {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for Statement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Statement::Error(x) => x.fmt(fmt),
      Statement::Macro(x) => x.fmt(fmt),
      Statement::Continue(x) => x.fmt(fmt),
      Statement::Break(x) => x.fmt(fmt),
      Statement::Return(x) => x.fmt(fmt),
      Statement::PackDec(x) => x.fmt(fmt),
      Statement::UnionDec(x) => x.fmt(fmt),
      Statement::VarDec(x) => x.fmt(fmt),
      Statement::Cast(x) => x.fmt(fmt),
      Statement::Match(x) => x.fmt(fmt),
      Statement::Use(x) => x.fmt(fmt),
      Statement::If(x) => x.fmt(fmt),
      Statement::ForLoop(x) => x.fmt(fmt),
      Statement::WhileLoop(x) => x.fmt(fmt),
      Statement::InfiniteLoop(x) => x.fmt(fmt),
      Statement::Expression(x) => x.fmt(fmt),
    }
  }
}

impl fmt::Display for ErrorStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(ErrorStatement '{:?}' '{}')", self.severity, self.message).as_str())?;
    Ok(())
  }
}

impl fmt::Display for MacroStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(MacroStatement '{}')", self.token.lexeme).as_str())?;
    Ok(())
  }
}

impl fmt::Display for ContinueStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(ContinueStatement)")?;
    Ok(())
  }
}

impl fmt::Display for BreakStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(BreakStatement)")?;
    Ok(())
  }
}

impl fmt::Display for ReturnStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(ReturnStatment)")?;
    Ok(())
  }
}

impl fmt::Display for PackDecStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(PackDecStatement ")?;
    fmt.write_str(format!("(Name '{}')", self.name_token.lexeme).as_str())?;
    for pack_dec in &self.pack_declarations {
      fmt.write_str(format!(" {}", pack_dec.to_string()).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for PackDeclaration {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(PackDec {}{})",
        self.type_var,
        match &self.expression {
          Some(x) => format!(" {}", x),
          None => "".to_string(),
        }
      )
      .as_str(),
    )?;
    Ok(())
  }
}

impl fmt::Display for UnionDecStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(UnionDecStatement ")?;
    fmt.write_str(format!("(Name '{}')", self.name_token.lexeme).as_str())?;
    for union_dec in &self.union_declarations {
      fmt.write_str(format!(" {}", union_dec.to_string()).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for UnionDeclaration {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(UnionDec '{}'",
        self.identifier.lexeme,
      )
      .as_str(),
    )?;
    for union_type in &self.type_list {
      fmt.write_str(format!(" {}", union_type).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for VarDecStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(VarDecStatement {} '{}'{}{})",
        self.var,
        self.assignment.lexeme,
        match &self.expression {
          Some(x) => format!(" {}", x),
          None => "".to_string(),
        },
        match &self.function {
          Some(x) => format!(" {}", x),
          None => "".to_string(),
        }
      )
      .as_str(),
    )?;
    Ok(())
  }
}

impl fmt::Display for CastStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(CastStatement {})", self.function).as_str())?;
    Ok(())
  }
}

impl fmt::Display for MatchStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(MatchStatement {} (Cases", self.match_condition).as_str())?;
    for entry in &self.match_entries {
      fmt.write_str(format!(" {}", entry).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for MatchEntry {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(MatchEntry {} (Body", self.match_expression).as_str())?;
    for stmt in &self.statement_list {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for IfStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(IfStatement {} (TrueBody", self.condition).as_str())?;
    for stmt in &self.true_body {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str(")")?;
    match &self.else_token {
      Some(_) => {
        fmt.write_str(" (ElseBody")?;
        for stmt in &self.else_body {
          fmt.write_str(format!(" {}", stmt).as_str())?;
        }
        fmt.write_str(")")?;
      }
      None => {}
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for UseStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(UseStatement")?;
    for token in &self.id_tokens {
      fmt.write_str(format!(" (Id {})", token.lexeme).as_str())?;
    }
    match &self.as_token {
      Some(_) => {
        fmt.write_str(" As")?;
        match &self.alias_token {
          Some(id) => fmt.write_str(format!(" (Id {})", id.lexeme).as_str())?,
          None => {}
        }
      }
      None => {}
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for ForLoopStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(ForLoopStatement '{}' {} (Body",
        self.iterator.lexeme, self.iterable
      )
      .as_str(),
    )?;
    for stmt in &self.loop_body {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for WhileStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(WhileStatement {} (Body", self.condition).as_str())?;
    for stmt in &self.loop_body {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for InfiniteLoopStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(InfiniteLoopStatement (Body")?;
    for stmt in &self.loop_body {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for ExpressionStatement {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(ExpressionStatement {})", self.expression).as_str())?;
    Ok(())
  }
}

impl fmt::Display for Function {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(Function (Params {}) (Returns {}) (Body",
        self.param_list, self.return_list
      )
      .as_str(),
    )?;
    for stmt in &self.function_body {
      fmt.write_str(format!(" {}", stmt).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for Expression {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Binary(x) => x.fmt(fmt),
      Expression::Prefix(x) => x.fmt(fmt),
      Expression::Postfix(x) => x.fmt(fmt),
      Expression::Member(x) => x.fmt(fmt),
      Expression::ArrayAccess(x) => x.fmt(fmt),
      Expression::Cast(x) => x.fmt(fmt),
      Expression::Literal(x) => x.fmt(fmt),
      Expression::Var(x) => x.fmt(fmt),
      Expression::FunctionCall(x) => x.fmt(fmt),
    }
  }
}

impl fmt::Display for BinaryExpression {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(BinaryExpression '{}' {} {})",
        self.operator.lexeme, self.lhs, self.rhs
      )
      .as_str(),
    )?;
    Ok(())
  }
}

impl fmt::Display for PrefixExpression {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt
      .write_str(format!("(PrefixExpression '{}' {})", self.operator.lexeme, self.rhs).as_str())?;
    Ok(())
  }
}

impl fmt::Display for PostfixExpression {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(PostfixExpression '{}' {})",
        self.operator.lexeme, self.lhs
      )
      .as_str(),
    )?;
    Ok(())
  }
}

impl fmt::Display for MemberAccess {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(MemberAccess {} '{}')", self.lhs, self.id.lexeme).as_str())?;
    Ok(())
  }
}

impl fmt::Display for ArrayAccess {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(ArrayAccess {} {})", self.lhs, self.expr).as_str())?;
    Ok(())
  }
}

impl fmt::Display for CastExpression {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(CastExpression {} {})", self.lhs, self.cast_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for FunctionCall {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(FunctionCall {} (Args", self.target).as_str())?;
    for (exp, _comma) in &self.arguments {
      fmt.write_str(format!(" {}", exp).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for Literal {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Literal::Boolean(x) => fmt.write_str(format!("(Boolean '{}')", x.lexeme).as_str())?,
      Literal::Number(x) => fmt.write_str(format!("(Number '{}')", x.lexeme).as_str())?,
      Literal::String(x) => fmt.write_str(format!("(String '{}')", x.lexeme).as_str())?,
      Literal::Array(x) => x.fmt(fmt)?,
      Literal::Tuple(x) => x.fmt(fmt)?,
    };
    Ok(())
  }
}

impl fmt::Display for Tuple {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(Tuple")?;
    for (exp, _comma) in &self.contents {
      fmt.write_str(format!(" {}", exp).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for ArrayLiteral {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(ArrayLiteral")?;
    for (arg, _comma) in &self.args {
      fmt.write_str(format!(" {}", arg).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for Var {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Var::Typed(x) => x.fmt(fmt),
      Var::Untyped(x) => x.fmt(fmt),
    }
  }
}

impl fmt::Display for TypeVar {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(TypeVar {} {})", self.var, self.var_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for UntypedVar {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(Var {})", self.id.lexeme).as_str())?;
    Ok(())
  }
}

impl fmt::Display for Type {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Type::Auto(x) => x.fmt(fmt)?,
      Type::Comp(x) => x.fmt(fmt)?,
      Type::Sub(x) => x.fmt(fmt)?,
      Type::Func(x) => x.fmt(fmt)?,
      Type::Base(x) => x.fmt(fmt)?,
      Type::Lazy(x) => x.fmt(fmt)?,
      Type::Ref(x) => x.fmt(fmt)?,
      Type::Optional(x) => x.fmt(fmt)?,
      Type::Array(x) => x.fmt(fmt)?,
    };
    Ok(())
  }
}

impl fmt::Display for AutoType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(AutoType")?;
    fmt.write_str(
      match &self.auto_name {
        Some(x) => format!(" '{}')", x.lexeme),
        None => ")".to_string(),
      }
      .as_str(),
    )?;
    Ok(())
  }
}

impl fmt::Display for CompType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(CompType {})", self.sub_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for SubType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(SubType {})", self.sub_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for FuncType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(FuncType (ParamTypes")?;
    for param in &self.param_types {
      fmt.write_str(format!(" {}", param).as_str())?;
    }
    fmt.write_str(") (ReturnTypes")?;
    for return_type in &self.return_types {
      fmt.write_str(format!(" {}", return_type).as_str())?;
    }
    fmt.write_str("))")?;
    Ok(())
  }
}

impl fmt::Display for BaseType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(BaseType '{}')", self.base_token.lexeme).as_str())?;
    Ok(())
  }
}

impl fmt::Display for LazyType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(LazyType {})", self.sub_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for RefType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(RefType {})", self.sub_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for OptionalType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(OptionalType {})", self.sub_type).as_str())?;
    Ok(())
  }
}

impl fmt::Display for ArrayType {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(format!("(ArrayType {}", self.base).as_str())?;
    match &*self.sub_type {
      Some(x) => fmt.write_str(format!(" (IndexType {})", x).as_str())?,
      None => {}
    }
    fmt.write_str(")")
  }
}

impl fmt::Display for ParameterList {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(ParameterList")?;
    for (param, _comma) in &self.params {
      fmt.write_str(format!(" {}", param).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for Parameter {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match &self.type_var {
      Some(x) => fmt.write_str(format!("(Parameter {})", x).as_str())?,
      None => match &self.var_arg_token {
        Some(x) => fmt.write_str(format!("(Parameter {})", x.lexeme).as_str())?,
        None => {}
      },
    };
    Ok(())
  }
}

impl fmt::Display for ReturnList {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str("(ReturnList")?;
    for (ret, _comma) in &self.returns {
      fmt.write_str(format!(" {}", ret).as_str())?;
    }
    fmt.write_str(")")?;
    Ok(())
  }
}

impl fmt::Display for ReturnEntry {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt.write_str(
      format!(
        "(Return {}{})",
        self.type_var,
        match &self.expression {
          Some(x) => format!(" {}", x),
          None => "".to_string(),
        }
      )
      .as_str(),
    )?;
    Ok(())
  }
}
