#include "ast.hpp"
#include "helper.hpp"
#include <assert.h>

SymType getSymTypeFromLexeme(std::string lexeme) {
  //This could be better
  SymType result = SymType::None;
  if(lexeme == "string") result = SymType::String;
  if(lexeme == "bool") result = SymType::Boolean;
  if(lexeme == "byte") result = SymType::Byte;
  if(lexeme == "i16") result = SymType::I16;
  if(lexeme == "i32") result = SymType::I32;
  if(lexeme == "i64") result = SymType::I64;
  if(lexeme == "s16") result = SymType::S16;
  if(lexeme == "s32") result = SymType::S32;
  if(lexeme == "s64") result = SymType::S64;
  if(lexeme == "u16") result = SymType::U16;
  if(lexeme == "u32") result = SymType::U32;
  if(lexeme == "u64") result = SymType::U64;
  if(lexeme == "f32") result = SymType::F32;
  if(lexeme == "f64") result = SymType::F64;
  if(lexeme == "f128") result = SymType::F128;
  return result;
}

void VariantDec::buildVTable(VTable* vtable, SymbolTable* table) {

}

void PackDec::buildVTable(VTable* vtable, SymbolTable* table) {

}

void EnumDec::buildVTable(VTable* vtable, SymbolTable* table) {

}

void OpFuncDec::buildVTable(VTable* vtable, SymbolTable* table) {
  //this should also just add the operator function to the table and not the vtable
}

void CastFuncDec::buildVTable(VTable* vtable, SymbolTable* table) {
  //this should actually just add the cast function to the table and not the vtable
}

void FuncDec::buildVTable(VTable* vtable, SymbolTable* table) {

}

void VarDec::buildVTable(VTable* vtable, SymbolTable* table) {

}

Symbol* BaseType::buildSymbolTable(SymbolTable* table) { 
  SymType mainType = (_type->type == TokenType::Auto) ? SymType::Auto : getSymTypeFromLexeme(_type->lexeme.string_lex);
  
  symbol = new Symbol("", mainType, {});

  if (_type->type == TokenType::Auto && _auto_name != nullptr) {
    TypeEntry* autoType = new TypeEntry(_auto_name->lexeme.string_lex);
    autoType->type = SymType::Unknown;
    auto t_result = table->addType(_auto_name->lexeme.string_lex, autoType);
    if (t_result != nullptr) {
      auto temp = symbol;
      symbol = Symbol::createError(ErrorType::Redeclaration, "This auto type name is already another type or another anonymous type in this scope.");
      symbol->node = this;
      symbol->sub_type = temp;
      return symbol;
    }

    symbol->custom_type = autoType;
    symbol->custom_type_name = _auto_name->lexeme.string_lex;
  }
  symbol->node = this;
  return symbol;
}

Symbol* CustomType::buildSymbolTable(SymbolTable* table) { 
  symbol = new Symbol("", SymType::Custom, {});
  //TODO add namespacing here. maybe even allow you to pass a "Variable" type in here and do the namespacing in the symbol table functions
  auto found = table->getTypeEntry(_type->_name->lexeme.string_lex);
  if (found == nullptr) {
    auto temp = Symbol::createError(ErrorType::NotFound, "Type not found");
    temp->node = this;
    symbol = temp;
    return symbol;
  }
  symbol->type = found->type;
  symbol->sub_type = found->sub_type;
  symbol->custom_type = found;
  symbol->custom_type_name = found->name;
  symbol->node = this;
  return symbol;
}

Symbol* FuncType::buildSymbolTable(SymbolTable* table) { 
  auto params = new std::vector<Symbol*>();
  for (auto par : *_param_types) {
    par->buildSymbolTable(table);
    params->push_back(par->symbol);
  }

  auto returns = new std::vector<Symbol*>();
  for (auto ret : *_return_types) {
    ret->buildSymbolTable(table);
    returns->push_back(ret->symbol);
  }

  //TODO what if one of the returns or one of the params has an error in it's type?
  symbol = Symbol::createFunction("", params, returns);
  symbol->node = this;
  return symbol;
}

Symbol* ConstType::buildSymbolTable(SymbolTable* table) { 
  _type->buildSymbolTable(table);
  if (_type->symbol->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down the tree.");
    symbol->node = this;
    return symbol;
  }

  symbol = _type->symbol;
  symbol->constant = true;
  symbol->node = this;
  return symbol;
}

