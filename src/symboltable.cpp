#include "symboltable.hpp"

bool Symbol::operator==(Symbol other){
  if (type != other.type || 
      *sub_type != *other.sub_type ||
      pointer_redirection_level != other.pointer_redirection_level ||
      custom_type_name != other.custom_type_name ||
      name != other.name) {
    return false;
  } else {
    if (params == nullptr && other.params == nullptr) return true;
    if (params == nullptr || other.params == nullptr) return false;
    if (params->size() != other.params->size()) return false;

    for (int i = 0; i < params->size(); i++) {
      if (*(*params)[i] != *(*other.params)[i]) {
        return false;
      }
    }
  }
  return true;
};

bool Symbol::operator!=(Symbol other){
  return !(*this == other);
}

bool Symbol::typeMatch(Symbol* first, Symbol* second) {
  return false;
}

bool Symbol::isNumber() {
  return type == SymType::I16 || type == SymType::I32 || type == SymType::I64 ||
         type == SymType::S16 || type == SymType::S32 || type == SymType::S64 ||
         type == SymType::U16 || type == SymType::U32 || type == SymType::U64 ||
         type == SymType::F32 || type == SymType::F64 || type == SymType::F128 ||
         type == SymType::Byte || (sub_type && sub_type->isNumber());
}

bool Symbol::isBoolean() {
  return type == SymType::Boolean || (sub_type && sub_type->isBoolean());
}

bool Symbol::isArray() {
  return type == SymType::Array || type == SymType::String || (sub_type && sub_type->isArray());
}

Symbol* Symbol::copy() {
  auto sym = new Symbol(name, type, sub_type->copy());
  sym->custom_type_name = custom_type_name;
  sym->custom_type = custom_type;

  if (params) {
    sym->params = new std::vector<Symbol*>();
    for (auto p : *params) {
      sym->params->push_back(p->copy());
    }
  }
  if (returns) {
    sym->returns = new std::vector<Symbol*>();
    for (auto r : *returns) {
      sym->returns->push_back(r->copy());
    }
  }

  sym->errorType = errorType;
  sym->assignable = assignable;
  sym->constant = constant;
  sym->computed = computed;
  sym->pointer_redirection_level = pointer_redirection_level;

  return sym;
}

Symbol* Symbol::createFunction(std::string name, std::vector<Symbol*>* params, std::vector<Symbol*>* returns) {
  auto symbol = new Symbol(name, SymType::Func, {});
  symbol->params = params;
  symbol->returns = returns;
  return symbol;
}

Symbol* Symbol::createArray(std::string name, Symbol* subtype) {
  return new Symbol(name, SymType::Array, subtype);
}

Symbol* Symbol::createError(ErrorType type, std::string name) {
  auto symbol = new Symbol(name, SymType::Error, {});
  symbol->errorType = type;
  return symbol;
}

Symbol* Symbol::createNone() {
  return new Symbol("", SymType::None, {});
}

Symbol* Symbol::createByte() {
  return new Symbol("", SymType::Byte, {});
}

Symbol* Symbol::createBoolean() {
  return new Symbol("", SymType::Boolean, {});
}

Symbol* Symbol::createBasic(SymType i) {
  return new Symbol("", i, {});
}

Symbol* Symbol::createString() {
  return new Symbol("", SymType::String, Symbol::createArray("", Symbol::createByte()));
}

Symbol* Symbol::createHex(int length) {
  if (length > 1) {
    return Symbol::createArray("", Symbol::createByte());
  }
  return Symbol::createByte();
}

Symbol* VTable::addDeclaration(std::string name, Symbol* symbol) {
  Symbol* result = nullptr;
  auto found = declaration_list->find(name);
  if (found == declaration_list->end()) {
    (*declaration_list)[name] = symbol;
  }
  return result;
}

TypeEntry* TypeEntry::createFromTypes(std::string name, std::vector<Symbol*>* types) {
  auto entry = new TypeEntry(name);
  for (auto sym : *types) {
    entry->vtable->addDeclaration(sym->name, sym);
  }
  return entry;
}

