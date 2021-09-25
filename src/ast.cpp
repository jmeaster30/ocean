#include "ast.hpp"

#include <iostream>

/** VarType toCastString **/
std::string BaseType::toCastString() {
  return std::string(_type->lexeme.string_lex) + (_auto_name ? "-" + std::string(_auto_name->lexeme.string_lex) : "");
}

std::string CustomType::toCastString() {
  return _type->toCastString();
}

std::string FuncType::toCastString() {
  std::string params = "(";
  for(int i = 0; i < _param_types->size(); i++) {
    params += (*_param_types)[i]->toCastString();
    if (i != _param_types->size() - 1)
      params += "+";
  }
  params += ")";
  std::string returns = "(";
  for(int i = 0; i < _return_types->size(); i++) {
    returns += (*_return_types)[i]->toCastString();
    if (i != _return_types->size() - 1)
      returns += "+";
  }
  returns += ")";
  return "func" + params + returns;
}

std::string ConstType::toCastString() {
  return _type->toCastString() + "-c";
}

std::string PointerType::toCastString() {
  return _type->toCastString() + "-p";
}

std::string ArrayType::toCastString() {
  return _type->toCastString() + "-a";
}

std::string Variable::toCastString() {
  return std::string(_name->lexeme.string_lex) + ":" + _var->toCastString();
}

/** NODE TYPE FUNCTIONS **/

std::string BaseType::getNodeType() { return "BaseType"; }

std::string CustomType::getNodeType() { return "CustomType"; }

std::string FuncType::getNodeType() { return "FuncType"; }

std::string ConstType::getNodeType() { return "ConstType"; }

std::string PointerType::getNodeType() { return "PointerType"; }

std::string ArrayType::getNodeType() { return "ArrayType"; }

std::string Parameter::getNodeType() { return "Parameter"; }

std::string Program::getNodeType() { return "Program"; }

std::string Macro::getNodeType() { return "Macro"; }

std::string CompoundStmt::getNodeType() { return "Compound"; }

std::string VarDec::getNodeType() { return "VarDec"; }

std::string FuncDec::getNodeType() { return "FuncDec"; }

std::string CastFuncDec::getNodeType() { return "CastFuncDec"; }

std::string EnumDec::getNodeType() { return "EnumDec"; }

std::string PackDec::getNodeType() { return "PackDec"; }

std::string VariantDec::getNodeType() { return "VariantDec"; }

std::string IfStmt::getNodeType() { return "If"; }

std::string SwitchCase::getNodeType() { return "Case"; }

std::string SwitchStmt::getNodeType() { return "Switch"; }

std::string WhileStmt::getNodeType() { return "While"; }

std::string ForStmt::getNodeType() { return "For"; }

std::string ExprStmt::getNodeType() { return "Expr"; }

std::string StopStmt::getNodeType() { return "Stop"; }

std::string BreakStmt::getNodeType() { return "Break"; }

std::string ContinueStmt::getNodeType() { return "Continue"; }

std::string Variable::getNodeType() { return "Variable"; }

std::string MemberAccess::getNodeType() { return "MemberAccess"; }

std::string ArrayAccess::getNodeType() { return "ArrayAccess"; }

std::string Call::getNodeType() { return "Call"; }

std::string Assignment::getNodeType() { return "Assignment"; }

std::string BinaryExpr::getNodeType() { return "Binary"; }

std::string UnaryExpr::getNodeType() { return "Unary"; }

std::string Cast::getNodeType() { return "Cast"; }

std::string IntValue::getNodeType() { return "Int"; }

std::string HexValue::getNodeType() { return "Hex"; }

std::string BoolValue::getNodeType() { return "Bool"; }

std::string FloatValue::getNodeType() { return "Float"; }

std::string StringValue::getNodeType() { return "String"; }

std::string ArrayValue::getNodeType() { return "Array"; }

std::string ObjectValue::getNodeType() { return "Object"; }

/** toString FUNCTIONS **/

//! The following functions are not great and will result in a major performance hit
//!     (but we shouldn't be converting the entire ast to a string very often if even more than once)
//! However... I am not at the point of optimizing code and it seems like something I can do later
//! This comment recognizing the issue will hold me accountable and I accept that my future self will
//!s  be ashamed with me

std::string BaseType::toString() {
  std::string results = "(BaseType: ";
  if (_type) results += _type->toString();
  if (_auto_name) results += " (Name: " + _auto_name->toString() + ")";
  results += ")";
  return results;
}

