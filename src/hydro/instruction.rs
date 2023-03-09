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
  Break(HydroToken),
  Continue(HydroToken),
}

#[derive(Clone, Debug)]
pub enum OperationOrPrimary {
  Operation(Operation),
  Primary(Primary),
  New(New),
}

#[derive(Clone, Debug)]
pub enum Primary {
  Var(Var),
  Access(Access),
}

#[derive(Clone, Debug)]
pub struct Var {
  pub token: HydroToken,
}

impl Var {
  pub fn new(token: HydroToken) -> Self {
    Self { token }
  }
}

#[derive(Clone, Debug)]
pub struct Access {
  pub primary: Box<Primary>,
  pub identifier: Option<HydroToken>,
  pub index: Option<Box<Primary>>,
}

impl Access {
  pub fn new(
    primary: Box<Primary>,
    identifier: Option<HydroToken>,
    index: Option<Box<Primary>>,
  ) -> Self {
    Self {
      primary,
      identifier,
      index,
    }
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
pub struct New {
  pub token: HydroToken,
  pub new_type: Type,
  pub arguments: Vec<Primary>,
}

impl New {
  pub fn new(token: HydroToken, new_type: Type, arguments: Vec<Primary>) -> Self {
    Self {
      token,
      new_type,
      arguments,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Assignment {
  pub primary: Primary,
  pub equal: HydroToken,
  pub operation: OperationOrPrimary,
}

impl Assignment {
  pub fn new(primary: Primary, equal: HydroToken, operation: OperationOrPrimary) -> Self {
    Self {
      primary,
      equal,
      operation,
    }
  }
}

#[derive(Clone, Debug)]
pub struct If {
  pub if_token: HydroToken,
  pub condition: OperationOrPrimary,
  pub true_body: Vec<Instruction>,
  pub else_body: Vec<Instruction>,
}

impl If {
  pub fn new(
    if_token: HydroToken,
    condition: OperationOrPrimary,
    true_body: Vec<Instruction>,
    else_body: Vec<Instruction>,
  ) -> Self {
    Self {
      if_token,
      condition,
      true_body,
      else_body,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Return {
  pub return_token: HydroToken,
  pub value: OperationOrPrimary,
}

impl Return {
  pub fn new(return_token: HydroToken, value: OperationOrPrimary) -> Self {
    Self {
      return_token,
      value,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Loop {
  pub loop_token: HydroToken,
  pub body: Vec<Instruction>,
}

impl Loop {
  pub fn new(loop_token: HydroToken, body: Vec<Instruction>) -> Self {
    Self { loop_token, body }
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
