use super::value::Value;

#[derive(Debug, Clone)]
pub enum Instruction {
  PushValue(PushValue),
  PopValue(PopValue),

  Add(Add),
  Subtract(Subtract),
  Multiply(Multiply),
  Divide(Divide),
  Modulo(Modulo),

  LeftShift(LeftShift),
  RightShift(RightShift),

  BitwiseAnd(BitwiseAnd),
  BitwiseOr(BitwiseOr),
  BitwiseXor(BitwiseXor),
  BitwiseNot(BitwiseNot),

  And(And),
  Or(Or),
  Xor(Xor),
  Not(Not),

  Equal(Equal),
  NotEqual(NotEqual),
  LessThan(LessThan),
  GreaterThan(GreaterThan),
  LessThanEqual(LessThanEqual),
  GreaterThanEqual(GreaterThanEqual),

  Jump(Jump),
  Branch(Branch),

  Call(Call),
  Return(Return),

  Load(Load),
  Store(Store),
  Index(Index),
  AllocArray(AllocArray),
  AllocLayout(AllocLayout),
}

#[derive(Debug, Clone)]
pub struct PushValue {
  pub value: Value,
}

#[derive(Debug, Clone)]
pub struct PopValue {}

#[derive(Debug, Clone)]
pub struct Add {}

#[derive(Debug, Clone)]
pub struct Subtract {}

#[derive(Debug, Clone)]
pub struct Multiply {}

#[derive(Debug, Clone)]
pub struct Divide {}

#[derive(Debug, Clone)]
pub struct Modulo {}

#[derive(Debug, Clone)]
pub struct LeftShift {}

#[derive(Debug, Clone)]
pub struct RightShift {}

#[derive(Debug, Clone)]
pub struct BitwiseAnd {}

#[derive(Debug, Clone)]
pub struct BitwiseOr {}

#[derive(Debug, Clone)]
pub struct BitwiseXor {}

#[derive(Debug, Clone)]
pub struct BitwiseNot {}

#[derive(Debug, Clone)]
pub struct And {}

#[derive(Debug, Clone)]
pub struct Or {}

#[derive(Debug, Clone)]
pub struct Xor {}

#[derive(Debug, Clone)]
pub struct Not {}

#[derive(Debug, Clone)]
pub struct Equal {}

#[derive(Debug, Clone)]
pub struct NotEqual {}

#[derive(Debug, Clone)]
pub struct LessThan {}

#[derive(Debug, Clone)]
pub struct GreaterThan {}

#[derive(Debug, Clone)]
pub struct LessThanEqual {}

#[derive(Debug, Clone)]
pub struct GreaterThanEqual {}

#[derive(Debug, Clone)]
pub struct Jump {
  pub index: usize,
}

#[derive(Debug, Clone)]
pub struct Branch {
  pub true_index: usize,
  pub false_index: usize,
}

#[derive(Debug, Clone)]
pub struct Call {}

#[derive(Debug, Clone)]
pub struct Return {}

#[derive(Debug, Clone)]
pub struct Load {}

#[derive(Debug, Clone)]
pub struct Store {}

#[derive(Debug, Clone)]
pub struct Index {}

#[derive(Debug, Clone)]
pub struct AllocArray {
  // TODO There should be a better interface for this instruction.
  pub length: usize,
  pub default_value: Value,
}

#[derive(Debug, Clone)]
pub struct AllocLayout {
  pub module_name: Option<String>,
  pub layout_template_name: String,
}
