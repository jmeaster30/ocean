use crate::util::span::Spanned;

use super::instruction::{
  ArrayType, Assignment, BaseType, Function, If, Instruction, Loop, Operation, OperationOrPrimary,
  Primary, RefType, Return, Type, TypeDefinition,
};

impl Spanned for Instruction {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Instruction::Operation(x) => x.get_span(),
      Instruction::Assignment(x) => x.get_span(),
      Instruction::If(x) => x.get_span(),
      Instruction::Loop(x) => x.get_span(),
      Instruction::Function(x) => x.get_span(),
      Instruction::TypeDefinition(x) => x.get_span(),
      Instruction::Return(x) => x.get_span(),
      Instruction::Break(token) => (token.start, token.end),
      Instruction::Continue(token) => (token.start, token.end),
    }
  }
}

impl Spanned for Operation {
  fn get_span(&self) -> (usize, usize) {
    let end = if self.arguments.len() == 0 {
      self.identifier.end
    } else {
      let (_, aend) = self.arguments[self.arguments.len() - 1].get_span();
      aend
    };

    (self.identifier.start, end)
  }
}

impl Spanned for Primary {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for Assignment {
  fn get_span(&self) -> (usize, usize) {
    let (_val_start, val_end) = self.operation.get_span();
    (self.identifier.start, val_end)
  }
}

impl Spanned for If {
  fn get_span(&self) -> (usize, usize) {
    let end = if self.else_body.len() == 0 && self.true_body.len() == 0 {
      let (_, c_end) = self.condition.get_span();
      c_end
    } else if self.else_body.len() == 0 {
      let (_, tb_end) = self.true_body[self.true_body.len() - 1].get_span();
      tb_end
    } else {
      let (_, eb_end) = self.else_body[self.else_body.len() - 1].get_span();
      eb_end
    };

    (self.if_token.start, end)
  }
}

impl Spanned for Loop {
  fn get_span(&self) -> (usize, usize) {
    let end = if self.body.len() == 0 {
      self.loop_token.end
    } else {
      let (_, loop_end) = self.body[self.body.len() - 1].get_span();
      loop_end
    };
    (self.loop_token.end, end)
  }
}

impl Spanned for Function {
  fn get_span(&self) -> (usize, usize) {
    // TODO make this a little cleaner maybe but I think it is tolerable
    (
      self.func_token.start,
      self.body[self.body.len() - 1].get_span().1,
    )
  }
}

impl Spanned for TypeDefinition {
  fn get_span(&self) -> (usize, usize) {
    // TODO
    (0, 0)
  }
}

impl Spanned for OperationOrPrimary {
  fn get_span(&self) -> (usize, usize) {
    match self {
      OperationOrPrimary::Operation(x) => x.get_span(),
      OperationOrPrimary::Primary(x) => x.get_span(),
    }
  }
}

impl Spanned for Return {
  fn get_span(&self) -> (usize, usize) {
    let (_val_start, val_end) = self.value.get_span();
    (self.return_token.start, val_end)
  }
}

impl Spanned for Type {
  fn get_span(&self) -> (usize, usize) {
    match self {
      Type::ArrayType(x) => x.get_span(),
      Type::BaseType(x) => x.get_span(),
      Type::RefType(x) => x.get_span(),
    }
  }
}

impl Spanned for ArrayType {
  fn get_span(&self) -> (usize, usize) {
    let (start, _) = self.base_type.get_span();
    (start, self.right_square.end)
  }
}

impl Spanned for BaseType {
  fn get_span(&self) -> (usize, usize) {
    (self.token.start, self.token.end)
  }
}

impl Spanned for RefType {
  fn get_span(&self) -> (usize, usize) {
    let (_, end) = self.base_type.get_span();
    (self.ref_token.start, end)
  }
}
