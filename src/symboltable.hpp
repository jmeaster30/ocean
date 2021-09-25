#pragma once

#include <string>
#include <vector>
#include <unordered_map>

//forward declarations
class AstNode;
class TypeEntry;

enum class SymType {
  Error, None, Unknown, Auto,
  Custom, Variant, Enum,
  Func, Array, String,
  Boolean, Byte,
  I16, I32, I64,
  S16, S32, S64,
  U16, U32, U64,
  F32, F64, F128,
};

enum class ErrorType {
  None, Redeclaration, NotFound, SizeParameterNotNumber,
  LhsRhsTypeMismatch, CastFuncMultipleParams, CastFuncMultipleReturns,
  CastFuncReturnTypeMismatch, UnexpectedType, RuntimeCaseCondition,
  NoCastExists, UnknownVariable, NoMemberVariables, TypeDoesNotHaveMember,
  NotIterableType,
};

class Symbol { //these are variables
public:
  std::string name = "";

  SymType type = SymType::Unknown;
  Symbol* sub_type = {}; //used for enum amd for auto determined type
  std::string custom_type_name = "";
  TypeEntry* custom_type = {};

  std::vector<Symbol*>* params = {};
  std::vector<Symbol*>* returns = {};

  ErrorType errorType = ErrorType::None;
  AstNode* node = {};

  bool assignable = false;
  bool constant = false;
  bool computed = false;
  int pointer_redirection_level = 0; //0 is not a pointer, 1 is pointer, 2 is pointer to a pointer, etc etc

  Symbol(std::string n, SymType t, Symbol* st) : name(n), type(t), sub_type(st) {};

  //exact symbol match
  bool operator==(Symbol other);
  bool operator!=(Symbol other);

  bool isNumber();
  bool isBoolean();
  bool isArray();

  Symbol* copy();

  //only type match / auto type handling
  static bool typeMatch(Symbol*, Symbol*);

  static Symbol* createFunction(std::string name, std::vector<Symbol*>* params, std::vector<Symbol*>* returns);
  static Symbol* createArray(std::string name, Symbol* subtype);
  static Symbol* createError(ErrorType type, std::string message);
  static Symbol* createNone();
  static Symbol* createByte();
  static Symbol* createBoolean();
  static Symbol* createBasic(SymType i);
  static Symbol* createString();
  static Symbol* createHex(int length);
};

class VTable {
public:
  std::unordered_map<std::string, Symbol*>* declaration_list = {};

  VTable() {
    declaration_list = new std::unordered_map<std::string, Symbol*>();
  }

  Symbol* addDeclaration(std::string name, Symbol* symbol);
};

class TypeEntry { //these are for types
public:
  std::string name = "";
  SymType type = SymType::Custom;
  Symbol* sub_type = {};
  VTable* vtable = {};

  TypeEntry(std::string n) : name(n) {
    vtable = new VTable();
  };

  bool matchMembers(std::unordered_map<std::string, Symbol*>* members);
  Symbol* getMember(std::string);

  //useful for creating the custom types we get from multi return functions
  static TypeEntry* createFromTypes(std::string name, std::vector<Symbol*>* types);
};

class SymbolTable {
public:
  SymbolTable* parent_scope = {};
  std::vector<SymbolTable*>* sub_scopes = {};

  std::string namespace_name = "";
  std::unordered_map<std::string, std::vector<Symbol*>*>* current_scope = {};
  std::unordered_map<std::string, TypeEntry*>* type_table = {};

  SymbolTable(SymbolTable* pscope, std::string name) : parent_scope(pscope), namespace_name(name) {
    sub_scopes = new std::vector<SymbolTable*>();
    current_scope = new std::unordered_map<std::string, std::vector<Symbol*>*>();
    type_table = new std::unordered_map<std::string, TypeEntry*>();
  }

  //creates a child scope on this scope and sets up the connections properly
  SymbolTable* createChildScope();
  SymbolTable* createChildScope(std::string name);

  //add a symbol to the current scope. Return nullptr on success and return the conflicting symbol on redefinition
  Symbol* addSymbol(std::string name, Symbol* type);
  //add a custom type to the current scope. Return nullptr on success and return the conflicting type on redefinition
  TypeEntry* addType(std::string name, TypeEntry* type);

  //a variable and a function can have the same name but a variable cannot share the name as another variable
  Symbol* getVarSymbol(std::string name);
  //same here but functions can be overloaded so they are matched by their parameters
  Symbol* getFuncSymbol(std::string name, std::vector<Symbol*>* params);
  //there can only be one type per name 
  TypeEntry* getTypeEntry(std::string name);
  //this is useful for searching up a type by the member values so we can get the type of anonymous objects
  TypeEntry* getTypeEntry(std::unordered_map<std::string, Symbol*>* members);
};
