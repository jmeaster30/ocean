use super::lexer::HydroToken;

#[derive(Clone, Debug)]
pub enum Instruction {
  Operation(Operation),
  Assignment(Assignment),
  If(If),
  Loop(Loop),
  Function(Function),
  TypeDefinition(TypeDefinition),
  Return(Return),
  Break,
  Continue,
}

#[derive(Clone, Debug)]
pub enum OperationOrPrimary {
  Operation(Operation),
  Primary(Primary),
}

#[derive(Clone, Debug)]
pub struct Primary {
  pub token: HydroToken,
}

impl Primary {
  pub fn new(token: HydroToken) -> Self {
    Self { token }
  }
}

#[derive(Clone, Debug)]
pub struct Operation {
  pub identifier: HydroToken,
  pub arguments: Vec<Primary>,
}

impl Operation {
  pub fn new(identifier: HydroToken, arguments: Vec<Primary>) -> Self {
    Self {
      identifier,
      arguments,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Assignment {
  pub identifier: HydroToken,
  pub equal: HydroToken,
  pub operation: OperationOrPrimary,
}

impl Assignment {
  pub fn new(identifier: HydroToken, equal: HydroToken, operation: OperationOrPrimary) -> Self {
    Self {
      identifier,
      equal,
      operation,
    }
  }
}

#[derive(Clone, Debug)]
pub struct If {
  pub condition: OperationOrPrimary,
  pub true_body: Vec<Instruction>,
  pub else_body: Vec<Instruction>,
}

impl If {
  pub fn new(
    condition: OperationOrPrimary,
    true_body: Vec<Instruction>,
    else_body: Vec<Instruction>,
  ) -> Self {
    Self {
      condition,
      true_body,
      else_body,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Return {
  pub value: OperationOrPrimary,
}

impl Return {
  pub fn new(value: OperationOrPrimary) -> Self {
    Self { value }
  }
}

#[derive(Clone, Debug)]
pub struct Loop {
  pub body: Vec<Instruction>,
}

impl Loop {
  pub fn new(body: Vec<Instruction>) -> Self {
    Self { body }
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  pub func_token: HydroToken,
  pub identifier: HydroToken,
  pub parameter_list: Vec<TypeVar>,
  pub return_type: Option<Type>,
  pub body: Vec<Instruction>,
}

impl Function {
  pub fn new(
    func_token: HydroToken,
    identifier: HydroToken,
    parameter_list: Vec<TypeVar>,
    return_type: Option<Type>,
    body: Vec<Instruction>,
  ) -> Self {
    Self {
      func_token,
      identifier,
      parameter_list,
      return_type,
      body,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypeDefinition {
  pub type_token: HydroToken,
  pub identifier: HydroToken,
  pub left_curly: HydroToken,
  pub entries: Vec<TypeVar>,
  pub right_curly: HydroToken,
}

impl TypeDefinition {
  pub fn new(
    type_token: HydroToken,
    identifier: HydroToken,
    left_curly: HydroToken,
    entries: Vec<TypeVar>,
    right_curly: HydroToken,
  ) -> Self {
    Self {
      type_token,
      identifier,
      left_curly,
      entries,
      right_curly,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TypeVar {
  pub identifier: HydroToken,
  pub type_def: Type,
}

impl TypeVar {
  pub fn new(identifier: HydroToken, type_def: Type) -> Self {
    Self {
      identifier,
      type_def,
    }
  }
}

#[derive(Clone, Debug)]
pub enum Type {
  ArrayType(ArrayType),
  BaseType(BaseType),
  RefType(RefType),
}

#[derive(Clone, Debug)]
pub struct ArrayType {
  pub base_type: Box<Type>,
  pub left_square: HydroToken,
  pub index_type: Box<Type>,
  pub right_square: HydroToken,
}

impl ArrayType {
  pub fn new(
    base_type: Box<Type>,
    left_square: HydroToken,
    index_type: Box<Type>,
    right_square: HydroToken,
  ) -> Self {
    Self {
      base_type,
      left_square,
      index_type,
      right_square,
    }
  }
}

#[derive(Clone, Debug)]
pub struct BaseType {
  pub token: HydroToken,
}

impl BaseType {
  pub fn new(token: HydroToken) -> Self {
    Self { token }
  }
}

#[derive(Clone, Debug)]
pub struct RefType {
  pub ref_token: HydroToken,
  pub base_type: Box<Type>,
}

impl RefType {
  pub fn new(ref_token: HydroToken, base_type: Box<Type>) -> Self {
    Self {
      ref_token,
      base_type,
    }
  }
}
