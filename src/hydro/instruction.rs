use super::value::Value;
use crate::hydro::function::Target;
use crate::hydro::value::Type;

#[derive(Debug, Clone)]
pub enum Instruction {
  PushValue(PushValue),
  PopValue(PopValue),
  Duplicate(Duplicate),
  Swap(Swap),
  Rotate(Rotate),

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
  Cast(Cast),

  Load(Load),
  Store(Store),
  GetArrayIndex(GetArrayIndex),
  SetArrayIndex(SetArrayIndex),
  GetLayoutIndex(GetLayoutIndex),
  SetLayoutIndex(SetLayoutIndex),
  Allocate(Allocate),
  AllocateArray(AllocateArray),
}

#[derive(Debug, Clone)]
pub struct PushValue {
  pub value: Value,
}

#[derive(Debug, Clone)]
pub struct PopValue {}

#[derive(Debug, Clone)]
pub struct Duplicate {
  pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct Swap {}

#[derive(Debug, Clone)]
pub struct Rotate {
  pub size: usize,
}

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
  pub target: Target,
}

#[derive(Debug, Clone)]
pub struct Branch {
  pub true_target: Target,
  pub false_target: Target,
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
pub struct SetArrayIndex {}

#[derive(Debug, Clone)]
pub struct GetArrayIndex {}

#[derive(Debug, Clone)]
pub struct SetLayoutIndex {
  pub member: String,
}

#[derive(Debug, Clone)]
pub struct GetLayoutIndex {
  pub member: String,
}

#[derive(Debug, Clone)]
pub struct Cast {
  pub to_type: Type,
}

#[derive(Debug, Clone)]
pub struct Allocate {
  pub allocated_type: Type,
}

#[derive(Debug, Clone)]
pub struct AllocateArray {
  pub array_size: Option<u64>,
  pub array_sub_type: Type,
}
