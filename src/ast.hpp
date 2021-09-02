#pragma once

#include <string>
#include <vector>

#include "token.hpp"

class AstNode {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class Statement : public AstNode {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class Expression : public AstNode {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class VarType : public AstNode {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class BaseType : public VarType {
public:
  Token* _type;
  Token* _auto_name;

  BaseType(Token* type, Token* auto_name) :
    _type(type), _auto_name(auto_name) {}

  std::string getNodeType();
  std::string toString();
};

class FuncType : public VarType {
public:
  Token* _func;
  std::vector<VarType*>* _param_types;
  std::vector<VarType*>* _return_types;

  FuncType(Token* func, std::vector<VarType*>* param_types, std::vector<VarType*>* return_types) :
    _func(func), _param_types(param_types), _return_types(return_types) {}

  std::string getNodeType();
  std::string toString();
};

class ConstType : public VarType {
public:
  VarType* _type;

  ConstType(VarType* type) : _type(type) {}

  std::string getNodeType();
  std::string toString();
};

class PointerType : public VarType {
public:
  VarType* _type;
  
  PointerType(VarType* type) : _type(type) {}

  std::string getNodeType();
  std::string toString();
};

class ArrayType : public VarType {
public:
  VarType* _type;
  Expression* _array_length;

  ArrayType(VarType* type, Expression* array_length) :
    _type(type), _array_length(array_length) {}

  std::string getNodeType();
  std::string toString();
};

class Parameter : public AstNode {
public:
  Token* _id;
  VarType* _type;

  Parameter(Token* id, VarType* type) :
    _id(id), _type(type) {}

  std::string getNodeType();
  std::string toString();
};

class Program : public AstNode {
public:
  std::vector<Statement*>* _stmts;

  Program(std::vector<Statement*>* stmts) : _stmts(stmts) {}

  std::string getNodeType();
  std::string toString();
};

class Macro : public Statement {
public:
  Token* _macro;

  Macro(Token* macro) : _macro(macro) {}

  std::string getNodeType();
  std::string toString();
};

class CompoundStmt : public Statement {
public:
  std::vector<Statement*>* _stmts;

  CompoundStmt(std::vector<Statement*>* stmts) : _stmts(stmts) {}

  std::string getNodeType();
  std::string toString();
};

class Declaration : public Statement {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class VarDec : public Declaration {
public:
  Token* _id;
  VarType* _type;
  Expression* _expr;

  VarDec(Token* id, VarType* type, Expression* expr) :
    _id(id), _type(type), _expr(expr) {}

  std::string getNodeType();
  std::string toString();
};

class FuncDec : public Declaration {
public:
  Token* _id;
  std::vector<Parameter*>* _params;
  std::vector<Parameter*>* _returns;
  CompoundStmt* _body;

  FuncDec(Token* id, std::vector<Parameter*>* params, std::vector<Parameter*>* returns, CompoundStmt* body) :
    _id(id), _params(params), _returns(returns), _body(body) {}

  std::string getNodeType();
  std::string toString();
};

class EnumDec : public Declaration {
public:
  Token* _start;
  Token* _id;
  std::vector<Declaration*>* _declist;

  EnumDec(Token* start, Token* id, std::vector<Declaration*>* declist) :
    _start(start), _id(id), _declist(declist) {}

  std::string getNodeType();
  std::string toString();
};

class PackDec : public Declaration {
public:
  Token* _start;
  Token* _id;
  std::vector<Declaration*>* _declist;

  PackDec(Token* start, Token* id, std::vector<Declaration*>* declist) :
    _start(start), _id(id), _declist(declist) {}

  std::string getNodeType();
  std::string toString();
};

class VariantDec : public Declaration {
public:
  Token* _start;
  Token* _id;
  std::vector<Declaration*>* _declist;

  VariantDec(Token* start, Token* id, std::vector<Declaration*>* declist) :
    _start(start), _id(id), _declist(declist) {}

  std::string getNodeType();
  std::string toString();
};

class IfStmt : public Statement {
public:
  Token* _start;
  Expression* _cond;
  CompoundStmt* _body;
  Statement* _elseBody; //will either be if statement or compound statement

  IfStmt(Token* start, Expression* cond, CompoundStmt* body, Statement* elseBody) :
    _start(start), _cond(cond), _body(body), _elseBody(elseBody) {}

  std::string getNodeType();
  std::string toString();
};

class SwitchCase : public AstNode {
public:
  Expression* _case;
  CompoundStmt* _body;

  SwitchCase(Expression* c, CompoundStmt* body) :
    _case(c), _body(body) {}

  std::string getNodeType();
  std::string toString();
};

class SwitchStmt : public Statement {
public:
  Token* _start;
  Expression* _cond;
  std::vector<SwitchCase*>* _cases;