Symbol* PointerType::buildSymbolTable(SymbolTable* table) { 
  _type->buildSymbolTable(table);
  if (_type->symbol->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down the tree.");
    symbol->node = this;
    return symbol;
  }

  symbol = _type->symbol;
  symbol->pointer_redirection_level += 1;
  symbol->node = this;
  return symbol;
}

Symbol* ArrayType::buildSymbolTable(SymbolTable* table) { 
  _type->buildSymbolTable(table);
  if (_type->symbol->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down the tree.");
    symbol->node = this;
    return symbol;
  }

  if (_array_length) _array_length->buildSymbolTable(table);
  if (_array_length && !_array_length->symbol->isNumber()) {
    symbol = Symbol::createError(ErrorType::SizeParameterNotNumber, "The size parameter of this array type is not an number.");
    symbol->sub_type = Symbol::createArray("", _type->symbol);
  } else {
    symbol = Symbol::createArray("", _type->symbol);
  }
  symbol->node = this;
  return symbol;
}

Symbol* Parameter::buildSymbolTable(SymbolTable* table) { 
  _type->buildSymbolTable(table);
  if (_type->symbol->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down the tree.");
    symbol->node = this;
    return symbol;
  }
  symbol = _type->symbol;
  symbol->name = _id->lexeme.string_lex;
  symbol->node = this;
  return symbol;
}

Symbol* Program::buildSymbolTable(SymbolTable* table) {
  Symbol* result = nullptr;
  for (auto stmt : *_stmts) {
    auto s_result = stmt->buildSymbolTable(table);
    if (s_result->type == SymType::Error && result == nullptr) {
      result = Symbol::createError(ErrorType::None, "There is an error further down the tree");
    }
  }
  if (result == nullptr) {
    symbol = Symbol::createNone();
    symbol->node = this;
  } else {
    symbol = result;
    symbol->node = this;
  }
  return symbol;
}

Symbol* Macro::buildSymbolTable(SymbolTable* table) { 
  symbol = Symbol::createNone();
  symbol->node = this;
  return symbol;
}

Symbol* CompoundStmt::buildSymbolTable(SymbolTable* table) { 
  Symbol* result = nullptr;
  for (auto stmt : *_stmts) {
    auto s_result = stmt->buildSymbolTable(table);
    if (s_result->type == SymType::Error && result == nullptr) {
      result = Symbol::createError(ErrorType::None, "There is an error further down the tree");
    }
  }
  if (result == nullptr) {
    symbol = Symbol::createNone();
  } else {
    symbol = result;
  }
  symbol->node = this;
  return symbol;
}

Symbol* VarDec::buildSymbolTable(SymbolTable* table) {
  auto t_result = _type->buildSymbolTable(table);
  if (t_result->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down in the tree.");
    symbol->node = this;
    std::cout << "Type error >:(" << std::endl;
    if (_expr) _expr->symbol = Symbol::createNone();
    return symbol;
  }

  symbol = _type->symbol;
  symbol->assignable = true;
  symbol->name = _id->lexeme.string_lex;
  auto result = table->addSymbol(_id->lexeme.string_lex, symbol);
  if (_expr) {
    auto e_result = _expr->buildSymbolTable(table);
    if (e_result->type == SymType::Error) {
      auto temp = Symbol::createError(ErrorType::None, "diublaiusdbf");
      temp->sub_type = symbol;
      temp->node = this;
      symbol = temp;
      return symbol;
    }

    if (!Symbol::typeMatch(symbol, _expr->symbol)) {
      Symbol* orig = symbol;
      symbol = Symbol::createError(ErrorType::LhsRhsTypeMismatch, "The right hand side of the assignement does not have the same type as the left hand side");
      symbol->sub_type = orig;
    }
  }
  if (result != nullptr) {
    symbol = Symbol::createError(ErrorType::Redeclaration, "This variable has already been declared");
    symbol->sub_type = result;
  }
  symbol->node = this;
  return symbol;
}

