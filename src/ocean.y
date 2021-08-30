%{
#include <stdio.h>
#include <vector>
#include <iostream>

#include "token.hpp"
#include "ast.hpp"

extern void yyerror(const char* s);
extern "C" int yylex();

//extern "C" void macro_start(char* in);

extern FILE* yyin;
extern char* yytext;
extern int line_number;
extern int column_number;

extern Program* root;

%}

%locations
%define parse.error verbose

%union {
  Token* token;
  Program* program;
  Statement* statement;
  Parameter* parameter;
  Expression* expression;
  VarType* vartype;
  Var* var;
  Variable* nvar;
  Declaration* declaration;
  CompoundStmt* compound;

  std::vector<Statement*>* stmtlist;
  std::vector<SwitchCase*>* switchcaselist;
  std::vector<Parameter*>* paramlist;
  std::vector<Declaration*>* declist;
  std::vector<Expression*>* arglist;
}

%token ENDOFFILE 0 "end of file"
%token <token> TYPE AUTO VOID
%token <token> ENUM PACK VARIANT
%token <token> STOP BREAK CONTINUE IF ELSE WHILE FOR IN BY
%token <token> SWITCH RANGE

%token <token> QUESTION TILDE NEWLINE CONST
%token <token> ARROW DOT NOT COMMA COLON SEMICOLON DUBCOLON
%token <token> PAREN_OPEN PAREN_CLOSED
%token <token> SQUARE_OPEN SQUARE_CLOSED
%token <token> BRACE_OPEN BRACE_CLOSED
%token <token> ANGLE_OPEN ANGLE_CLOSED
%token <token> BOOLVAL
%token <token> INTVAL
%token <token> HEXVAL
%token <token> FLOATVAL
%token <token> STRINGVAL
%token <token> IDENTIFIER

%token <token> MACRO

%token <token> SHIFT
%token <token> EQUIV
%token <token> RELAT
%token <token> BITWISE
%token <token> LOGIC
%token <token> ADD
%token <token> MULT

%token <token> ASSIGN

%type <program> PROG
%type <stmtlist> STMT_LIST
%type <statement> STMT IFSTMT FELSE SWITCHSTMT WHILELOOP FORLOOP ASSIGNMENT
%type <switchcaselist> SWITCHCASELIST SWITCHBODY
%type <declist> DEC_LIST FDEC ENUM_LIST FENUM OBJARGS FOBJARGS
%type <paramlist> PARAMS FPARAMS
%type <parameter> PARAM
%type <expression> EXPR BITWISER EQUIVALENCE COMPARATIVE SHIFTER ADDITIVE MULTIPLICATIVE RANGER UNARY PRIMARY POSTFIX ARRAYVAL
%type <vartype> VARTYPE
%type <var> VAR
%type <nvar> NVAR
%type <arglist> ARGS FARGS
%type <declaration> DEC
%type <compound> CMPD

%%

PROG : STMT_LIST { $$ = new Program($1); root = $$; }
     | ENDOFFILE { $$ = new Program(new std::vector<Statement*>()); root = $$; }
     ;

STMT_LIST : STMT_LIST NEWLINE { $$ = $1; }
          | STMT_LIST STMT LINEEND  { $$ = $1; $$->push_back($2); }
          | STMT_LIST error NEWLINE { $$ = $1; yyclearin; }
          | NEWLINE { $$ = new std::vector<Statement*>(); }
          | STMT LINEEND  { $$ = new std::vector<Statement*>(); $$->push_back($1); }
          | error NEWLINE { $$ = new std::vector<Statement*>(); yyclearin; }
          ;

LINEEND : NEWLINE
        | ENDOFFILE
        ;

STMT : MACRO { $$ = new Macro($1); }
     | DEC { $$ = $1; }
     | IFSTMT { $$ = $1; }
     | SWITCHSTMT { $$ = $1; }
     | WHILELOOP { $$ = $1; }
     | FORLOOP { $$ = $1; }
     | ASSIGNMENT { $$ = $1; }
     | CMPD { $$ = $1; }
     | STOP { $$ = new StopStmt($1); }
     | BREAK { $$ = new BreakStmt($1); }
     | CONTINUE { $$ = new ContinueStmt($1); }
     ;

SWITCHSTMT : SWITCH EXPR SWITCHBODY { $$ = new SwitchStmt($1, $2, $3); }
           ;

SWITCHBODY : BRACE_OPEN SWITCHCASELIST BRACE_CLOSED { $$ = $2; }
           ;

