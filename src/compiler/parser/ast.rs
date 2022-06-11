use crate::compiler::errors::Severity;
use crate::compiler::lexer::Token;

pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn new(statements: Vec<Statement>) -> Self {
    Self { statements }
  }
}

pub enum Statement {
  Error(ErrorStatement),
  Macro(MacroStatement),
  Continue(ContinueStatement),
  Break(BreakStatement),
  Return(ReturnStatement),
  PackDec(PackDecStatement),
  EnumDec(EnumDecStatement),
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

pub struct MacroStatement {
  pub token: Token,
}

impl MacroStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

pub struct ErrorStatement {
  pub message: String,
  pub severity: Severity,
  pub tokens: Vec<Token>,
}

impl ErrorStatement {
  pub fn new(message: String, severity: Severity, tokens: Vec<Token>) -> Self {
    Self {
      message,
      severity,
      tokens,
    }
  }
}

pub struct ContinueStatement {
  pub token: Token,
}

impl ContinueStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

pub struct BreakStatement {
  pub token: Token,
}

impl BreakStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

pub struct ReturnStatement {
  pub token: Token,
}

impl ReturnStatement {
  pub fn new(token: Token) -> Self {
    Self { token }
  }
}

pub struct PackDecStatement {
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
      pack_token,
      name_token,
      open_brace,
      pack_declarations,
      close_brace,
    }
  }
}

pub struct PackDeclaration {
  pub type_var: TypeVar,
  pub assignment: Option<Token>,
  pub expression: Option<Expression>,
}

impl PackDeclaration {
  pub fn new(type_var: TypeVar, assignment: Option<Token>, expression: Option<Expression>) -> Self {
    Self {
      type_var,
      assignment,
      expression,
    }
  }
}

pub struct EnumDecStatement {
  pub enum_token: Token,
  pub name_token: Token,
  pub open_brace: Token,
  pub enum_declarations: Vec<EnumDeclaration>,
  pub close_brace: Token,
}

impl EnumDecStatement {
  pub fn new(
    enum_token: Token,
    name_token: Token,
    open_brace: Token,
    enum_declarations: Vec<EnumDeclaration>,
    close_brace: Token,
  ) -> Self {
    Self {
      enum_token,
      name_token,
      open_brace,
      enum_declarations,
      close_brace,
    }
  }
}

pub struct EnumDeclaration {
  pub identifier: Token,
  pub enum_storage: Option<EnumStorage>,
}

impl EnumDeclaration {
  pub fn new(identifier: Token, enum_storage: Option<EnumStorage>) -> Self {
    Self {
      identifier,
      enum_storage,
    }
  }
}

pub struct EnumStorage {
  left_paren: Token,
  type_list: Vec<(Type, Option<Token>)>,
  right_paren: Token,
}

impl EnumStorage {
  pub fn new(left_paren: Token, type_list: Vec<(Type, Option<Token>)>, right_paren: Token) -> Self {
    Self {
      left_paren,
      type_list,
      right_paren,
    }
  }
}

pub struct VarDecStatement {
  pub type_var: TypeVar,
  pub assignment: Token,
  pub expression: Option<Expression>,
  pub function: Option<Function>,
}

impl VarDecStatement {
  pub fn new(
    type_var: TypeVar,
    assignment: Token,
    expression: Option<Expression>,
    function: Option<Function>,
  ) -> Self {
    Self {
      type_var,
      assignment,
      expression,
      function,
    }
  }
}

pub struct CastStatement {
  pub cast_token: Token,
  pub function: Function,
}

impl CastStatement {
  pub fn new(cast_token: Token, function: Function) -> Self {
    Self {
      cast_token,
      function,
    }
  }
}

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

pub struct IfStatement {
  pub if_token: Token,
  pub condition: Expression,
  pub left_curly: Token,
  pub true_body: Vec<Statement>,
  pub else_token: Option<Token>,
  pub else_body: Vec<Statement>,
}