Symbol* FuncDec::buildSymbolTable(SymbolTable* table) {
  auto child = table->createChildScope();
  auto params = new std::vector<Symbol*>();
  for (auto par : *_params) {
    par->buildSymbolTable(table);
    params->push_back(par->symbol);
    if(par->symbol->type != SymType::Error)
      child->addSymbol(par->symbol->name, par->symbol);
  }

  auto returns = new std::vector<Symbol*>();
  for (auto ret : *_returns) {
    ret->buildSymbolTable(table);
    returns->push_back(ret->symbol);
    if(ret->symbol->type != SymType::Error)
      child->addSymbol(ret->symbol->name, ret->symbol);
  }

  symbol = Symbol::createFunction(_id->lexeme.string_lex, params, returns);
  symbol->node = this;

  auto func_sym = table->addSymbol(_id->lexeme.string_lex, symbol);
  if (func_sym != nullptr) {
    symbol = Symbol::createError(ErrorType::Redeclaration, "An operator function alrady exists for this operator and parameter types.");
    symbol->node = this;
    return symbol;
  }

  auto result = _body->buildSymbolTable(child);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "There was an error in the body of this function.");
    newError->sub_type = symbol;
    symbol = newError;
    symbol->node = this;
  }

  return symbol;
}

Symbol* OpFuncDec::buildSymbolTable(SymbolTable* table) {
  auto child = table->createChildScope();
  auto params = new std::vector<Symbol*>();
  for (auto par : *_params) {
    par->buildSymbolTable(table);
    params->push_back(par->symbol);
    if(par->symbol->type != SymType::Error)
      child->addSymbol(par->symbol->name, par->symbol);
  }

  auto returns = new std::vector<Symbol*>();
  for (auto ret : *_returns) {
    ret->buildSymbolTable(table);
    returns->push_back(ret->symbol);
    if(ret->symbol->type != SymType::Error)
      child->addSymbol(ret->symbol->name, ret->symbol);
  }

  switch(_op->type) {
    //one or two params
    case TokenType::AddOp:
      if (_params->size() < 1 || _params->size() > 2) {
        symbol = Symbol::createError(ErrorType::OpFuncParameterSizeMismatch, "This operator function can only have one or two parameters.");
      }
      break;
    //exactly one param
    case TokenType::Not:
    case TokenType::Question:
    case TokenType::Tilde:
      if (_params->size() != 1) {
        symbol = Symbol::createError(ErrorType::OpFuncParameterSizeMismatch, "This operator function can only have one parameter.");
      }
      break;
    //exactly two params
    case TokenType::LogOp:
    case TokenType::BitOp:
    case TokenType::EqOp:
    case TokenType::RelOp:
    case TokenType::LeftAngle:
    case TokenType::RightAngle:
    case TokenType::Shift:
    case TokenType::MultOp:
    case TokenType::Range:
      if (_params->size() != 2) {
        symbol = Symbol::createError(ErrorType::OpFuncParameterSizeMismatch, "This operator function can only have two parameters.");
      }
      break;
    default:
      break;
  }
  if (symbol != nullptr) {
    symbol->node = this;
    return symbol;
  }

  symbol = Symbol::createFunction(_op->lexeme.string_lex, params, returns);
  symbol->node = this;

  auto func_sym = table->addSymbol(_op->lexeme.string_lex, symbol);
  if (func_sym != nullptr) {
    symbol = Symbol::createError(ErrorType::Redeclaration, "An operator function alrady exists for this operator and parameter types.");
    symbol->node = this;
    return symbol;
  }

  auto result = _body->buildSymbolTable(child);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "There was an error in the body of this function.");
    newError->sub_type = symbol;
    symbol = newError;
    symbol->node = this;
  }
  return symbol;
}