SWITCHCASELIST : SWITCHCASELIST EXPR ARROW CMPD {
                    $$ = $1;  
                    SwitchCase* switchCase = new SwitchCase($2, $4);
                    $$->push_back(switchCase);
               }
               | SWITCHCASELIST NEWLINE { $$ = $1; }
               | { $$ = new std::vector<SwitchCase*>(); }
               ;

IFSTMT : IF EXPR CMPD FELSE { $$ = new IfStmt($1, $2, $3, $4); }
       ;

FELSE : ELSE CMPD { $$ = $2; }
      | ELSE IFSTMT { $$ = $2; }
      | { $$ = nullptr; }
      ;

WHILELOOP : WHILE EXPR CMPD { $$ = new WhileStmt($1, $2, $3); }
          ;

FORLOOP : FOR IDENTIFIER IN EXPR CMPD { $$ = new ForStmt($1, $2, $4, nullptr, $5); } 
        | FOR IDENTIFIER IN EXPR BY EXPR CMPD { $$ = new ForStmt($1, $2, $4, $6, $7); }
        ;

CMPD : BRACE_OPEN STMT_LIST BRACE_CLOSED { $$ = new CompoundStmt($2); }
     ;

DEC : IDENTIFIER COLON VARTYPE { $$ = new VarDec($1, $3, nullptr); }
    | IDENTIFIER COLON VARTYPE ASSIGN EXPR { $$ = new VarDec($1, $3, $5); }
    | IDENTIFIER COLON PAREN_OPEN PARAMS PAREN_CLOSED ARROW PAREN_OPEN PARAMS PAREN_CLOSED CMPD {
          $$ = new FuncDec($1, $4, $8, $10);
    }
    | ENUM IDENTIFIER FENUM { $$ = new EnumDec($1, $2, $3); }
    | PACK IDENTIFIER FDEC { $$ = new PackDec($1, $2, $3); }
    | VARIANT IDENTIFIER FDEC { $$ = new VariantDec($1, $2, $3); }
    ;

FDEC : BRACE_OPEN DEC_LIST BRACE_CLOSED { $$ = $2; }
     ;

DEC_LIST : DEC_LIST DEC SEMICOLON { $$ = $1; $$->push_back($2); }
         | DEC_LIST DEC NEWLINE { $$ = $1; $$->push_back($2); }
         | DEC_LIST NEWLINE { $$ = $1; }
         | { $$ = new std::vector<Declaration*>(); }
         ;

FENUM : BRACE_OPEN ENUM_LIST BRACE_CLOSED { $$ = $2; }

ENUM_LIST : ENUM_LIST IDENTIFIER COLON EXPR SEMICOLON { $$ = $1; $$->push_back(new VarDec($2, nullptr, $4)); }
          | ENUM_LIST IDENTIFIER COLON EXPR NEWLINE { $$ = $1; $$->push_back(new VarDec($2, nullptr, $4)); }
          | ENUM_LIST IDENTIFIER SEMICOLON { $$ = $1; $$->push_back(new VarDec($2, nullptr, nullptr)); }
          | ENUM_LIST IDENTIFIER NEWLINE { $$ = $1; $$->push_back(new VarDec($2, nullptr, nullptr)); }
          | ENUM_LIST NEWLINE { $$ = $1; }
          | { $$ = new std::vector<Declaration*>(); }
          ;

PARAMS : FPARAMS { $$ = $1; }
       | { $$ = new std::vector<Parameter*>(); }
       ;

FPARAMS : FPARAMS COMMA PARAM { $$ = $1; $$->push_back($3); } 
        | PARAM { $$ = new std::vector<Parameter*>(); $$->push_back($1); }
        ;

PARAM : IDENTIFIER COLON VARTYPE { $$ = new Parameter($1, $3); }
      ;

ASSIGNMENT : VAR ASSIGN EXPR { $$ = new ExprStmt(new Assignment($1, $2, $3)); }
           | EXPR { $$ = new ExprStmt($1); }
           ;

EXPR : BITWISER LOGIC EXPR { $$ = new BinaryExpr($2, $1, $3); }
     | BITWISER { $$ = $1; }
     ;

BITWISER : BITWISER BITWISE EQUIVALENCE { $$ = new BinaryExpr($2, $1, $3); }
         | EQUIVALENCE { $$ = $1; }
         ;
     
EQUIVALENCE : EQUIVALENCE EQUIV COMPARATIVE { $$ = new BinaryExpr($2, $1, $3); }
            | COMPARATIVE { $$ = $1; }
            ;  

