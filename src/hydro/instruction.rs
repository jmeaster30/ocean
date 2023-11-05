use super::value::Value;
use crate::hydro::function::Target;
use crate::hydro::value::Type;
use ocean_helpers::Debuggable;
// Intellij thinks these are unused but they are used by the Debuggable derive macro
use crate::hydro::debugcontext::DebugContext;
use crate::hydro::debuggable::Debuggable;
use crate::hydro::exception::Exception;
use crate::hydro::executable::Executable;
use crate::hydro::executioncontext::ExecutionContext;
use crate::hydro::module::Module;

#[derive(Debug, Clone)]
pub enum Instruction {
  PushValue(Push),
  PopValue(Pop),
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

#[derive(Debug, Clone, Debuggable)]
pub struct Push {
  pub value: Value,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Pop {}

#[derive(Debug, Clone, Debuggable)]
pub struct Duplicate {
  pub offset: usize,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Swap {}

#[derive(Debug, Clone, Debuggable)]
pub struct Rotate {
  pub size: i64,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Add {}

#[derive(Debug, Clone, Debuggable)]
pub struct Subtract {}

#[derive(Debug, Clone, Debuggable)]
pub struct Multiply {}

#[derive(Debug, Clone, Debuggable)]
pub struct Divide {}

#[derive(Debug, Clone, Debuggable)]
pub struct Modulo {}

#[derive(Debug, Clone, Debuggable)]
pub struct LeftShift {}

#[derive(Debug, Clone, Debuggable)]
pub struct RightShift {}

#[derive(Debug, Clone, Debuggable)]
pub struct BitwiseAnd {}

#[derive(Debug, Clone, Debuggable)]
pub struct BitwiseOr {}

#[derive(Debug, Clone, Debuggable)]
pub struct BitwiseXor {}

#[derive(Debug, Clone, Debuggable)]
pub struct BitwiseNot {}

#[derive(Debug, Clone, Debuggable)]
pub struct And {}

#[derive(Debug, Clone, Debuggable)]
pub struct Or {}

#[derive(Debug, Clone, Debuggable)]
pub struct Xor {}

#[derive(Debug, Clone, Debuggable)]
pub struct Not {}

#[derive(Debug, Clone, Debuggable)]
pub struct Equal {}

#[derive(Debug, Clone, Debuggable)]
pub struct NotEqual {}

#[derive(Debug, Clone, Debuggable)]
pub struct LessThan {}

#[derive(Debug, Clone, Debuggable)]
pub struct GreaterThan {}

#[derive(Debug, Clone, Debuggable)]
pub struct LessThanEqual {}

#[derive(Debug, Clone, Debuggable)]
pub struct GreaterThanEqual {}

#[derive(Debug, Clone, Debuggable)]
pub struct Jump {
  pub target: Target,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Branch {
  pub true_target: Target,
  pub false_target: Target,
}

#[derive(Debug, Clone)]
pub struct Call {}

#[derive(Debug, Clone, Debuggable)]
pub struct Return {}

#[derive(Debug, Clone, Debuggable)]
pub struct Load {}

#[derive(Debug, Clone, Debuggable)]
pub struct Store {}

#[derive(Debug, Clone, Debuggable)]
pub struct SetArrayIndex {}

#[derive(Debug, Clone, Debuggable)]
pub struct GetArrayIndex {}

#[derive(Debug, Clone, Debuggable)]
pub struct SetLayoutIndex {
  pub member: String,
}

#[derive(Debug, Clone, Debuggable)]
pub struct GetLayoutIndex {
  pub member: String,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Cast {
  pub to_type: Type,
}

#[derive(Debug, Clone, Debuggable)]
pub struct Allocate {
  pub allocated_type: Type,
}

#[derive(Debug, Clone, Debuggable)]
pub struct AllocateArray {
  pub array_size: Option<u64>,
  pub array_sub_type: Type,
}