Symbol* CastFuncDec::buildSymbolTable(SymbolTable* table) { 
  auto child = table->createChildScope();
  auto params = new std::vector<Symbol*>();
  for (auto par : *_params) {
    par->buildSymbolTable(table);
    params->push_back(par->symbol);
    if(par->symbol->type != SymType::Error)
      child->addSymbol(par->symbol->name, par->symbol);
  }

  auto returns = new std::vector<Symbol*>();
  for (auto ret : *_returns) {
    ret->buildSymbolTable(table);
    returns->push_back(ret->symbol);
    if(ret->symbol->type != SymType::Error)
      child->addSymbol(ret->symbol->name, ret->symbol);
  }

  if (_params->size() != 1) {
    symbol = Symbol::createError(ErrorType::CastFuncMultipleParams, "A cast function can only have a single parameter.");
    symbol->node = this;
    return symbol;
  }

  if (_returns->size() != 1) {
    symbol = Symbol::createError(ErrorType::CastFuncMultipleReturns, "A cast function can only have a single return.");
    symbol->node = this;
    return symbol;
  }

  _casting_type->buildSymbolTable(table);
  auto a = _casting_type->symbol;
  auto b = (*returns)[0];
  if (!Symbol::typeMatch(a, b)) {
    symbol = Symbol::createError(ErrorType::CastFuncReturnTypeMismatch, "The casting type must match the return type of a cast function.");
    symbol->node = this;
    return symbol; //maybe don't return here
  }

  std::string castName = "cast-" + _casting_type->toCastString();

  symbol = Symbol::createFunction(castName, params, returns);
  symbol->node = this;

  auto func_sym = table->addSymbol(castName, symbol);
  if (func_sym != nullptr) {
    symbol = Symbol::createError(ErrorType::Redeclaration, "A casting function already exists for this set of types in this order.");
    symbol->node = this;
    return symbol;
  }

  auto result = _body->buildSymbolTable(child);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "There was an error in the body of this function.");
    newError->sub_type = symbol;
    symbol = newError;
    symbol->node = this;
  }
  return symbol;
}

Symbol* EnumDec::buildSymbolTable(SymbolTable* table) { 
  auto result = new TypeEntry(_id->lexeme.string_lex);
  _base_type->buildSymbolTable(table);
  result->sub_type = _base_type->symbol;
  auto vtable = new VTable();
  for(auto dec : *_declist) {
    dec->buildVTable(vtable, table);
  }
  result->vtable = vtable;
  table->addType(result->name, result);

  symbol = new Symbol(result->name, SymType::Enum, _base_type->symbol);
  symbol->node = this;
  symbol->custom_type = result;
  symbol->custom_type_name = result->name;
  return symbol;
}

Symbol* PackDec::buildSymbolTable(SymbolTable* table) {
  auto result = new TypeEntry(_id->lexeme.string_lex);
  auto vtable = new VTable();
  for(auto dec : *_declist) {
    dec->buildVTable(vtable, table);
  }
  result->vtable = vtable;
  table->addType(result->name, result);

  symbol = new Symbol(result->name, SymType::Custom, {});
  symbol->node = this;
  symbol->custom_type = result;
  symbol->custom_type_name = result->name;
  return symbol;
}

Symbol* VariantDec::buildSymbolTable(SymbolTable* table) {
  auto result = new TypeEntry(_id->lexeme.string_lex);
  auto vtable = new VTable();
  for(auto dec : *_declist) {
    dec->buildVTable(vtable, table);
  }
  result->vtable = vtable;
  table->addType(result->name, result);

  symbol = new Symbol(result->name, SymType::Variant, {});
  symbol->node = this;
  symbol->custom_type = result;
  symbol->custom_type_name = result->name;
  return symbol;
}

Symbol* IfStmt::buildSymbolTable(SymbolTable* table) {
  _cond->buildSymbolTable(table);
  if (_cond->symbol->type != SymType::Boolean) {
    symbol = Symbol::createError(ErrorType::UnexpectedType, "The condition for if statements must evaluate to a boolean.");
  }

  auto result = _body->buildSymbolTable(table);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "There was an error in the true branch of this if block.");
    newError->sub_type = symbol;
    symbol = newError;
  }

  if (_elseBody) { 
    auto e_result = _elseBody->buildSymbolTable(table);
    if (e_result->type == SymType::Error) {
      if (symbol->errorType == ErrorType::None) {
        symbol->name = "There was an error in both the if and the else branches";
      } else {
        auto newError = Symbol::createError(ErrorType::None, "There was an error in the false branch of this if block.");
        newError->sub_type = symbol;
        symbol = newError;
      }
    }
  }

  if (symbol == nullptr) {
    symbol = Symbol::createNone();
  }

  symbol->node = this;
  return symbol;
}

