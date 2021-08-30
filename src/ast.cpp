#include "ast.hpp"

#include <iostream>

/** NODE TYPE FUNCTIONS **/

std::string BaseType::getNodeType() { return "BaseType"; }

std::string ConstType::getNodeType() { return "ConstType"; }

std::string PointerType::getNodeType() { return "PointerType"; }

std::string ArrayType::getNodeType() { return "ArrayType"; }

std::string Parameter::getNodeType() { return "Parameter"; }

std::string Program::getNodeType() { return "Program"; }

std::string Macro::getNodeType() { return "Macro"; }

std::string CompoundStmt::getNodeType() { return "Compound"; }

std::string VarDec::getNodeType() { return "VarDec"; }

std::string FuncDec::getNodeType() { return "FuncDec"; }

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

std::string IntValue::getNodeType() { return "Int"; }

std::string HexValue::getNodeType() { return "Hex"; }

std::string BoolValue::getNodeType() { return "Bool"; }

std::string FloatValue::getNodeType() { return "Float"; }

std::string StringValue::getNodeType() { return "String"; }

std::string ArrayValue::getNodeType() { return "Array"; }

std::string ObjectValue::getNodeType() { return "Object"; }

/** PRINT FUNCTIONS **/

void BaseType::print() {
  std::cout << "(BaseType: ";
  if (_type) std::cout << _type->toString();
  if (_auto_name) std::cout << " (Name: " << _auto_name->toString() << ")";
  std::cout << ")";
}

void ConstType::print() {
  std::cout << "(Const: ";
  if (_type) _type->print();
  std::cout << ")";
}

void PointerType::print() {
  std::cout << "(Pointer: ";
  if (_type) _type->print();
  std::cout << ")";
}

void ArrayType::print() {
  std::cout << "(ArrayType: ";
  if (_type) _type->print();
  if (_array_length) {
    std::cout << " (Size: ";
    _array_length->print();
    std::cout << ")";
  }
  std::cout << ")";
}

void Parameter::print() {
  std::cout << "(Parameter: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_type) {
    std::cout << " ";
    _type->print();
  }
  std::cout << ")";
}

void Program::print() {
  std::cout << "(Program:";
  if (_stmts) {
    for (auto stmt : *_stmts) {
      std::cout << " ";
      stmt->print();
    }
  }
  std::cout << ")";
}

void Macro::print() {
  std::cout << "(Macro: ";
  if (_macro) std::cout << _macro->toString();
  std::cout << ")";
}

void CompoundStmt::print() {
  std::cout << "(Compound:";
  if (_stmts) {
    for (auto stmt : *_stmts) {
      std::cout << " ";
      stmt->print();
    }
  }
  std::cout << ")";
}

void VarDec::print() {
  std::cout << "(VarDec: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_type) {
    std::cout << " ";
    _type->print();
  }
  if (_expr) {
    std::cout << " (Value: ";
    _expr->print();
    std::cout << ")";
  }
  std::cout << ")";
}

void FuncDec::print() {
  std::cout << "(FuncDec: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_params) {
    std::cout << " (Params:";
    for (auto param : *_params) {
      std::cout << " ";
      param->print();
    }
    std::cout << ")";
  }
  if (_returns) {
    std::cout << " (Returns:";
    for (auto ret : *_returns) {
      std::cout << " ";
      ret->print();
    }
    std::cout << ")";
  }
  if (_body) _body->print();
  std::cout << ")";
}

void EnumDec::print() {
  std::cout << "(Enum: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_declist) {
    for (auto dec : *_declist) {
      std::cout << " ";
      dec->print();
    }
  }
  std::cout << ")";
}

void PackDec::print() {
  std::cout << "(Pack: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_declist) {
    for (auto dec : *_declist) {
      std::cout << " ";
      dec->print();
    }
  }
  std::cout << ")";
}

void VariantDec::print() {
  std::cout << "(Variant: ";
  if (_id) std::cout << "(Name: " << _id->toString() << ")";
  if (_declist) {
    for (auto dec : *_declist) {
      std::cout << " ";
      dec->print();
    }
  }
  std::cout << ")";
}

void IfStmt::print() {
  std::cout << "(If: ";
  if (_cond) {
    std::cout << "(Condition: ";
    _cond->print();
    std::cout << ")";
  }
  if (_body) {
    std::cout << " (True: ";
    _body->print();
    std::cout << ")";
  }
  if (_elseBody) {
    std::cout << " (False: ";
    _elseBody->print();
    std::cout << ")";
  }
  std::cout << ")";
}