std::string CustomType::toString() {
  std::string results = "(CustomType: ";
  if (_type) results += _type->toString();
  results += ")";
  return results;
}

std::string FuncType::toString() {
  std::string results = "(FuncType:";
  if (_param_types) {
    results += " (ParamTypes:";
    for (auto ptype : *_param_types) {
      results += " " + ptype->toString();
    }
    results += ")";
  }
  if (_return_types) {
    results += " (ReturnTypes:";
    for (auto rtype : *_return_types) {
      results += " " + rtype->toString();
    }
    results += ")";
  }
  results += ")";
  return results;
}

std::string ConstType::toString() {
  std::string results = "(Const: ";
  if (_type) results += _type->toString();
  results += ")";
  return results;
}

std::string PointerType::toString() {
  std::string results = "(Pointer: ";
  if (_type) results += _type->toString();
  results += ")";
  return results;
}

std::string ArrayType::toString() {
  std::string results = "(ArrayType: ";
  if (_type) results += _type->toString();
  if (_array_length) {
    results += " (Size: " + _array_length->toString() + ")";
  }
  results += ")";
  return results;
}

std::string Parameter::toString() {
  std::string results = "(Parameter: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_type) {
    results += " " + _type->toString();
  }
  results += ")";
  return results;
}

std::string Program::toString() {
  std::string results = "(Program:";
  if (_stmts) {
    for (auto stmt : *_stmts) {
      results += " " + stmt->toString();
    }
  }
  results += ")";
  return results;
}

std::string Macro::toString() {
  std::string results = "(Macro: ";
  if (_macro) results += _macro->toString();
  results += ")";
  return results;
}

std::string CompoundStmt::toString() {
  std::string results = "(Compound:";
  if (_stmts) {
    for (auto stmt : *_stmts) {
      results += " " + stmt->toString();
    }
  }
  results += ")";
  return results;
}

std::string VarDec::toString() {
  std::string results = "(VarDec: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_type) {
    results += " " +  _type->toString();
  }
  if (_expr) {
    results += " (Value: " + _expr->toString() + ")";
  }
  results += ")";
  return results;
}

std::string FuncDec::toString() {
  std::string results = "(FuncDec: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_params) {
    results += " (Params:";
    for (auto param : *_params) {
      results += " " + param->toString();
    }
    results += ")";
  }
  if (_returns) {
    results += " (Returns:";
    for (auto ret : *_returns) {
      results += " " + ret->toString();
    }
    results += ")";
  }
  if (_body) results += " " + _body->toString();
  results += ")";
  return results;
}

std::string CastFuncDec::toString() {
  std::string results = "(CastFuncDec: ";
  if (_casting_type) results += "(Name: " + _casting_type->toString() + ")";
  if (_params) {
    results += " (Params:";
    for (auto param : *_params) {
      results += " " + param->toString();
    }
    results += ")";
  }
  if (_returns) {
    results += " (Returns:";
    for (auto ret : *_returns) {
      results += " " + ret->toString();
    }
    results += ")";
  }
  if (_body) results += " " + _body->toString();
  results += ")";
  return results;
}

std::string EnumDec::toString() {
  std::string results = "(Enum: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_base_type) results += " " + _base_type->toString();
  if (_declist) {
    for (auto dec : *_declist) {
      results += " " + dec->toString();
    }
  }
  results += ")";
  return results;
}

std::string PackDec::toString() {
  std::string results = "(Pack: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_declist) {
    for (auto dec : *_declist) {
      results += " " + dec->toString();
    }
  }
  results += ")";
  return results;
}

std::string VariantDec::toString() {
  std::string results = "(Variant: ";
  if (_id) results += "(Name: " + _id->toString() + ")";
  if (_declist) {
    for (auto dec : *_declist) {
      results += " " + dec->toString();
    }
  }
  results += ")";
  return results;
}

std::string IfStmt::toString() {
  std::string results = "(If: ";
  if (_cond) {
    results += "(Condition: " + _cond->toString() + ")";
  }
  if (_body) {
    results += " (True: " + _body->toString() + ")";
  }
  if (_elseBody) {
    results += " (False: " + _elseBody->toString() + ")";
  }
  results += ")";
  return results;
}

std::string SwitchCase::toString() {
  std::string results = "(SwitchCase: ";
  if (_case) {
    results += "(Case: " + _case->toString() + ")";
  }
  if (_body) {
    results += " " + _body->toString();
  }
  results += ")";
  return results;
}