COMPARATIVE : COMPARATIVE RELAT SHIFTER { $$ = new BinaryExpr($2, $1, $3); }
            | COMPARATIVE ANGLE_OPEN SHIFTER { $$ = new BinaryExpr($2, $1, $3); }
            | COMPARATIVE ANGLE_CLOSED SHIFTER { $$ = new BinaryExpr($2, $1, $3); }
            | SHIFTER { $$ = $1; }
            ;

SHIFTER : SHIFTER SHIFT ADDITIVE { $$ = new BinaryExpr($2, $1, $3); }
        | ADDITIVE { $$ = $1; }
        ;

ADDITIVE : ADDITIVE ADD MULTIPLICATIVE { $$ = new BinaryExpr($2, $1, $3); }
         | MULTIPLICATIVE { $$ = $1; }
         ;

MULTIPLICATIVE : MULTIPLICATIVE MULT RANGER { $$ = new BinaryExpr($2, $1, $3); }
               | RANGER { $$ = $1; }
               ;

RANGER : RANGER RANGE UNARY { $$ = new BinaryExpr($2, $1, $3); }
       | UNARY { $$ = $1; }
       ;

UNARY : NOT UNARY { $$ = new UnaryExpr($1, $2); }
      | ADD UNARY { $$ = new UnaryExpr($1, $2); }
      | TILDE UNARY { $$ = new UnaryExpr($1, $2); }
      | POSTFIX { $$ = $1; }
      ;

POSTFIX : POSTFIX QUESTION { $$ = new UnaryExpr($2, $1); }
        | PRIMARY { $$ = $1; }
        ;

PRIMARY : VAR { $$ = $1; }
        | PAREN_OPEN EXPR PAREN_CLOSED { $$ = $2; }
        | INTVAL { $$ = new IntValue($1); }
        | HEXVAL { $$ = new HexValue($1); }
        | BOOLVAL { $$ = new BoolValue($1); }
        | FLOATVAL { $$ = new FloatValue($1); }
        | STRINGVAL { $$ = new StringValue($1); }
        | ARRAYVAL { $$ = $1; }
        | PAREN_OPEN OBJARGS PAREN_CLOSED { $$ = new ObjectValue($2); }
        ;

VARTYPE : TYPE { $$ = new BaseType($1, nullptr); }
        | AUTO { $$ = new BaseType($1, nullptr); }
        | AUTO ANGLE_OPEN IDENTIFIER ANGLE_CLOSED { $$ = new BaseType($1, $3); }
        | VOID { $$ = new BaseType($1, nullptr); }
        | IDENTIFIER { $$ = new BaseType($1, nullptr); }
        | VARTYPE CONST { $$ = new ConstType($1); }
        | VARTYPE TILDE { $$ = new PointerType($1); }
        | VARTYPE SQUARE_OPEN EXPR SQUARE_CLOSED { $$ = new ArrayType($1, $3); }
        ;

NVAR : IDENTIFIER DUBCOLON NVAR { $$ = new Variable($1, $3); }
     | IDENTIFIER { $$ = new Variable($1, nullptr); }
     ;

VAR : VAR DOT IDENTIFIER { $$ = new MemberAccess($1, $3); }
    | VAR SQUARE_OPEN EXPR SQUARE_CLOSED { $$ = new ArrayAccess($1, $3); }
    | VAR PAREN_OPEN ARGS PAREN_CLOSED { $$ = new Call($1, $3); }
    | NVAR { $$ = $1; } 
    ;

ARRAYVAL : SQUARE_OPEN ARGS SQUARE_CLOSED { $$ = new ArrayValue($2); }
         ;

ARGS : FARGS { $$ = $1; }
     | { $$ = new std::vector<Expression*>(); }
     ;

FARGS : FARGS COMMA EXPR { $$ = $1; $$->push_back($3); }
      | EXPR { $$ = new std::vector<Expression*>(); $$->push_back($1); }
      ;

OBJARGS : FOBJARGS { $$ = $1; }
        | { $$ = new std::vector<Declaration*>(); }
        ;

FOBJARGS : FOBJARGS COMMA IDENTIFIER COLON EXPR { $$ = $1; $$->push_back(new VarDec($3, nullptr, $5)); }
         | IDENTIFIER COLON EXPR { $$ = new std::vector<Declaration*>(); $$->push_back(new VarDec($1, nullptr, $3)); }
         ;
%%
