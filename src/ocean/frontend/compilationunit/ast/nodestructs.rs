use ocean_macros::{AstNode, New};
use crate::ocean::frontend::compilationunit::ast::astnodeindex::AstNodeIndex;
use crate::ocean::frontend::compilationunit::ast::astnode::{AstNodeMetadata, AstNodeTrait};
use crate::ocean::frontend::compilationunit::token::tokenindex::TokenIndex;

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Statement {
    metadata: AstNodeMetadata,
    pub data: AstNodeIndex,
    pub statement: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Annotation {
    metadata: AstNodeMetadata,
    pub token: TokenIndex,
    pub annotation_arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct AnnotationArgument {
    metadata: AstNodeMetadata,
    pub name: TokenIndex,
    pub value: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ExpressionStatement {
    metadata: AstNodeMetadata,
    pub expression_node: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct CompoundStatement {
    metadata: AstNodeMetadata,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct WhileLoop {
    metadata: AstNodeMetadata,
    pub condition: AstNodeIndex,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ForLoop {
    metadata: AstNodeMetadata,
    pub iterator: AstNodeIndex,
    pub iterable: AstNodeIndex,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Loop {
    metadata: AstNodeMetadata,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Branch {
    metadata: AstNodeMetadata,
    pub condition: AstNodeIndex,
    pub body: AstNodeIndex,
    pub else_branch: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ElseBranch {
    metadata: AstNodeMetadata,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Match {
    metadata: AstNodeMetadata,
    pub expression: AstNodeIndex,
    pub cases: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct MatchCase {
    metadata: AstNodeMetadata,
    pub pattern: AstNodeIndex,
    pub body: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Assignment {
    metadata: AstNodeMetadata,
    pub left_expression: AstNodeIndex,
    pub equal_token: TokenIndex,
    pub right_expression: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct LetTarget {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Identifier {
    metadata: AstNodeMetadata,
    pub identifier: TokenIndex,
    pub optional_type: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Function {
    metadata: AstNodeMetadata,
    pub identifier: TokenIndex,
    pub params: AstNodeIndex,
    pub results: AstNodeIndex,
    pub compound_statement: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct FunctionParam {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct FunctionReturn {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
    pub expression: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Pack {
    metadata: AstNodeMetadata,
    pub custom_type: AstNodeIndex,
    pub interface_declaration: Option<AstNodeIndex>,
    pub members: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct PackMember {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Union {
    metadata: AstNodeMetadata,
    pub custom_type: AstNodeIndex,
    pub interface_declaration: Option<AstNodeIndex>,
    pub members:AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct UnionMember {
    metadata: AstNodeMetadata,
    pub identifier: TokenIndex,
    pub sub_type: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct UnionSubTypes {
    metadata: AstNodeMetadata,
    pub types: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct UnionSubTypeEntry {
    metadata: AstNodeMetadata,
    pub type_entry: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Interface {
    metadata: AstNodeMetadata,
    pub custom_type: AstNodeIndex,
    pub entries: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct InterfaceEntry {
    metadata: AstNodeMetadata,
    pub function: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct InterfaceDeclaration {
    metadata: AstNodeMetadata,
    pub implemented_interfaces: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct InterfaceImplDeclaration {
    metadata: AstNodeMetadata,
    pub interface_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Return {
    metadata: AstNodeMetadata,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Break {
    metadata: AstNodeMetadata,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Continue {
    metadata: AstNodeMetadata,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Using {
    metadata: AstNodeMetadata,
    pub path: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct UsingPathEntry {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ExpressionNode {
    metadata: AstNodeMetadata,
    pub expression: AstNodeIndex,
}

// Expression ast nodes
#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct StringLiteral {
    metadata: AstNodeMetadata,
    pub literal: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ArrayLiteral {
    metadata: AstNodeMetadata,
    pub arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Number {
    metadata: AstNodeMetadata,
    pub literal: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Boolean {
    metadata: AstNodeMetadata,
    pub literal: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct InterpolatedString {
    metadata: AstNodeMetadata,
    pub literal: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Cast {
    metadata: AstNodeMetadata,
    pub expression: AstNodeIndex,
    pub casted_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Variable {
    metadata: AstNodeMetadata,
    pub identifier: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Call {
    metadata: AstNodeMetadata,
    pub target: AstNodeIndex,
    pub arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ArrayIndex {
    metadata: AstNodeMetadata,
    pub target: AstNodeIndex,
    pub arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Argument {
    metadata: AstNodeMetadata,
    pub value: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct Tuple {
    metadata: AstNodeMetadata,
    pub tuple_members: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TupleMember {
    metadata: AstNodeMetadata,
    pub identifier: TokenIndex,
    pub value: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct SubExpression {
    metadata: AstNodeMetadata,
    pub expression: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct PrefixOperator {
    metadata: AstNodeMetadata,
    pub operator: TokenIndex,
    pub expression: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct PostfixOperator {
    metadata: AstNodeMetadata,
    pub expression: AstNodeIndex,
    pub operator: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct BinaryOperator {
    metadata: AstNodeMetadata,
    pub left_expression: AstNodeIndex,
    pub operator: TokenIndex,
    pub right_expression: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TernaryOperator {
    metadata: AstNodeMetadata,
    pub left_expression: AstNodeIndex,
    pub first_operator: TokenIndex,
    pub middle_expression: TokenIndex,
    pub second_operator: TokenIndex,
    pub right_expression: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct BaseType {
    metadata: AstNodeMetadata,
    pub base_type: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct CustomType {
    metadata: AstNodeMetadata,
    pub identifier: TokenIndex,
    pub type_parameters: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TupleType {
    metadata: AstNodeMetadata,
    pub tuple_arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TupleArgument {
    metadata: AstNodeMetadata,
    pub optional_name: Option<TokenIndex>,
    pub argument_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TypeParameters {
    metadata: AstNodeMetadata,
    pub type_arguments: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct TypeArgument {
    metadata: AstNodeMetadata,
    pub argument_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct SubType {
    metadata: AstNodeMetadata,
    pub sub_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct AutoType {
    metadata: AstNodeMetadata,
    pub auto_token: TokenIndex,
    pub identifier: TokenIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct LazyType {
    metadata: AstNodeMetadata,
    pub lazy_token: TokenIndex,
    pub base_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct RefType {
    metadata: AstNodeMetadata,
    pub ref_token: TokenIndex,
    pub base_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct MutType {
    metadata: AstNodeMetadata,
    pub mut_token: TokenIndex,
    pub base_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct ArrayType {
    metadata: AstNodeMetadata,
    pub base_type: AstNodeIndex,
    pub index_type: Option<AstNodeIndex>,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct VariableType {
    metadata: AstNodeMetadata,
    pub base_type: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct FunctionType {
    metadata: AstNodeMetadata,
    pub param_types: AstNodeIndex,
    pub result_types: AstNodeIndex,
}

#[derive(AstNode, Copy, Clone, Debug, New)]
pub struct FunctionTypeArgument {
    metadata: AstNodeMetadata,
    pub arg_type: AstNodeIndex
}
