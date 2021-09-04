#pragma once

#include <string>
#include <vector>
#include <unordered_map>

enum SymType {
  Error, None, Unknown,
  Custom, Variant, Enum,
  Func, Array, String,
  Boolean, Byte,
  I16, I32, I64,
  S16, S32, S64,
  U16, U32, U64,
  F16, F32, F64,
};

class SymbolType { //these are variables
public:
  SymType type = SymType::Unknown;
  SymType sub_type = SymType::None; //used for enum amd for auto determined type
  std::string custom_type_name = "";

  std::vector<SymbolType*>* param_types = {}; //will be used for arrays and functions
  std::vector<SymbolType*>* return_types = {};

  bool assignable = false;
  bool constant = false;
  int pointer_redirection_level = 0; //0 is not a pointer, 1 is pointer, 2 is pointer to a pointer, etc etc

  SymbolType(SymType t, SymType st) : type(t), sub_type(st) {};

  static SymbolType* createFunction(std::vector<SymbolType*>*, std::vector<SymbolType*>*) { return nullptr; };
  static SymbolType* createArray(std::vector<SymbolType*>*) { return nullptr; };
};

class CustomTypeEntry { //these are for types
public:
  SymType type = SymType::Custom; //named auto types and enums will have their type determined through here I believe
  std::unordered_map<std::string, SymbolType*>* member_types = {};

  CustomTypeEntry() {
    member_types = new std::unordered_map<std::string, SymbolType*>();
  };
};

class SymbolTable {
public:
  SymbolTable* parent_scope = {};
  std::vector<SymbolTable*>* sub_scopes = {};

  std::string namespace_name = "";
  std::unordered_map<std::string, std::vector<SymbolType*>*>* current_scope = {};
  std::unordered_map<std::string, CustomTypeEntry*>* type_table = {};

  SymbolTable(SymbolTable* pscope, std::string name) : parent_scope(pscope), namespace_name(name) {
    sub_scopes = new std::vector<SymbolTable*>();
    current_scope = new std::unordered_map<std::string, std::vector<SymbolType*>*>();
    type_table = new std::unordered_map<std::string, CustomTypeEntry*>();
  }

  //creates a child scope on this scope and sets up the connections properly
  SymbolTable* createChildScope(std::string name) { return nullptr; };

  //add a symbol to the current scope. Return nullptr on success and return the conflicting symbol on redefinition
  SymbolType* addSymbol(std::string name, SymbolType* type) { return nullptr; };
  //add a custom type to the current scope. Return nullptr on success and return the conflicting type on redefinition
  CustomTypeEntry* addType(std::string name, CustomTypeEntry* type) { return nullptr; };
};
