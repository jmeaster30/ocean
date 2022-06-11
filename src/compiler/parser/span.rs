use super::ast::*;

pub trait Spanned {
  fn get_span(&self) -> (usize, usize);
}

impl Spanned for MacroStatement {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for ErrorStatement {
  fn get_span(&self) -> (usize, usize) {
    if (self.tokens.is_empty()) {
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
    match self.expression {
      Some(x) => {
        let (expr_start, expr_end) = x.get_span();
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
    match self.enum_storage {
      Some(x) => {
        let (enum_start, enum_end) = x.get_space();
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
    let (type_var_start, type_var_end) = self.type_var.get_span();
    match self.expression {
      Some(x) => {
        let (expr_start, expr_end) = x.get_span();
        (type_var_start, expr_end)
      }
      None => match self.function {
        Some(x) => {
          let (func_start, func_end) = x.get_span();
          (type_var_start, fund_end)
        }
        None => (type_var_start, assignment.end),
      },
    }
  }
}

impl Spanned for CastStatement {
  fn get_span(&self) -> (usize, usize) {
    let (func_start, func_end) = self.function.get_span();
    (self.cast_token.start, func_end)
  }
}