Symbol* SwitchCase::buildSymbolTable(SymbolTable* table) { 
  if (_case) {
    auto result = _case->buildSymbolTable(table);
    if (result->type == SymType::Error) {
      symbol = Symbol::createError(ErrorType::None, "There was an error in the condition of this switch case");
      symbol->node = this;
      _body->symbol = Symbol::createNone();
      return symbol;
    }
  }

  if (_case && !_case->symbol->computed) {
    symbol = Symbol::createError(ErrorType::RuntimeCaseCondition, "The case condition must be able to be evaluated at compile time. If you are doing calculations in the case condition make sure any variables are determined at compile-time.");
    symbol->node = this;
    return symbol;
  }

  auto switchcase = table->createChildScope("");
  auto result = _body->buildSymbolTable(switchcase);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "There was an error in the body of this case.");
    newError->sub_type = symbol;
    symbol = newError;
  }

  if (symbol == nullptr) {
    symbol = Symbol::createNone();
  }

  symbol->node = this;
  return symbol;
}

Symbol* SwitchStmt::buildSymbolTable(SymbolTable* table) {
  _cond->buildSymbolTable(table);

  Symbol* result = nullptr;
  for (auto scase : *_cases) {
    auto c_result = scase->buildSymbolTable(table);
    if (c_result->type != SymType::Error && !Symbol::typeMatch(scase->symbol, _cond->symbol)) {
      auto orig = scase->symbol;
      scase->symbol = Symbol::createError(ErrorType::UnexpectedType, "The case condition doesn't match the switch condition.");
      scase->symbol->sub_type = orig;
      scase->symbol->node = scase;
    }
    if (c_result->type == SymType::Error && result == nullptr) {
      auto result = Symbol::createError(ErrorType::None, "There was an error in one of the case statements");
      symbol = result;
    }
  }

  if (symbol == nullptr) {
    symbol = Symbol::createNone();
  }
  
  symbol->node = this;
  return symbol;
}

Symbol* WhileStmt::buildSymbolTable(SymbolTable* table) { 
  _cond->buildSymbolTable(table);
  //check for is boolean
  if (!_cond->symbol->isBoolean()) {
    symbol = Symbol::createError(ErrorType::UnexpectedType, "The while condition must evaluate to a boolean.");
    symbol->node = this;
    return symbol;
  }

  auto whilechild = table->createChildScope("");
  auto result = _body->buildSymbolTable(whilechild);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "The body of this while loop has an error in it.");
    newError->sub_type = symbol;
    symbol = newError;
  }

  if (symbol == nullptr) {
    symbol = Symbol::createNone();
  }

  symbol->node = this;
  return symbol;
}

Symbol* ForStmt::buildSymbolTable(SymbolTable* table) {
  auto i_result = _iter->buildSymbolTable(table);
  auto b_result = _by ? _by->buildSymbolTable(table) : nullptr;

  if (i_result->type == SymType::Error || (b_result != nullptr && b_result->type == SymType::Error)) {
    symbol = Symbol::createError(ErrorType::None, "There was an error in the iterator or the by number");
    symbol->node = this;
    _body->symbol = Symbol::createNone();
    return symbol;
  }

  if (!_iter->symbol->isArray()) {
    symbol = Symbol::createError(ErrorType::UnexpectedType, "Iterator must be evaluate to an array type.");
    symbol->node = this;
    _body->symbol = Symbol::createNone();
    return symbol;
  }

  if (_by != nullptr && !_by->symbol->isNumber()) {
    symbol = Symbol::createError(ErrorType::UnexpectedType, "By must evaluate to a number.");
    symbol->node = this;
    _body->symbol = Symbol::createNone();
    return symbol;
  }

  auto forscope = table->createChildScope("");
  auto itersym = _iter->symbol->copy();
  itersym->name = _id->lexeme.string_lex;
  forscope->addSymbol(_id->lexeme.string_lex, itersym);

  auto result = _body->buildSymbolTable(forscope);
  if (result->type == SymType::Error) {
    auto newError = Symbol::createError(ErrorType::None, "The body of this for loop has an error in it.");
    newError->sub_type = symbol;
    symbol = newError;
  }

  if (symbol == nullptr) {
    symbol = Symbol::createNone();
  }

  symbol->node = this;
  return symbol;
}

Symbol* ExprStmt::buildSymbolTable(SymbolTable* table) {
  _expr->buildSymbolTable(table);
  if (_expr->symbol->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There is an error further down the tree");
  } else {
    symbol = Symbol::createNone();
  }
  symbol->node = this;
  return symbol;
}

Symbol* StopStmt::buildSymbolTable(SymbolTable* table) { symbol = Symbol::createNone(); symbol->node = this; return symbol; }