void SwitchCase::print() {
  std::cout << "(SwitchCase: ";
  if (_case) {
    std::cout << "(Case: ";
    _case->print();
    std::cout << ")";
  }
  if (_body) {
    std::cout << " ";
    _body->print();
  }
  std::cout << ")";
}

void SwitchStmt::print() {
  std::cout << "(Switch: ";
  if (_cond) {
    std::cout << "(Cond: ";
    _cond->print();
    std::cout << ")";
  }
  if (_cases) {
    for (auto c : *_cases) {
      std::cout << " ";
      c->print();
    }
  }
}

void WhileStmt::print() {
  std::cout << "(While: ";
  if (_cond) {
    std::cout << "(Cond: ";
    _cond->print();
    std::cout << ")";
  }
  if (_body) {
    std::cout << " ";
    _body->print();
  }
  std::cout << ")";
}

void ForStmt::print() {
  std::cout << "(For: ";
  if (_id) std::cout << "(IterName: " << _id->toString() << ")";
  if (_iter) {
    std::cout << " (Iterable: ";
    _iter->print();
    std::cout << ")";
  }
  if (_by) {
    std::cout << " (By: ";
    _by->print();
    std::cout << ")";
  }
  if (_body) {
    std::cout << " ";
    _body->print();
  }
  std::cout << ")";
}

void ExprStmt::print() {
  std::cout << "(Expr: ";
  if (_expr) _expr->print();
  std::cout << ")";
}

void StopStmt::print() { std::cout << "(Stop:)"; }
void BreakStmt::print() { std::cout << "(Break:)"; }
void ContinueStmt::print() { std::cout << "(Continue:)"; }

void Variable::print() {
  std::cout << "(Variable: ";
  if (!_var) {
    if (_name) std::cout << "(Name: " << _name->toString() << ")";
  } else {
    if (_name) std::cout << "(Namespace: " << _name->toString() << ")";
    if (_var) {
      std::cout << " ";
      _var->print();
    }
  }
  std::cout << ")";
}

void MemberAccess::print() {
  std::cout << "(MemberAccess: ";
  if (_parent) _parent->print();
  if (_id) std::cout << " (MemberName: " << _id->toString() << ")";
  std::cout << ")";
}

void ArrayAccess::print() {
  std::cout << "(ArrayAccess: ";
  if (_parent) _parent->print();
  if (_expr) {
    std::cout << " (Element: ";
    _expr->print();
    std::cout << ")";
  }
  std::cout << ")";
}

void Call::print() {
  std::cout << "(Call: ";
  if (_parent) _parent->print();
  if (_args) {
    std::cout << " (Args:";
    for(auto arg : *_args) {
      std::cout << " ";
      arg->print();
    }
    std::cout << ")";
  }
  std::cout << ")";
}

void Assignment::print() {
  std::cout << "(Assignment: ";
  if (_op) std::cout << "(Op: " << _op->toString() << ")";
  if (_var) {
    _var->print();
  }
  if (_expr) {
    std::cout << " (Value: ";
    _expr->print();
    std::cout << ")";
  }
  std::cout << ")";
}

void BinaryExpr::print() {
  std::cout << "(Binary: ";
  if (_op) std::cout << "(Op: " << _op->toString() << ")";
  if (_left) {
    std::cout << " ";
    _left->print();
  }
  if (_right) {
    std::cout << " ";
    _right->print();
  }
  std::cout << ")";
}

void UnaryExpr::print() {
  std::cout << "(Unary: ";
  if (_op) std::cout << "(Op: " << _op->toString() << ")";
  if (_expr) {
    std::cout << " ";
    _expr->print();
  }
  std::cout << ")";
}

void IntValue::print() {
  std::cout << "(Int: ";
  if (_value) std::cout << _value->toString();
  std::cout << ")";
}

void HexValue::print() {
  std::cout << "(Hex: ";
  if (_value) std::cout << _value->toString();
  std::cout << ")";
}

void BoolValue::print() {
  std::cout << "(Bool: ";
  if (_value) std::cout << _value->toString();
  std::cout << ")";
}

void FloatValue::print() {
  std::cout << "(Float: ";
  if (_value) std::cout << _value->toString();
  std::cout << ")";
}

void StringValue::print() {
  std::cout << "(String: ";
  if (_value) std::cout << _value->toString();
  std::cout << ")";
}

void ArrayValue::print() {
  std::cout << "(Array:";
  if (_elements) {
    for (auto elem : *_elements) {
      std::cout << " ";
      elem->print();
    }
  }
  std::cout << ")";
}

void ObjectValue::print() {
  std::cout << "(ObjectInitialize:";
  if (_elements) {
    for (auto elem : *_elements) {
      std::cout << " ";
      elem->print();
    }
  }
  std::cout << ")";
}
