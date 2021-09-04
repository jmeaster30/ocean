#pragma once

#include <string>
#include <vector>
#include <unordered_map>

enum SymType {
  Error, None, Unknown,
  Custom, Variant, Enum,
  Func, Array, Void, 
  Number, String, Boolean, Byte,
  I16, I32, I64,
  S16, S32, S64,
  U16, U32, U64,
  F16, F32, F64,
};

class SymbolType {
public:
  SymType type = SymType::Unknown;
  SymType sub_type = SymType::None; //used for enum amd for auto determined type
  std::string custom_type_name = "";

  std::vector<SymbolType*>* param_types = {}; //will be used for arrays and variants also
  std::vector<SymbolType*>* return_types = {};

  bool assignable = false;
  bool constant = false; 
  int pointer_redirection_level = 0; //0 is not a pointer, 1 is pointer, 2 is pointer to a pointer, etc etc

  SymbolType() {};
};

class CustomTypeEntry {
public:
  SymType type = SymType::Custom; //named auto types will have their type determined through here I believe
  std::unordered_map<std::string, SymbolType*>* member_types = {};
};

class SymbolTable {
public:
  SymbolTable* parent_scope = {};
  std::vector<SymbolTable*>* sub_scopes = {};

  std::unordered_map<std::string, std::vector<SymbolType*>*>* current_scope = {};
  std::unordered_map<std::string, CustomTypeEntry*>* type_table = {};

  SymbolTable() {};
};
