use std::cell::RefCell;
use std::rc::Rc;
use crate::ocean::frontend::tokentype::TokenType;
use crate::util::token::Token;
use itertools::{Either, Itertools};
use ocean_macros::New;
use crate::ocean::frontend::semanticanalysis::symboltable::SymbolTable;

#[derive(Clone, Debug, New)]
pub struct Program {
  pub statements: Vec<StatementNode>,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct StatementNode {
  pub data: Vec<StatementNodeData>,
  pub statement: Option<Statement>,
}

#[derive(Clone, Debug)]
pub enum StatementNodeData {
  #[deprecated(note="I am too lazy to fully remove this but comments should be tracked as trivia now")]
  Comment(Comment),
  Annotation(Annotation),
}

#[derive(Clone, Debug, New)]
pub struct Comment {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Annotation {
  pub token: Token<TokenType>,
  pub left_paren: Option<Token<TokenType>>,
  pub annotation_arguments: Vec<AnnotationArgument>,
  pub right_paren: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct AnnotationArgument {
  pub name: Token<TokenType>,
  pub colon: Token<TokenType>,
  pub value: ExpressionNode,
  pub comma: Option<Token<TokenType>>,
}

#[derive(Clone, Debug)]
pub enum Statement {
  WhileLoop(WhileLoop),
  ForLoop(ForLoop),
  Loop(Loop),
  Branch(Branch),
  Match(Match),
  Assignment(Assignment),
  Function(Function),
  Pack(Pack),
  Union(Union),
  Interface(Interface),
  Return(Return),
  Break(Break),
  Continue(Continue),
  Using(Using),
  Expression(ExpressionStatement),
}

#[derive(Clone, Debug, New)]
pub struct ExpressionStatement {
  pub expression_node: ExpressionNode,
  pub semicolon_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct CompoundStatement {
  pub left_curly: Token<TokenType>,
  pub body: Vec<StatementNode>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct WhileLoop {
  pub while_token: Token<TokenType>,
  pub condition: ExpressionNode,
  pub body: CompoundStatement,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct ForLoop {
  pub for_token: Token<TokenType>,
  pub iterator: ExpressionNode,
  pub in_token: Token<TokenType>,
  pub iterable: ExpressionNode,
  pub body: CompoundStatement,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct Loop {
  pub loop_token: Token<TokenType>,
  pub body: CompoundStatement,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct Branch {
  pub if_token: Token<TokenType>,
  pub condition: ExpressionNode,
  pub body: CompoundStatement,
  pub else_branch: Option<ElseBranch>,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct ElseBranch {
  pub else_token: Token<TokenType>,
  pub body: Either<CompoundStatement, Box<Branch>>,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct Match {
  pub match_token: Token<TokenType>,
  pub expression: ExpressionNode,
  pub left_curly: Token<TokenType>,
  pub cases: Vec<MatchCase>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct MatchCase {
  pub pattern: ExpressionNode,
  pub arrow_token: Token<TokenType>,
  pub body: Either<Statement, CompoundStatement>,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct Assignment {
  pub left_expression: Either<LetTarget, ExpressionNode>,
  pub equal_token: Token<TokenType>,
  pub right_expression: ExpressionStatement,
}

#[derive(Clone, Debug, New)]
pub struct LetTarget {
  pub let_token: Token<TokenType>,
  pub identifier: Identifier,
}

#[derive(Clone, Debug, New)]
pub struct Identifier {
  pub identifier: Token<TokenType>,
  pub colon: Option<Token<TokenType>>,
  pub optional_type: Option<Type>,
}

#[derive(Clone, Debug)]
pub enum Type {
  Base(BaseType),
  Custom(CustomType),
  Auto(AutoType),
  Lazy(LazyType),
  Ref(RefType),
  Mutable(MutType),
  Function(FunctionType),
  Array(ArrayType),
  VariableType(VariableType),
}

#[derive(Clone, Debug, New)]
pub struct BaseType {
  pub base_type: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct CustomType {
  pub identifier: Token<TokenType>,
  pub type_parameters: Option<TypeParameters>,
}

#[derive(Clone, Debug, New)]
pub struct TypeParameters {
  pub left_paren_token: Token<TokenType>,
  pub type_arguments: Vec<TypeArgument>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct TypeArgument {
  pub argument_type: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct SubType {
  pub left_paren_token: Token<TokenType>,
  pub sub_type: Box<Type>,
  pub right_paren_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct AutoType {
  pub auto_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct LazyType {
  pub lazy_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct RefType {
  pub ref_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct MutType {
  pub mut_token: Token<TokenType>,
  pub base_type: Box<Type>,
}

#[derive(Clone, Debug, New)]
pub struct ArrayType {
  pub base_type: Box<Type>,
  pub left_square: Token<TokenType>,
  pub index_type: Option<Box<Type>>,
  pub right_square: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct VariableType {
  pub base_type: Box<Type>,
  pub spread_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionType {
  pub function_token: Token<TokenType>,
  pub param_left_paren: Token<TokenType>,
  pub param_types: Vec<FunctionTypeArgument>,
  pub param_right_paren: Token<TokenType>,
  pub arrow_token: Option<Token<TokenType>>,
  pub result_left_paren: Option<Token<TokenType>>,
  pub result_types: Vec<FunctionTypeArgument>,
  pub result_right_paren: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionTypeArgument {
  pub arg_type: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Function {
  pub function_token: Token<TokenType>,
  pub identifier: Token<TokenType>,
  pub param_left_paren: Token<TokenType>,
  pub params: Vec<FunctionParam>,
  pub param_right_paren: Token<TokenType>,
  pub arrow_token: Option<Token<TokenType>>,
  pub result_left_paren: Option<Token<TokenType>>,
  pub results: Vec<FunctionReturn>,
  pub result_right_paren: Option<Token<TokenType>>,
  pub compound_statement: Option<CompoundStatement>,
  #[default(None)]
  pub table: Option<Rc<RefCell<SymbolTable>>>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionParam {
  pub identifier: Identifier,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct FunctionReturn {
  pub identifier: Identifier,
  pub equal_token: Option<Token<TokenType>>,
  pub expression: Option<ExpressionNode>,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Pack {
  pub pack_token: Token<TokenType>,
  pub custom_type: CustomType,
  pub interface_declaration: Option<InterfaceDeclaration>,
  pub left_curly: Token<TokenType>,
  pub members: Vec<PackMember>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct PackMember {
  pub identifier: Identifier,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Union {
  pub union_token: Token<TokenType>,
  pub custom_type: CustomType,
  pub interface_declaration: Option<InterfaceDeclaration>,
  pub left_curly: Token<TokenType>,
  pub members: Vec<UnionMember>,
  pub right_curly: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct UnionMember {
  pub identifier: Token<TokenType>,
  pub sub_type: Option<UnionSubTypes>,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct UnionSubTypes {
  pub left_paren: Token<TokenType>,
  pub types: Vec<UnionSubTypeEntry>,
  pub right_paren: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct UnionSubTypeEntry {
  pub types: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Interface {
  pub interface_token: Token<TokenType>,
  pub custom_type: CustomType,
  pub left_curly_token: Token<TokenType>,
  pub entries: Vec<InterfaceEntry>,
  pub right_curly_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct InterfaceEntry {
  pub function: Function,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct InterfaceDeclaration {
  pub colon_token: Token<TokenType>,
  pub implemented_interfaces: Vec<InterfaceImplDeclaration>,
}

#[derive(Clone, Debug, New)]
pub struct InterfaceImplDeclaration {
  pub interface_type: Type,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Return {
  pub return_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Break {
  pub break_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Continue {
  pub continue_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Using {
  pub using_token: Token<TokenType>,
  pub path: Vec<UsingPathEntry>,
}

impl Using {
  pub fn get_file_path(&self) -> String {
    self.path.iter().map(|x| x.identifier.lexeme.clone()).join("/").to_string() + ".sea"
  }
}

#[derive(Clone, Debug, New)]
pub struct UsingPathEntry {
  pub identifier: Token<TokenType>,
  pub dot_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct ExpressionNode {
  pub tokens: Vec<Either<Token<TokenType>, AstNodeExpression>>,
  #[default(None)]
  pub parsed_expression: Option<Expression>,
}

// Expression ast nodes

#[derive(Clone, Debug)]
pub enum Expression {
  String(StringLiteral),
  ArrayLiteral(ArrayLiteral),
  Number(Number),
  Boolean(Boolean),
  InterpolatedString(InterpolatedString),
  Variable(Variable),
  Tuple(Tuple),
  Call(Call),
  ArrayIndex(ArrayIndex),
  SubExpression(SubExpression),
  Cast(Cast),
  PrefixOperation(PrefixOperator),
  PostfixOperation(PostfixOperator),
  BinaryOperation(BinaryOperator),
  AstNode(AstNodeExpression),
}

#[derive(Clone, Debug)]
pub enum AstNodeExpression {
  Match(Box<Match>),
  Loop(Box<Loop>),
  ForLoop(Box<ForLoop>),
  WhileLoop(Box<WhileLoop>),
  Branch(Box<Branch>),
  Function(Box<Function>),
}

#[derive(Clone, Debug, New)]
pub struct StringLiteral {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct ArrayLiteral {
  pub left_square_token: Token<TokenType>,
  pub arguments: Vec<Argument>,
  pub right_square_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Number {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Boolean {
  pub token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct InterpolatedString {
  pub token: Token<TokenType>,
  pub subexpressions: Vec<Expression>,
}

#[derive(Clone, Debug, New)]
pub struct Cast {
  pub expression: Box<Expression>,
  pub as_token: Token<TokenType>,
  pub casted_type: Type,
}

#[derive(Clone, Debug, New)]
pub struct Variable {
  pub identifier: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Call {
  pub target: Box<Expression>,
  pub left_paren: Token<TokenType>,
  pub arguments: Vec<Argument>,
  pub right_paren: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct ArrayIndex {
  pub target: Box<Expression>,
  pub left_square: Token<TokenType>,
  pub argument: Vec<Argument>,
  pub right_square: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct Argument {
  pub value: Expression,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct Tuple {
  pub left_token: Token<TokenType>,
  pub tuple_members: Vec<TupleMember>,
  pub right_token: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct TupleMember {
  pub identifier: Option<Token<TokenType>>,
  pub colon_token: Option<Token<TokenType>>,
  pub value: Expression,
  pub comma_token: Option<Token<TokenType>>,
}

#[derive(Clone, Debug, New)]
pub struct SubExpression {
  pub left_paren: Token<TokenType>,
  pub expression: Box<Expression>,
  pub right_paren: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct PrefixOperator {
  pub operator: Token<TokenType>,
  pub expression: Box<Expression>,
}

#[derive(Clone, Debug, New)]
pub struct PostfixOperator {
  pub expression: Box<Expression>,
  pub operator: Token<TokenType>,
}

#[derive(Clone, Debug, New)]
pub struct BinaryOperator {
  pub left_expression: Box<Expression>,
  pub operator: Token<TokenType>,
  pub right_expression: Box<Expression>,
}

#[derive(Clone, Debug, New)]
pub struct TernaryOperator {
  pub left_expression: Expression,
  pub first_operator: Token<TokenType>,
  pub middle_expression: Expression,
  pub second_operator: Token<TokenType>,
  pub right_expression: Expression,
}

// annotation ast nodes
#[derive(Clone, Debug)]
pub enum AnnotationNode {
  Operator(AnnotationOperator),
  Hydro,
  FunctionAnnotation,
  Cast,
  None,
}

#[derive(Clone, Debug)]
pub enum OperatorType {
  Infix,
  Postfix,
  Prefix,
}

#[derive(Clone, Debug, New)]
pub struct AnnotationOperator {
  pub operator: String,
  pub operator_type: OperatorType,
  pub left_hand_side_name: Option<String>,
  pub left_precedence: Option<usize>,
  pub right_hand_side_name: Option<String>,
  pub right_precedence: Option<usize>,
}
