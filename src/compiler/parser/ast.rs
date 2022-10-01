use crate::compiler::errors::Severity;
use crate::compiler::lexer::Token;

#[derive(Clone)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new(statements: Vec<Statement>) -> Self {
    Self { statements }
  }
}

#[derive(Clone)]
pub enum Statement {
  Error(ErrorStatement),
  Macro(MacroStatement),
  Continue(ContinueStatement),
  Break(BreakStatement),
  Return(ReturnStatement),
  PackDec(PackDecStatement),
  UnionDec(UnionDecStatement),
  VarDec(VarDecStatement),
  Cast(CastStatement),
  Match(MatchStatement),
  Use(UseStatement),
  If(IfStatement),
  ForLoop(ForLoopStatement),
  WhileLoop(WhileStatement),
  InfiniteLoop(InfiniteLoopStatement),
  Expression(ExpressionStatement),
}

#[derive(Clone)]
pub struct MacroStatement {
  pub type_id: Option<i32>,
  pub token: Token,
}

impl MacroStatement {
  pub fn new(token: Token) -> Self {
    Self { type_id: None, token }
  }
}

#[derive(Clone)]
pub struct ErrorStatement {
  pub type_id: Option<i32>,
  pub message: String,
  pub severity: Severity,
  pub tokens: Vec<Token>,
}

impl ErrorStatement {
  pub fn new(message: String, severity: Severity, tokens: Vec<Token>) -> Self {
    Self {
      type_id: None,
      message,
      severity,
      tokens,
    }
  }
}

#[derive(Clone)]
pub struct ContinueStatement {
  pub token: Token,
}

impl ContinueStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

#[derive(Clone)]
pub struct BreakStatement {
  pub token: Token,
}

impl BreakStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

#[derive(Clone)]
pub struct ReturnStatement {
  pub token: Token,
}

impl ReturnStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

#[derive(Clone)]
pub struct PackDecStatement {
  pub type_id: Option<i32>,
  pub pack_token: Token,
  pub name_token: Token,
  pub open_brace: Token,
  pub pack_declarations: Vec<PackDeclaration>,
  pub close_brace: Token,
}

impl PackDecStatement {
  pub fn new(
    pack_token: Token,
    name_token: Token,
    open_brace: Token,
    pack_declarations: Vec<PackDeclaration>,
    close_brace: Token,
  ) -> Self {
    Self {
      type_id: None,
      pack_token,
      name_token,
      open_brace,
      pack_declarations,
      close_brace,
    }
  }
}

#[derive(Clone)]
pub struct PackDeclaration {
  pub type_id: Option<i32>,
  pub type_var: TypeVar,
  pub assignment: Option<Token>,
  pub expression: Option<Expression>,
}

impl PackDeclaration {
  pub fn new(type_var: TypeVar, assignment: Option<Token>, expression: Option<Expression>) -> Self {
    Self {
      type_id: None,
      type_var,
      assignment,
      expression,
    }
  }
}

#[derive(Clone)]
pub struct UnionDecStatement {
  pub type_id: Option<i32>,
  pub union_token: Token,
  pub name_token: Token,
  pub open_brace: Token,
  pub union_declarations: Vec<UnionDeclaration>,
  pub close_brace: Token,
}

impl UnionDecStatement {
  pub fn new(
    union_token: Token,
    name_token: Token,
    open_brace: Token,
    union_declarations: Vec<UnionDeclaration>,
    close_brace: Token,
  ) -> Self {
    Self {
      type_id: None,
      union_token,
      name_token,
      open_brace,
      union_declarations,
      close_brace,
    }
  }
}

#[derive(Clone)]
pub struct UnionDeclaration {
  pub type_id: Option<i32>,
  pub identifier: Token,
  pub left_paren: Option<Token>,
  pub type_list: Vec<Box<Type>>,
  pub right_paren: Option<Token>,
}

impl UnionDeclaration {
  pub fn new(
    identifier: Token,
    left_paren: Option<Token>,
    type_list: Vec<Box<Type>>,
    right_paren: Option<Token>,
  ) -> Self {
    Self {
      type_id: None,
      identifier,
      left_paren,
      type_list,
      right_paren,
    }
  }
}

#[derive(Clone)]
pub struct VarDecStatement {
  pub type_id: Option<i32>,
  pub let_token: Token,
  pub var: Var,
  pub assignment: Option<Token>,
  pub expression: Option<Expression>,
}