impl IfStatement {
  pub fn new(
    if_token: Token,
    condition: Expression,
    left_curly: Token,
    true_body: Vec<Statement>,
    else_token: Option<Token>,
    else_body: Vec<Statement>,
  ) -> Self {
    Self {
      if_token,
      condition,
      left_curly,
      true_body,
      else_token,
      else_body,
    }
  }
}

pub struct UseStatement {
  pub use_token: Token,
  pub id_tokens: Vec<Token>,
}

impl UseStatement {
  pub fn new(use_token: Token, id_tokens: Vec<Token>) -> Self {
    Self {
      use_token,
      id_tokens,
    }
  }
}

pub struct ForLoopStatement {
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

pub struct WhileStatement {
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
      loop_token,
      condition,
      left_curly,
      loop_body,
      right_curly,
    }
  }
}

pub struct InfiniteLoopStatement {
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
      loop_token,
      left_curly,
      loop_body,
      right_curly,
    }
  }
}

pub struct ExpressionStatement {
  pub expression: Expression,
}

impl ExpressionStatement {
  pub fn new(expression: Expression) -> Self {
    Self { expression }
  }
}

pub struct Function {
  pub param_left_paren: Token,
  pub param_list: ParameterList,
  pub param_right_paren: Token,
  pub arrow: Token,
  pub returns_left_paren: Token,
  pub return_list: ReturnList,
  pub return_right_paren: Token,
  pub left_curly: Option<Token>,
  pub function_body: Vec<Statement>,
  pub right_curly: Option<Token>,
}

impl Function {
  pub fn new(
    param_left_paren: Token,
    param_list: ParameterList,
    param_right_paren: Token,
    arrow: Token,
    returns_left_paren: Token,
    return_list: ReturnList,
    return_right_paren: Token,
    left_curly: Option<Token>,
    function_body: Vec<Statement>,
    right_curly: Option<Token>,
  ) -> Self {
    Self {
      param_left_paren,
      param_list,
      param_right_paren,
      arrow,
      returns_left_paren,
      return_list,
      return_right_paren,
      left_curly,
      function_body,
      right_curly,
    }
  }
}

pub enum Expression {
  Binary(BinaryExpression),
  Prefix(PrefixExpression),
  Postfix(PostfixExpression),
  Member(MemberAccess),
  ArrayAccess(ArrayAccess),
  Cast(CastExpression),
  Literal(Literal),
  Var(Var),
  FunctionCall(FunctionCall),
}

pub struct BinaryExpression {
  pub lhs: Box<Expression>,
  pub operator: Token,
  pub rhs: Box<Expression>,
}

impl BinaryExpression {
  pub fn new(lhs: Box<Expression>, operator: Token, rhs: Box<Expression>) -> Self {
    Self { lhs, operator, rhs }
  }
}

pub struct PrefixExpression {
  pub operator: Token,
  pub rhs: Box<Expression>,
}

impl PrefixExpression {
  pub fn new(operator: Token, rhs: Box<Expression>) -> Self {
    Self { operator, rhs }
  }
}

pub struct PostfixExpression {
  pub lhs: Box<Expression>,
  pub operator: Token,
}

impl PostfixExpression {
  pub fn new(lhs: Box<Expression>, operator: Token) -> Self {
    Self { lhs, operator }
  }
}

pub struct MemberAccess {
  pub lhs: Box<Expression>,
  pub dot: Token,
  pub id: Token,
}

impl MemberAccess {
  pub fn new(lhs: Box<Expression>, dot: Token, id: Token) -> Self {
    Self { lhs, dot, id }
  }
}

pub struct ArrayAccess {
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
      lhs,
      left_square,
      expr,
      right_square,
    }
  }
}

pub struct CastExpression {
  pub lhs: Box<Expression>,
  pub as_token: Token,
  pub cast_type: Type,
}

impl CastExpression {
  pub fn new(lhs: Box<Expression>, as_token: Token, cast_type: Type) -> Self {
    Self {
      lhs,
      as_token,
      cast_type,
    }
  }
}