Symbol* BreakStmt::buildSymbolTable(SymbolTable* table) { symbol = Symbol::createNone(); symbol->node = this; return symbol; }

Symbol* ContinueStmt::buildSymbolTable(SymbolTable* table) { symbol = Symbol::createNone(); symbol->node = this; return symbol; }

Symbol* Variable::buildSymbolTable(SymbolTable* table) {
  std::string varname = _name->lexeme.string_lex;
  auto var = table->getVarSymbol(varname);
  if (var == nullptr) {
    symbol = Symbol::createError(ErrorType::UnknownVariable, "This variable could not be found from the current scope.");
  } else {
    symbol = var->copy();
  }
  symbol->node = this;
  return symbol;
}

Symbol* MemberAccess::buildSymbolTable(SymbolTable* table) {
  auto result = _parent->buildSymbolTable(table);
  if (result->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There was an error in the parent of this member access");
    symbol->node = this;
    return symbol;  
  }

  if (result->custom_type == nullptr) {
    symbol = Symbol::createError(ErrorType::NoMemberVariables, "This variable has no member variables.");
  } else {
    auto member = _parent->symbol->custom_type->getMember(_id->lexeme.string_lex);
    if (member == nullptr) {
      symbol = Symbol::createError(ErrorType::TypeDoesNotHaveMember, "This type does not have a member variable with that name.");
    } else {
      symbol = member->copy();
    }
  }

  symbol->node = this;
  return symbol;
}

Symbol* ArrayAccess::buildSymbolTable(SymbolTable* table) {
  auto p_result = _parent->buildSymbolTable(table);
  auto e_result = _expr->buildSymbolTable(table);

  if (p_result->type == SymType::Error || e_result->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "klasjdfhalskdj");
    symbol->node = this;
    return symbol;
  }

  if(p_result->type == SymType::Array) {
    if (e_result->isNumber()) {
      symbol = p_result->sub_type->copy();
    } else {
      symbol = Symbol::createError(ErrorType::UnexpectedType, "Indexes for an iterable type must be a number.");
    }
  } else {
    symbol = Symbol::createError(ErrorType::NotIterableType, "This type is not iterable so it cannot be accessed through array access.");
  }

  symbol->node = this;
  return symbol;
}

Symbol* Call::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createError(ErrorType::UhOh, "Call typecheck not implemented");
  symbol->node = this;
  return symbol;
}

Symbol* Assignment::buildSymbolTable(SymbolTable* table) { 
  symbol = Symbol::createError(ErrorType::UhOh, "Assignment typecheck not implemented");
  symbol->node = this;
  return symbol;
}