Symbol* TypeEntry::getMember(std::string name) {
  if (vtable == nullptr) return nullptr;
  auto result = vtable->declaration_list->find(name);
  if(result == vtable->declaration_list->end()) return nullptr;
  return result->second;
}


bool TypeEntry::matchMembers(std::unordered_map<std::string, Symbol*>* members) {
  bool result = true;
  for (auto&[name, entry] : *members) {
    auto found = vtable->declaration_list->find(name);
    if (found == vtable->declaration_list->end() || *entry != *(found->second)) {
      result = false;
      break;
    }
  }
  return result;
}

SymbolTable* SymbolTable::createChildScope() {
  return createChildScope("");
}

SymbolTable* SymbolTable::createChildScope(std::string name) {
  auto child = new SymbolTable(this, name);
  sub_scopes->push_back(child);
  return child;
}

Symbol* SymbolTable::addSymbol(std::string name, Symbol* type) {
  Symbol* result = nullptr;
  auto found = current_scope->find(name);
  if (found == current_scope->end()) {
    //add new symbol
    auto group = new std::vector<Symbol*>();
    group->push_back(type);
    (*current_scope)[name] = group;
  } else {
    auto group = found->second;
    for(auto etype : *group) {
      if (*etype == *type) {
        result = etype;
        break;
      }
    }
  }
  return result;
}

TypeEntry* SymbolTable::addType(std::string name, TypeEntry* type) {
  auto found = type_table->find(name);
  if (found == type_table->end()) {
    (*type_table)[name] = type;
  }
  return found->second;
}

Symbol* SymbolTable::getVarSymbol(std::string name) {
  Symbol* result = nullptr;
  auto found = current_scope->find(name);
  if (found != current_scope->end()) {
    for (auto sym : *found->second) {
      if (sym->type != SymType::Func) {
        result = sym;
        break;
      }
    }
  }
  if (result == nullptr && parent_scope != nullptr) {
    //we couldn't find it in this scope so check the next higher one
    result = parent_scope->getVarSymbol(name);
  }
  return result;
}

Symbol* SymbolTable::getFuncSymbol(std::string name, std::vector<Symbol*>* params) {
  Symbol* result = nullptr;
  auto found = current_scope->find(name);
  if (found != current_scope->end()) {
    for (auto sym : *found->second) {
      if (sym->type != SymType::Func) continue;
      if (sym->params->size() == params->size()) {
        bool isMatch = true;
        for (int i = 0; i < params->size(); i++) {
          auto a = (*params)[i];
          auto b = (*sym->params)[i];
          if (a->type != b->type ||
              *(a->sub_type) != *(b->sub_type) ||
              a->custom_type_name != b->custom_type_name ||
              a->pointer_redirection_level != b->pointer_redirection_level) {
            isMatch = false;
            break;
          }
        }
        if (isMatch) {
          result = sym;
          break;
        }
      }
    }
  }
  if (result == nullptr && parent_scope != nullptr) {
    result = parent_scope->getVarSymbol(name);
  }
  return result;
}

TypeEntry* SymbolTable::getTypeEntry(std::string name) {
  TypeEntry* result = nullptr;
  auto found = type_table->find(name);
  if (found != type_table->end()) {
    result = found->second;
  }
  if (result == nullptr && parent_scope != nullptr) {
    //we couldn't find it in this scope so check the next higher one
    result = parent_scope->getTypeEntry(name);
  }
  return result;
}

TypeEntry* SymbolTable::getTypeEntry(std::unordered_map<std::string, Symbol*>* members) {
  TypeEntry* result = nullptr;
  
  for(auto&[name, entry] : *type_table) {
    if (entry->matchMembers(members)) {
      result = entry;
      break;
    }
  }

  if (result == nullptr && parent_scope != nullptr) {
    //we couldn't find it in this scope so check the next higher one
    result = parent_scope->getTypeEntry(members);
  }
  return result;
}
