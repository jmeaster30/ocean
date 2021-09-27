#include <iostream>
#include <vector>

#include "helper.hpp"

#include "ast.hpp"

#include "ocean.tab.hpp"

extern FILE* yyin;
Program* root;

int main(int argc, char** argv) {
  ++argv, --argc; //skip over the program name
  if(argc > 0){
    yyin = fopen(argv[0], "r");
    if(yyin == nullptr)
    {
      std::cout << "The file '" << argv[0] << "' was not able to be opened" << std::endl;
      return 1;
    }
  }else{
    std::cout << "Please supply a source file." << std::endl;
    return 1;
  }

  yyparse();
  debug("Done Parsing...");
  if(root == nullptr){
    std::cout << "There was an issue with parsing this file. The parser returned a null ast root." << std::endl;
    return 1;
  } else {
    adebug(root->toString());
  }

  debug("This file contained valid ocean source code");
  Program* main_root = root; //save the main root node of the ast so it doesn't get messed up by parsing other files
  SymbolTable* table = new SymbolTable(nullptr, "global");
  auto final_type = main_root->buildSymbolTable(table);

  if(final_type->type == SymType::Error) {
    auto error_list = new std::vector<std::string>();
    main_root->getErrors(error_list);
    std::cout << "There were " << error_list->size() << " errors :(" << std::endl; 
    for (auto error : *error_list) {
      std::cout << error << std::endl;
    }
  } else {
    std::cout << "There were no type checking errors!" << std::endl;
  }

  return 0;
}