Symbol* BinaryExpr::buildSymbolTable(SymbolTable* table) { 
  auto left = _left->buildSymbolTable(table);
  auto right = _right->buildSymbolTable(table);
  if (left->type == SymType::Error || right->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There was an error furter down the tree.");
    symbol->node = this;
    return symbol;
  }

  switch(_op->type) {
    case TokenType::EqOp:
      if (Symbol::typeMatch(left, right)) {
        symbol = Symbol::createBoolean();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::RelOp:
    case TokenType::LeftAngle:
    case TokenType::RightAngle:
      if (left->isNumber() && right->isNumber()) {
        symbol = Symbol::createBoolean();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::Shift:
      if ((left->isNumber() ||
          (left->isArray() && left->sub_type->isNumber()) ||
          (left->isArray() && left->sub_type->isBoolean())) &&
          right->isNumber()) {
        symbol = left->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::LogOp:
      if (left->isBoolean() && right->isBoolean()) {
        symbol = Symbol::createBoolean();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::BitOp:
      if (Symbol::typeMatch(left, right) &&
          (left->isNumber() || left->isBoolean() ||
          (left->isArray() && left->sub_type->isNumber()) ||
          (left->isArray() && left->sub_type->isBoolean()))) {
        symbol = left->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::AddOp:
      if ((left->isNumber() && right->isNumber()) ||
          (left->isString() && right->isString())) {
        symbol = left->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::MultOp:
      if (left->isNumber() && right->isNumber()) {
        symbol = left->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    case TokenType::Range:
      if (left->isNumber() && right->isNumber()) {
        symbol = Symbol::createArray("", left->copy());
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied types.");
      }
      break;
    default:
      symbol = Symbol::createError(ErrorType::UhOh, "We should not have gotten this error. Unknown binary expression operator!!!!");
      break;
  }

  symbol->node = this;
  return symbol;
}

Symbol* UnaryExpr::buildSymbolTable(SymbolTable* table) {
  auto result = _expr->buildSymbolTable(table);
  if (result->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "There was an error further down the tree.");
    symbol->node = this;
    return symbol;
  }

  //TODO make this look up any operator overloaded functions
  switch (_op->type) {
    case TokenType::Not:
      if (result->isBoolean()) {
        symbol = result->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied type.");
      }
      break;
    case TokenType::AddOp:
      if (result->isNumber()) {
        //TODO make this change unsigned numbers to signed numbers (but not backwards)
        symbol = result->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied type.");
      }
      break;
    case TokenType::Tilde: //binary not
      if (result->isNumber() || result->isBoolean() ||
          (result->isArray() && result->sub_type->isNumber()) ||
          (result->isArray() && result->sub_type->isBoolean())) {
        symbol = result->copy();
      } else {
        symbol = Symbol::createError(ErrorType::UnexpectedType, "This operator does not have an implicit or explicit definition for the supplied type.");
      }
      break;
    case TokenType::Question:
      symbol = result->copy();
      if (symbol->pointer_redirection_level < 1) {
        symbol = Symbol::createError(ErrorType::DereferenceNonPointer, "Cannot dereference a non-pointer.");
      } else {
        symbol->pointer_redirection_level -= 1;
      }
      break;
    default:
      symbol = Symbol::createError(ErrorType::UhOh, "We should not have gotten this error. Unknown unary expression operator!!!!");
      break;
  }

  symbol->node = this;
  return symbol;
}

Symbol* Cast::buildSymbolTable(SymbolTable* table) {
  auto t_result = _type->buildSymbolTable(table);
  auto e_result = _expr->buildSymbolTable(table);
  if (t_result->type == SymType::Error || e_result->type == SymType::Error) {
    symbol = Symbol::createError(ErrorType::None, "akshdfl");
    symbol->node = this;
    return symbol;
  }

  if (!Symbol::typeMatch(_type->symbol, _expr->symbol)) {
    std::string castFunc = "cast-" + _type->toCastString();
    auto castSym = table->getFuncSymbol(castFunc, new std::vector { _expr->symbol });
    if (castSym != nullptr) {
      symbol = _type->symbol->copy();
    } else {
      symbol = Symbol::createError(ErrorType::NoCastExists, "No cast exists to the supplied type. Define your own cast function or cast to a different type.");
    }
  }
  else
  {
    symbol = _type->symbol->copy();
  }

  symbol->node = this;
  return symbol;
}

Symbol* IntValue::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createBasic(SymType::U64);
  symbol->node = this;
  return symbol;
}

Symbol* HexValue::buildSymbolTable(SymbolTable* table) {
  std::string value = _value->lexeme.string_lex;
  int byte_length = value.length() % 2 == 0 ? value.length() / 2 : (value.length() + 1) / 2;
  symbol = Symbol::createHex(byte_length);
  symbol->node = this;
  return symbol;
}

Symbol* BoolValue::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createBoolean();
  symbol->node = this;
  return symbol;
}

Symbol* FloatValue::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createBasic(SymType::F128);
  symbol->node = this;
  return symbol;
}

Symbol* StringValue::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createString();
  symbol->node = this;
  return symbol;
}

Symbol* ArrayValue::buildSymbolTable(SymbolTable* table) {
  Symbol* type = nullptr;
  bool valid = true;
  bool error = false;
  for (auto elem : *_elements) {
    auto etype = elem->buildSymbolTable(table);
    if (type == nullptr && etype->type != SymType::Error) {
      type = etype->copy();
    } else if (etype->type == SymType::Error) {
      error = true;
    } else if (!Symbol::typeMatch(etype, type)){
      valid = false;
    }
  }

  if (valid && !error) {
    symbol = Symbol::createArray("", type);
  } else if (error) {
    symbol = Symbol::createError(ErrorType::None, "asldkjfas");
  } else {
    symbol = Symbol::createError(ErrorType::UnexpectedType, "Each element of the array must evaluate to the same type.");
  }

  symbol->node = this;
  return symbol;
}

Symbol* ObjectValue::buildSymbolTable(SymbolTable* table) {
  symbol = Symbol::createNone();
  symbol->node = this;
  return symbol;
}