impl VarDecStatement {
  pub fn new(
    let_token: Token,
    var: Var,
    assignment: Option<Token>,
    expression: Option<Expression>,
  ) -> Self {
    Self {
      type_id: None,
      let_token,
      var,
      assignment,
      expression,
    }
  }
}

#[derive(Clone)]
pub struct CastStatement {
  pub type_id: Option<i32>,
  pub cast_token: Token,
  pub function: Expression,
}

impl CastStatement {
  pub fn new(cast_token: Token, function: Expression) -> Self {
    Self {
      type_id: None,
      cast_token,
      function,
    }
  }
}

#[derive(Clone)]
pub struct MatchStatement {
  pub match_token: Token,
  pub match_condition: Expression,
  pub left_curly: Token,
  pub match_entries: Vec<MatchEntry>,
  pub right_curly: Token,
}

impl MatchStatement {
  pub fn new(
    match_token: Token,
    match_condition: Expression,
    left_curly: Token,
    match_entries: Vec<MatchEntry>,
    right_curly: Token,
  ) -> Self {
    Self {
      match_token,
      match_condition,
      left_curly,
      match_entries,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct MatchEntry {
  pub match_expression: Expression,
  pub left_curly: Token,
  pub statement_list: Vec<Statement>,
  pub right_curly: Token,
}

impl MatchEntry {
  pub fn new(
    match_expression: Expression,
    left_curly: Token,
    statement_list: Vec<Statement>,
    right_curly: Token,
  ) -> Self {
    Self {
      match_expression,
      left_curly,
      statement_list,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct IfStatement {
  pub type_id: Option<i32>,
  pub if_token: Token,
  pub condition: Expression,
  pub left_curly: Token,
  pub true_body: Vec<Statement>,
  pub right_curly: Token,
  pub else_token: Option<Token>,
  pub else_left_curly: Option<Token>,
  pub else_body: Vec<Statement>,
  pub else_right_curly: Option<Token>,
}

impl IfStatement {
  pub fn new(
    if_token: Token,
    condition: Expression,
    left_curly: Token,
    true_body: Vec<Statement>,
    right_curly: Token,
    else_token: Option<Token>,
    else_left_curly: Option<Token>,
    else_body: Vec<Statement>,
    else_right_curly: Option<Token>,
  ) -> Self {
    Self {
      type_id: None,
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

#[derive(Clone)]
pub struct UseStatement {
  pub type_id: Option<i32>,
  pub use_token: Token,
  pub id_tokens: Vec<Token>,
  pub as_token: Option<Token>,
  pub alias_token: Option<Token>,
}

impl UseStatement {
  pub fn new(
    use_token: Token,
    id_tokens: Vec<Token>,
    as_token: Option<Token>,
    alias_token: Option<Token>,
  ) -> Self {
    Self {
      type_id: None,
      use_token,
      id_tokens,
      as_token,
      alias_token,
    }
  }
}

#[derive(Clone)]
pub struct ForLoopStatement {
  pub type_id: Option<i32>,
  pub loop_token: Token,
  pub iterator: Token,
  pub in_token: Token,
  pub iterable: Expression,
  pub left_curly: Token,
  pub loop_body: Vec<Statement>,
  pub right_curly: Token,
}

impl ForLoopStatement {
  pub fn new(
    loop_token: Token,
    iterator: Token,
    in_token: Token,
    iterable: Expression,
    left_curly: Token,
    loop_body: Vec<Statement>,
    right_curly: Token,
  ) -> Self {
    Self {
      type_id: None,
      loop_token,
      iterator,
      in_token,
      iterable,
      left_curly,
      loop_body,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct WhileStatement {
  pub type_id: Option<i32>,
  pub loop_token: Token,
  pub condition: Expression,
  pub left_curly: Token,
  pub loop_body: Vec<Statement>,
  pub right_curly: Token,
}

impl WhileStatement {
  pub fn new(
    loop_token: Token,
    condition: Expression,
    left_curly: Token,
    loop_body: Vec<Statement>,
    right_curly: Token,
  ) -> Self {
    Self {
      type_id: None,
      loop_token,
      condition,
      left_curly,
      loop_body,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct InfiniteLoopStatement {
  pub type_id: Option<i32>,
  pub loop_token: Token,
  pub left_curly: Token,
  pub loop_body: Vec<Statement>,
  pub right_curly: Token,
}

impl InfiniteLoopStatement {
  pub fn new(
    loop_token: Token,
    left_curly: Token,
    loop_body: Vec<Statement>,
    right_curly: Token,
  ) -> Self {
    Self {
      type_id: None,
      loop_token,
      left_curly,
      loop_body,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct ExpressionStatement {
  pub type_id: Option<i32>,
  pub expression: Expression,
}

impl ExpressionStatement {
  pub fn new(expression: Expression) -> Self {
    Self { type_id: None, expression }
  }
}

#[derive(Clone)]
pub enum Expression {
  Binary(BinaryExpression),
  Prefix(PrefixExpression),
  Postfix(PostfixExpression),
  Member(MemberAccess),
  ArrayAccess(ArrayAccess),
  Cast(CastExpression),
  Literal(Literal),
  Var(UntypedVar),
  FunctionCall(FunctionCall),
  Error(ErrorExpression),
}

#[derive(Clone)]
pub struct ErrorExpression {
  pub type_id: Option<i32>,
  pub severity: Severity,
  pub message: String,
  pub tokens: Vec<Token>,
}

impl ErrorExpression {
  pub fn new(severity: Severity, message: String, tokens: Vec<Token>) -> Self {
    Self {
      type_id: None,
      severity,
      message,
      tokens,
    }
  }
}

#[derive(Clone)]
pub struct BinaryExpression {
  pub type_id: Option<i32>,
  pub lhs: Box<Expression>,
  pub operator: Token,
  pub rhs: Box<Expression>,
}

impl BinaryExpression {
  pub fn new(lhs: Box<Expression>, operator: Token, rhs: Box<Expression>) -> Self {
    Self { lhs, operator, rhs, type_id: None }
  }
}

#[derive(Clone)]
pub struct PrefixExpression {
  pub type_id: Option<i32>,
  pub operator: Token,
  pub rhs: Box<Expression>,
}

impl PrefixExpression {
  pub fn new(operator: Token, rhs: Box<Expression>) -> Self {
    Self { operator, rhs, type_id: None }
  }
}

#[derive(Clone)]
pub struct PostfixExpression {
  pub type_id: Option<i32>,
  pub lhs: Box<Expression>,
  pub operator: Token,
}

impl PostfixExpression {
  pub fn new(lhs: Box<Expression>, operator: Token) -> Self {
    Self { lhs, operator, type_id: None }
  }
}

#[derive(Clone)]
pub struct MemberAccess {
  pub type_id: Option<i32>,
  pub lhs: Box<Expression>,
  pub dot: Token,
  pub id: Token,
}

impl MemberAccess {
  pub fn new(lhs: Box<Expression>, dot: Token, id: Token) -> Self {
    Self { lhs, dot, id, type_id: None }
  }
}

#[derive(Clone)]
pub struct ArrayAccess {
  pub type_id: Option<i32>,
  pub lhs: Box<Expression>,
  pub left_square: Token,
  pub expr: Box<Expression>,
  pub right_square: Token,
}

impl ArrayAccess {
  pub fn new(
    lhs: Box<Expression>,
    left_square: Token,
    expr: Box<Expression>,
    right_square: Token,
  ) -> Self {
    Self {
      type_id: None,
      lhs,
      left_square,
      expr,
      right_square,
    }
  }
}

#[derive(Clone)]
pub struct CastExpression {
  pub type_id: Option<i32>,
  pub lhs: Box<Expression>,
  pub as_token: Token,
  pub cast_type: Type,
}

impl CastExpression {
  pub fn new(lhs: Box<Expression>, as_token: Token, cast_type: Type) -> Self {
    Self {
      type_id: None,
      lhs,
      as_token,
      cast_type,
    }
  }
}

#[derive(Clone)]
pub struct FunctionCall {
  pub type_id: Option<i32>,
  pub target: Box<Expression>,
  pub left_paren: Token,
  pub arguments: Vec<Box<Expression>>,
  pub right_paren: Token,
}

impl FunctionCall {
  pub fn new(
    target: Box<Expression>,
    left_paren: Token,
    arguments: Vec<Box<Expression>>,
    right_paren: Token,
  ) -> Self {
    Self {
      type_id: None,
      target,
      left_paren,
      arguments,
      right_paren,
    }
  }
}

#[derive(Clone)]
pub enum Literal {
  Boolean(BoolLiteral),
  Number(NumberLiteral),
  String(StringLiteral),
  Array(ArrayLiteral),
  Tuple(Tuple),
  Function(Function),
}

#[derive(Clone)]
pub struct BoolLiteral {
  pub type_id: Option<i32>,
  pub token: Token,
}

impl BoolLiteral {
  pub fn new(token: Token) -> Self {
    Self { type_id: None, token }
  }
}

#[derive(Clone)]
pub struct NumberLiteral {
  pub type_id: Option<i32>,
  pub token: Token,
}

impl NumberLiteral {
  pub fn new(token: Token) -> Self {
    Self { type_id: None, token }
  }
}

#[derive(Clone)]
pub struct StringLiteral {
  pub type_id: Option<i32>,
  pub token: Token,
}

impl StringLiteral {
  pub fn new(token: Token) -> Self {
    Self { type_id: None, token }
  }
}

#[derive(Clone)]
pub struct Function {
  pub type_id: Option<i32>,
  pub func_token: Token,
  pub param_left_paren: Token,
  pub param_list: ParameterList,
  pub param_right_paren: Token,
  pub arrow: Token,
  pub returns_left_paren: Token,
  pub return_list: ReturnList,
  pub return_right_paren: Token,
  pub colon: Option<Token>,
  pub left_curly: Option<Token>,
  pub function_body: Vec<Statement>,
  pub right_curly: Option<Token>,
}

impl Function {
  pub fn new(
    func_token: Token,
    param_left_paren: Token,
    param_list: ParameterList,
    param_right_paren: Token,
    arrow: Token,
    returns_left_paren: Token,
    return_list: ReturnList,
    return_right_paren: Token,
    colon: Option<Token>,
    left_curly: Option<Token>,
    function_body: Vec<Statement>,
    right_curly: Option<Token>,
  ) -> Self {
    Self {
      type_id: None,
      func_token,
      param_left_paren,
      param_list,
      param_right_paren,
      arrow,
      returns_left_paren,
      return_list,
      return_right_paren,
      colon,
      left_curly,
      function_body,
      right_curly,
    }
  }
}

#[derive(Clone)]
pub struct Tuple {
  pub type_id: Option<i32>,
  pub left_paren: Token,
  pub contents: Vec<TupleEntry>,
  pub right_paren: Token,
}

impl Tuple {
  pub fn new(left_paren: Token, contents: Vec<TupleEntry>, right_paren: Token) -> Self {
    Self {
      type_id: None,
      left_paren,
      contents,
      right_paren,
    }
  }
}

#[derive(Clone)]
pub struct TupleEntry {
  pub type_id: Option<i32>,
  pub name: Option<Token>,
  pub colon: Option<Token>,
  pub expression: Expression,
}

impl TupleEntry {
  pub fn new(name: Option<Token>, colon: Option<Token>, expression: Expression) -> Self {
    Self {
      type_id: None,
      name,
      colon,
      expression,
    }
  }
}

#[derive(Clone)]
pub struct ArrayLiteral {
  pub type_id: Option<i32>,
  pub left_square: Token,
  pub args: Vec<Box<Expression>>,
  pub right_square: Token,
}

impl ArrayLiteral {
  pub fn new(left_square: Token, args: Vec<Box<Expression>>, right_square: Token) -> Self {
    Self {
      type_id: None,
      left_square,
      args,
      right_square,
    }
  }
}

#[derive(Clone)]
pub enum Var {
  Typed(TypeVar),
  Untyped(UntypedVar),
}

#[derive(Clone)]
pub struct TypeVar {
  pub type_id: Option<i32>,
  pub var: UntypedVar,
  pub colon: Token,
  pub var_type: Box<Type>,
}

impl TypeVar {
  pub fn new(var: UntypedVar, colon: Token, var_type: Box<Type>) -> Self {
    Self {
      type_id: None,
      var,
      colon,
      var_type,
    }
  }
}

#[derive(Clone)]
pub struct UntypedVar {
  pub type_id: Option<i32>,
  pub id: Token,
}

impl UntypedVar {
  pub fn new(id: Token) -> Self {
    Self { id, type_id: None }
  }
}

#[derive(Clone)]
pub enum Type {
  Auto(AutoType),
  Comp(CompType),
  Sub(SubType),
  Func(FuncType),
  Base(BaseType),
  Lazy(LazyType),
  Ref(RefType),
  Mutable(MutableType),
  Array(ArrayType),
  VarType(VarType),
}

#[derive(Clone)]
pub struct AutoType {
  pub auto_token: Token,
  pub auto_name: Option<Token>,
}

impl AutoType {
  pub fn new(auto_token: Token, auto_name: Option<Token>) -> Self {
    Self {
      auto_token,
      auto_name,
    }
  }
}

#[derive(Clone)]
pub struct CompType {
  pub comp_token: Token,
  pub sub_type: Box<Type>,
}

impl CompType {
  pub fn new(comp_token: Token, sub_type: Box<Type>) -> Self {
    Self {
      comp_token,
      sub_type,
    }
  }
}

#[derive(Clone)]
pub struct SubType {
  pub left_paren: Token,
  pub sub_type: Box<Type>,
  pub right_paren: Token,
}

impl SubType {
  pub fn new(left_paren: Token, sub_type: Box<Type>, right_paren: Token) -> Self {
    Self {
      left_paren,
      sub_type,
      right_paren,
    }
  }
}

#[derive(Clone)]
pub struct FuncType {
  pub func_token: Token,
  pub left_paren: Option<Token>,
  pub param_types: Vec<Box<Type>>,
  pub colon: Option<Token>,
  pub return_types: Vec<Box<Type>>,
  pub right_paren: Option<Token>,
}

impl FuncType {
  pub fn new(
    func_token: Token,
    left_paren: Option<Token>,
    param_types: Vec<Box<Type>>,
    colon: Option<Token>,
    return_types: Vec<Box<Type>>,
    right_paren: Option<Token>,
  ) -> Self {
    Self {
      func_token,
      left_paren,
      param_types,
      colon,
      return_types,
      right_paren,
    }
  }
}

#[derive(Clone)]
pub struct BaseType {
  pub base_token: Token,
}

impl BaseType {
  pub fn new(base_token: Token) -> Self {
    Self { base_token }
  }
}

#[derive(Clone)]
pub struct LazyType {
  pub lazy_token: Token,
  pub sub_type: Box<Type>,
}

impl LazyType {
  pub fn new(lazy_token: Token, sub_type: Box<Type>) -> Self {
    Self {
      lazy_token,
      sub_type,
    }
  }
}

#[derive(Clone)]
pub struct RefType {
  pub ref_token: Token,
  pub sub_type: Box<Type>,
}

impl RefType {
  pub fn new(ref_token: Token, sub_type: Box<Type>) -> Self {
    Self {
      ref_token,
      sub_type,
    }
  }
}

#[derive(Clone)]
pub struct MutableType {
  pub mut_token: Token,
  pub sub_type: Box<Type>,
}

impl MutableType {
  pub fn new(mut_token: Token, sub_type: Box<Type>) -> Self {
    Self {
      mut_token,
      sub_type,
    }
  }
}

#[derive(Clone)]
pub struct ArrayType {
  pub base: Box<Type>,
  pub left_square: Token,
  pub sub_type: Box<Option<Type>>,
  pub right_square: Token,
}

impl ArrayType {
  pub fn new(
    base: Box<Type>,
    left_square: Token,
    sub_type: Box<Option<Type>>,
    right_square: Token,
  ) -> Self {
    Self {
      base,
      left_square,
      sub_type,
      right_square,
    }
  }
}

#[derive(Clone)]
pub struct VarType {
  pub base: Box<Type>,
  pub triple_dot: Token,
}

impl VarType {
  pub fn new(base: Box<Type>, triple_dot: Token) -> Self {
    Self { base, triple_dot }
  }
}

#[derive(Clone)]
pub struct ParameterList {
  pub type_id: Option<i32>,
  pub params: Vec<Parameter>,
}

impl ParameterList {
  pub fn new(params: Vec<Parameter>) -> Self {
    Self { params, type_id: None }
  }
}

#[derive(Clone)]
pub struct Parameter {
  pub type_id: Option<i32>,
  pub type_var: TypeVar,
}

impl Parameter {
  pub fn new(type_var: TypeVar) -> Self {
    Self { type_var, type_id: None }
  }
}

#[derive(Clone)]
pub struct ReturnList {
  pub type_id: Option<i32>,
  pub returns: Vec<ReturnEntry>,
}

impl ReturnList {
  pub fn new(returns: Vec<ReturnEntry>) -> Self {
    Self { returns, type_id: None }
  }
}

#[derive(Clone)]
pub struct ReturnEntry {
  pub type_id: Option<i32>,
  pub type_var: TypeVar,
  pub assignment: Option<Token>,
  pub expression: Option<Box<Expression>>,
}

impl ReturnEntry {
  pub fn new(
    type_var: TypeVar,
    assignment: Option<Token>,
    expression: Option<Box<Expression>>,
  ) -> Self {
    Self {
      type_var,
      assignment,
      expression,
      type_id: None
    }
  }
}
