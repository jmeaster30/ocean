#include <iostream>
#include <vector>

#include "ast.hpp"

#include "ocean.tab.hpp"

extern FILE* yyin;
Program* root;

int main(int argc, char** argv) {
  ++argv, --argc; //skip over the program name argument
  if(argc > 0){
    yyin = fopen(argv[0], "r");
    if(yyin == nullptr)
    {
      std::cout << "The file '" << argv[0] << "' was not able to be opened" << std::endl;
      return 1;
    }
  }else{
    yyin = stdin;
  }

  yyparse();
  std::cout << "Done Parsing" << std::endl;
  if(root == nullptr){
    std::cout << "Null Root. THERE WAS A PROBLEM" << std::endl;
  } else {
    root->print();
  }

  //program* main_root = root; //save the main root node of the ast so it doesn't get messed up by parsing other files

  
  return 0;
}