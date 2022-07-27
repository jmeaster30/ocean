use super::lexer::HydroToken;

#[derive(Clone, Debug)]
pub enum Instruction {
  Operation(Operation),
  Assignment(Assignment),
  If(If),
  Loop(Loop),
  Function(Function),
  TypeDefinition(TypeDefinition),
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
  if_token: HydroToken,
  condition: Operation,
  left_curly: HydroToken,
  true_body: Vec<Instruction>,
  right_curly: HydroToken,
  else_token: Option<HydroToken>,
  else_left_curly: Option<HydroToken>,
  else_body: Vec<Instruction>,
  else_right_curly: Option<HydroToken>,
}

impl If {
  pub fn new(
    if_token: HydroToken,
    condition: Operation,
    left_curly: HydroToken,
    true_body: Vec<Instruction>,
    right_curly: HydroToken,
    else_token: Option<HydroToken>,
    else_left_curly: Option<HydroToken>,
    else_body: Vec<Instruction>,
    else_right_curly: Option<HydroToken>,
  ) -> Self {
    Self {
      if_token,
      condition,
      left_curly,
      true_body,
      right_curly,
      else_token,
      else_left_curly,
      else_body,
      else_right_curly,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Loop {
  while_token: HydroToken,
  condition: Operation,
  left_curly: HydroToken,
  body: Vec<Instruction>,
  right_curly: HydroToken,
}

impl Loop {
  pub fn new(
    while_token: HydroToken,
    condition: Operation,
    left_curly: HydroToken,
    body: Vec<Instruction>,
    right_curly: HydroToken,
  ) -> Self {
    Self {
      while_token,
      condition,
      left_curly,
      body,
      right_curly,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  func_token: HydroToken,
  identifier: HydroToken,
  parameter_list: Option<ParameterList>,
  return_type: Option<HydroToken>,
  left_curly: HydroToken,
  body: Vec<Instruction>,
  right_curly: HydroToken,
}

impl Function {
  pub fn new(
    func_token: HydroToken,
    identifier: HydroToken,
    parameter_list: Option<ParameterList>,
    return_type: Option<HydroToken>,
    left_curly: HydroToken,
    body: Vec<Instruction>,
    right_curly: HydroToken,
  ) -> Self {
    Self {
      func_token,
      identifier,
      parameter_list,
      return_type,
      left_curly,
      body,
      right_curly,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ParameterList {
  left_paren: HydroToken,
  params: Vec<TypeVar>,
  right_paren: HydroToken,
}

impl ParameterList {
  pub fn new(left_paren: HydroToken, params: Vec<TypeVar>, right_paren: HydroToken) -> Self {
    Self {
      left_paren,
      params,
      right_paren,
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
  colon: HydroToken,
  type_def: Type,
}

impl TypeVar {
  pub fn new(identifier: HydroToken, colon: HydroToken, type_def: Type) -> Self {
    Self {
      identifier,
      colon,
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
