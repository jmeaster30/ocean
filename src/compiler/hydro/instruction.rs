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
  Primary(HydroToken),
}

#[derive(Clone, Debug)]
pub struct Operation {
  identifier: HydroToken,
  arguments: Vec<HydroToken>,
}

impl Operation {
  pub fn new(identifier: HydroToken, arguments: Vec<HydroToken>) -> Self {
    Self {
      identifier,
      arguments,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Assignment {
  identifier: HydroToken,
  equal: HydroToken,
  operation: OperationOrPrimary,
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
  condition: OperationOrPrimary,
  true_body: Vec<Instruction>,
  else_body: Vec<Instruction>
}

impl If {
  pub fn new(
    condition: OperationOrPrimary,
    true_body: Vec<Instruction>,
    else_body: Vec<Instruction>
  ) -> Self {
    Self {
      condition,
      true_body,
      else_body
    }
  }
}

#[derive(Clone, Debug)]
pub struct Return {
  value: OperationOrPrimary,
}

impl Return {
    pub fn new(value: OperationOrPrimary) -> Self { Self { value } }
}

#[derive(Clone, Debug)]
pub struct Loop {
  condition: OperationOrPrimary,
  body: Vec<Instruction>
}

impl Loop {
  pub fn new(
    condition: OperationOrPrimary,
    body: Vec<Instruction>,
  ) -> Self {
    Self {
      condition,
      body
    }
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  func_token: HydroToken,
  identifier: HydroToken,
  parameter_list: Vec<TypeVar>,
  return_type: Option<Type>,
  body: Vec<Instruction>,
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
  type_token: HydroToken,
  identifier: HydroToken,
  left_curly: HydroToken,
  entries: Vec<TypeVar>,
  right_curly: HydroToken,
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
  identifier: HydroToken,
  type_def: Type,
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
  base_type: Box<Type>,
  left_square: HydroToken,
  index_type: Box<Type>,
  right_square: HydroToken,
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
  token: HydroToken,
}

impl BaseType {
  pub fn new(token: HydroToken) -> Self {
    Self { token }
  }
}

#[derive(Clone, Debug)]
pub struct RefType {
  ref_token: HydroToken,
  base_type: Box<Type>,
}

impl RefType {
  pub fn new(ref_token: HydroToken, base_type: Box<Type>) -> Self {
    Self {
      ref_token,
      base_type,
    }
  }
}