  SwitchStmt(Token* start, Expression* cond, std::vector<SwitchCase*>* cases) :
    _start(start), _cond(cond), _cases(cases) {}

  std::string getNodeType();
  std::string toString();
};

class WhileStmt : public Statement {
public:
  Token* _start;
  Expression* _cond;
  CompoundStmt* _body;

  WhileStmt(Token* start, Expression* cond, CompoundStmt* body) :
    _start(start), _cond(cond), _body(body) {}

  std::string getNodeType();
  std::string toString();
};

class ForStmt : public Statement {
public:
  Token* _start;
  Token* _id;
  Expression* _iter;
  Expression* _by;
  CompoundStmt* _body;

  ForStmt(Token* start, Token* id, Expression* iter, Expression* by, CompoundStmt* body) :
    _start(start), _id(id), _iter(iter), _by(by), _body(body) {}

  std::string getNodeType();
  std::string toString();
};

class ExprStmt : public Statement {
public:
  Expression* _expr;

  ExprStmt(Expression* expr) : _expr(expr) {}

  std::string getNodeType();
  std::string toString();
};

class StopStmt : public Statement {
public:
  Token* _token;

  StopStmt(Token* token) : _token(token) {}
  std::string getNodeType();
  std::string toString();
};

class BreakStmt : public Statement {
public:
  Token* _token;

  BreakStmt(Token* token) : _token(token) {}
  std::string getNodeType();
  std::string toString();
};

class ContinueStmt : public Statement {
public:
  Token* _token;

  ContinueStmt(Token* token) : _token(token) {}
  std::string getNodeType();
  std::string toString();
};

class Var : public Expression {
public:
  virtual std::string getNodeType() = 0;
  virtual std::string toString() = 0;
};

class Variable : public Var {
public:
  Token* _name;
  Variable* _var;

  Variable(Token* name, Variable* var) :  
    _name(name), _var(var) {}

  std::string getNodeType();
  std::string toString();
};

class MemberAccess : public Var {
public:
  Var* _parent;
  Token* _id;

  MemberAccess(Var* parent, Token* id) : _parent(parent), _id(id) {}

  std::string getNodeType();
  std::string toString();
};

class ArrayAccess : public Var {
public:
  Var* _parent;
  Expression* _expr;

  ArrayAccess(Var* parent, Expression* expr) : _parent(parent), _expr(expr) {}

  std::string getNodeType();
  std::string toString();
};

class Call : public Var {
public:
  Var* _parent;
  std::vector<Expression*>* _args;

  Call(Var* parent, std::vector<Expression*>* args) :
    _parent(parent), _args(args) {}

  std::string getNodeType();
  std::string toString();
};

class Assignment : public Expression {
public:
  Token* _op;
  Var* _var;
  Expression* _expr;

  Assignment(Var* var, Token* op, Expression* expr) :
    _var(var), _op(op), _expr(expr) {}

  std::string getNodeType();
  std::string toString();
};

class BinaryExpr : public Expression {
public:
  Token* _op;
  Expression* _left;
  Expression* _right;

  BinaryExpr(Token* op, Expression* left, Expression* right) :
    _op(op), _left(left), _right(right) {}

  std::string getNodeType();
  std::string toString();
};

class UnaryExpr : public Expression {
public:
  Token* _op;
  Expression* _expr;

  UnaryExpr(Token* op, Expression* expr) : _op(op), _expr(expr) {}

  std::string getNodeType();
  std::string toString();
};

class IntValue : public Expression {
public:
  Token* _value;

  IntValue(Token* value) : _value(value) {}

  std::string getNodeType();
  std::string toString();
};

class HexValue : public Expression {
public:
  Token* _value;

  HexValue(Token* value) : _value(value) {}

  std::string getNodeType();
  std::string toString();
};

class BoolValue : public Expression {
public:
  Token* _value;

  BoolValue(Token* value) : _value(value) {}

  std::string getNodeType();
  std::string toString();
};

class FloatValue : public Expression {
public:
  Token* _value;

  FloatValue(Token* value) : _value(value) {}

  std::string getNodeType();
  std::string toString();
};

class StringValue : public Expression {
public:
  Token* _value;

  StringValue(Token* value) : _value(value) {}

  std::string getNodeType();
  std::string toString();
};

class ArrayValue : public Expression {
public:
  std::vector<Expression*>* _elements;

  ArrayValue(std::vector<Expression*>* elements) : _elements(elements) {}

  std::string getNodeType();
  std::string toString();
};

class ObjectValue : public Expression {
public:
  std::vector<Declaration*>* _elements;

  ObjectValue(std::vector<Declaration*>* elements) : _elements(elements) {}

  std::string getNodeType();
  std::string toString();
};