pub struct FunctionCall {
  pub target: Box<Expression>,
  pub left_paren: Token,
  pub arguments: Vec<(Expression, Option<Token>)>,
  pub right_paren: Token,
}

impl FunctionCall {
  pub fn new(
    target: Box<Expression>,
    left_paren: Token,
    arguments: Vec<(Expression, Option<Token>)>,
    right_paren: Token,
  ) -> Self {
    Self {
      target,
      left_paren,
      arguments,
      right_paren,
    }
  }
}

pub enum Literal {
  Boolean(Token),
  Number(Token),
  String(Token),
  Array(ArrayLiteral),
}

pub struct ArrayLiteral {
  pub left_square: Token,
  pub args: Vec<(Box<Expression>, Option<Token>)>,
  pub right_square: Token,
}

impl ArrayLiteral {
  pub fn new(
    left_square: Token,
    args: Vec<(Box<Expression>, Option<Token>)>,
    right_square: Token,
  ) -> Self {
    Self {
      left_square,
      args,
      right_square,
    }
  }
}

pub struct TypeVar {
  pub var: Var,
  pub colon: Token,
  pub var_type: Box<Type>,
}

impl TypeVar {
  pub fn new(var: Var, colon: Token, var_type: Box<Type>) -> Self {
    Self {
      var,
      colon,
      var_type,
    }
  }
}

pub struct Var {
  pub id: Token,
}

impl Var {
  pub fn new(id: Token) -> Self {
    Self { id }
  }
}

pub enum Type {
  Auto(AutoType),
  Comp(CompType),
  Sub(SubType),
  Func(FuncType),
  Base(BaseType),
  Lazy(LazyType),
  Ref(RefType),
  Optional(OptionalType),
}

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

pub struct FuncType {
  pub func_token: Token,
  pub left_paren: Option<Token>,
  pub param_types: Vec<(Box<Type>, Option<Token>)>,
  pub colon: Option<Token>,
  pub return_types: Vec<(Box<Type>, Option<Token>)>,
  pub right_paren: Option<Token>,
}

impl FuncType {
  pub fn new(
    func_token: Token,
    left_paren: Option<Token>,
    param_types: Vec<(Box<Type>, Option<Token>)>,
    colon: Option<Token>,
    return_types: Vec<(Box<Type>, Option<Token>)>,
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

pub struct BaseType {
  pub base_token: Token,
}

impl BaseType {
  pub fn new(base_token: Token) -> Self {
    Self { base_token }
  }
}

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

pub struct OptionalType {
  pub optional_token: Token,
  pub sub_type: Box<Type>,
}

impl OptionalType {
  pub fn new(optional_token: Token, sub_type: Box<Type>) -> Self {
    Self {
      optional_token,
      sub_type,
    }
  }
}

pub struct ParameterList {
  pub params: Vec<Parameter>,
}

impl ParameterList {
  pub fn new(params: Vec<Parameter>) -> Self {
    Self { params }
  }
}

pub struct Parameter {
  pub type_var: Option<TypeVar>,
  pub var_arg_token: Option<Token>,
  pub comma: Option<Token>,
}

impl Parameter {
  pub fn new(
    type_var: Option<TypeVar>,
    var_arg_token: Option<Token>,
    comma: Option<Token>,
  ) -> Self {
    Self {
      type_var,
      var_arg_token,
      comma,
    }
  }
}

pub struct ReturnList {
  pub returns: Vec<ReturnEntry>,
}

impl ReturnList {
  pub fn new(returns: Vec<ReturnEntry>) -> Self {
    Self { returns }
  }
}

pub struct ReturnEntry {
  pub type_var: TypeVar,
  pub assignment: Option<Token>,
  pub expression: Option<Box<Expression>>,
  pub comma: Option<Token>,
}

impl ReturnEntry {
  pub fn new(
    type_var: TypeVar,
    assignment: Option<Token>,
    expression: Option<Box<Expression>>,
    comma: Option<Token>,
  ) -> Self {
    Self {
      type_var,
      assignment,
      expression,
      comma,
    }
  }
}