std::string SwitchStmt::toString() {
  std::string results = "(Switch: ";
  if (_cond) {
    results += "(Cond: " + _cond->toString() + ")";
  }
  if (_cases) {
    for (auto c : *_cases) {
      results += " " +  c->toString();
    }
  }
  results += ")";
  return results;
}

std::string WhileStmt::toString() {
  std::string results = "(While: ";
  if (_cond) {
    results += "(Cond: " + _cond->toString() + ")";
  }
  if (_body) {
    results += " " + _body->toString();
  }
  results += ")";
  return results;
}

std::string ForStmt::toString() {
  std::string results = "(For: ";
  if (_id) results += "(IterName: " + _id->toString() + ")";
  if (_iter) results += " (Iterable: " + _iter->toString() + ")";
  if (_by) results += " (By: " + _by->toString() + ")";
  if (_body) results += " " + _body->toString();
  results += ")";
  return results;
}

std::string ExprStmt::toString() {
  std::string results = "(Expr: ";
  if (_expr) results += _expr->toString();
  results += ")";
  return results;
}

std::string StopStmt::toString() { return "(Stop:)"; }
std::string BreakStmt::toString() { return "(Break:)"; }
std::string ContinueStmt::toString() { return "(Continue:)"; }

std::string Variable::toString() {
  std::string result = "(Variable: ";
  if (!_var) {
    if (_name) result += "(Name: " + _name->toString() + ")";
  } else {
    if (_name) result += "(Namespace: " + _name->toString() + ")";
    if (_var) result += " " + _var->toString();
  }
  result += ")";
  return result;
}

std::string MemberAccess::toString() {
  std::string result = "(MemberAccess: ";
  if (_parent) result += _parent->toString();
  if (_id) result += " (MemberName: " + _id->toString() + ")";
  result += ")";
  return result;
}

std::string ArrayAccess::toString() {
  std::string result = "(ArrayAccess: ";
  if (_parent) result += _parent->toString();
  if (_expr) result += " (Element: " + _expr->toString() + ")";
  result += ")";
  return result;
}

std::string Call::toString() {
  std::string result = "(Call: ";
  if (_parent) result += _parent->toString();
  if (_args) {
    result +=  " (Args:";
    for(auto arg : *_args) {
      result +=  " " + arg->toString();
    }
    result += ")";
  }
  result += ")";
  return result;
}

std::string Assignment::toString() {
  std::string result = "(Assignment: ";
  if (_op) result += "(Op: " + _op->toString() + ")";
  if (_var) result += _var->toString();
  if (_expr) result += " (Value: " +_expr->toString() + ")";
  result += ")";
  return result;
}

std::string BinaryExpr::toString() {
  std::string results = "(Binary: ";
  if (_op) results += "(Op: " + _op->toString() + ")";
  if (_left) results += " " + _left->toString();
  if (_right) results += " " + _right->toString();
  results += ")";
  return results;
}

std::string UnaryExpr::toString() {
  std::string results = "(Unary: ";
  if (_op) results += "(Op: " + _op->toString() + ")";
  if (_expr) results += " " + _expr->toString();
  results += ")";
  return results;
}

std::string Cast::toString() {
  std::string results = "(Cast: ";
  if (_type) results += _type->toString();
  if (_expr) results += " " + _expr->toString();
  results += ")";
  return results;
}

std::string IntValue::toString() {
  std::string results = "(Int: ";
  if (_value) results += _value->toString();
  results += ")";
  return results;
}

std::string HexValue::toString() {
  std::string results = "(Hex: ";
  if (_value) results += _value->toString();
  results += ")";
  return results;
}

std::string BoolValue::toString() {
  std::string results = "(Bool: ";
  if (_value) results += _value->toString();
  results += ")";
  return results;
}

std::string FloatValue::toString() {
  std::string results = "(Float: ";
  if (_value) results += _value->toString();
  results += ")";
  return results;
}

std::string StringValue::toString() {
  std::string results = "(String: ";
  if (_value) results += _value->toString();
  results += ")";
  return results;
}

std::string ArrayValue::toString() {
  std::string results = "(Array:";
  if (_elements) {
    for (auto elem : *_elements) {
      results += " " + elem->toString();
    }
  }
  results += ")";
  return results;
}

std::string ObjectValue::toString() {
  std::string results = "(ObjectInitialize:";
  if (_elements) {
    for (auto elem : *_elements) {
      results += " " + elem->toString();
    }
  }
  results += ")";
  return results;
}
