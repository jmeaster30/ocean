%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "helper.hpp"

#include <list>
#include <string>
#include "token.hpp"
#include "ast.hpp"

#include "ocean.tab.hpp"

#define ARRAY 258
#define yyterminate() return ENDOFFILE

//typedef union YYSTYPE YYSTYPE;
//extern YYSTYPE yylval; //passes the value to parser
extern "C" int yylex();

int line_number = 1; 
int column_number = 1;
int next_line_number = 1;
int next_column_number = 1;

void UpdateLineColumnNumber();
void Drop();
void Drop(const char* message);
void InvalidToken();
int ProcessToken(const char* name, int token_parser_id, TokenType token_type, int value_type);
%}

/* Rules Section  */
%%

  /******* KEYWORDS *********/
"bool"     { return ProcessToken("bool",   TYPE, TokenType::Type, 0); }
"byte"     { return ProcessToken("byte",   TYPE, TokenType::Type, 0); }
"i16"      { return ProcessToken("i16",    TYPE, TokenType::Type, 0); }
"i32"      { return ProcessToken("i32",    TYPE, TokenType::Type, 0); }
"i64"      { return ProcessToken("i64",    TYPE, TokenType::Type, 0); }
"s16"      { return ProcessToken("s16",    TYPE, TokenType::Type, 0); }
"s32"      { return ProcessToken("s32",    TYPE, TokenType::Type, 0); }
"s64"      { return ProcessToken("s64",    TYPE, TokenType::Type, 0); }
"f16"      { return ProcessToken("f16",    TYPE, TokenType::Type, 0); }
"f32"      { return ProcessToken("f32",    TYPE, TokenType::Type, 0); }
"f64"      { return ProcessToken("f64",    TYPE, TokenType::Type, 0); }
"auto"     { return ProcessToken("auto",   AUTO, TokenType::Auto, 0); } 
"void"     { return ProcessToken("void",   VOID, TokenType::Void, 0); }
"func"     { return ProcessToken("func",   FUNC, TokenType::Func, 0); }
"op"       { return ProcessToken("op",     OP,   TokenType::Op,   0); }

"const"    { return ProcessToken("const",    CONST,    TokenType::Const, 0); }
"enum"     { return ProcessToken("enum",     ENUM,     TokenType::Enum, 0); }
"pack"     { return ProcessToken("pack",     PACK,     TokenType::Pack, 0); }
"variant"  { return ProcessToken("variant",  VARIANT,  TokenType::Variant, 0); }

"default"  { return ProcessToken("DEFAULT",  DEFAULT,  TokenType::Default, 0); }
"stop"     { return ProcessToken("STOP",     STOP,     TokenType::Stop, 0); }
"break"    { return ProcessToken("BREAK",    BREAK,    TokenType::Break, 0); }
"continue" { return ProcessToken("CONTINUE", CONTINUE, TokenType::Continue, 0); }
"if"       { return ProcessToken("IF",       IF,       TokenType::If, 0); }
"else"     { return ProcessToken("ELSE",     ELSE,     TokenType::Else, 0); }
"while"    { return ProcessToken("WHILE",    WHILE,    TokenType::While, 0); }
"for"      { return ProcessToken("FOR",      FOR,      TokenType::For, 0); }
"in"       { return ProcessToken("IN",       IN,       TokenType::In, 0); }
"by"       { return ProcessToken("BY",       BY,       TokenType::By, 0); }
"switch"   { return ProcessToken("SWITCH",   SWITCH,   TokenType::Switch, 0); } 


  /******* WHITESPACE ********/
"\n" {
  return ProcessToken("NEWLINE", NEWLINE, TokenType::Newline, 0);
}

[ \t\r\v\f] {
  Drop("WHITESPACE");
};

  /******** MACRO / COMMENTS ********/
\@.*$ { return ProcessToken("MACRO", MACRO, TokenType::Macro, 0); }
\{("asm"|"ast")?\@[^\@]*\@+(?:[^\@\}][^\@]*\@+)*\} { return ProcessToken("MACRO", MACRO, TokenType::Macro, 0); }

"#".*$ { Drop(); } //just drop comments //return ProcessToken("COMMENT", COMMENT, 0); }
\{\#[^\#]*\#+(?:[^\#\}][^\#]*\#+)*\} { Drop(); } //just drop comments

  /******* OPERATORS/ OTHER SYMBOLS ********/
"+="|"-="|"*="|"/="|"%="|"&="|"|="|"^="|"<<="|">>=" { 
                    return ProcessToken("OPASSIGN",  ASSIGN,    TokenType::OpAssign,  0); }
">>"|"<<"         { return ProcessToken("SHIFT",     SHIFT,     TokenType::Shift,     0); } 
"?"               { return ProcessToken("QUESTION",  QUESTION,  TokenType::Question,  0); }
"=="|"!="         { return ProcessToken("EQUIV",     EQUIV,     TokenType::EqOp,      0); }
"<="|">="         { return ProcessToken("RELAT",     RELAT,     TokenType::RelOp,     0); }
"&&"|"||"|"^^"    { return ProcessToken("LOGIC",     LOGIC,     TokenType::LogOp,     0); }
"&"|"|"|"^"       { return ProcessToken("BITWISE",   BITWISE,   TokenType::BitOp,     0); } 
"+"|"-"           { return ProcessToken("ADD",       ADD,       TokenType::AddOp,     0); }
"*"|"/"|"%"       { return ProcessToken("MULT",      MULT,      TokenType::MultOp,    0); }
"->"              { return ProcessToken("ARROW",     ARROW,     TokenType::Apply,     0); }
"!"               { return ProcessToken("NOT",       NOT,       TokenType::Not,       0); }
"."               { return ProcessToken("DOT",       DOT,       TokenType::Access,    0); }
"..."             { return ProcessToken("RANGE",     RANGE,     TokenType::Range,     0); }
","               { return ProcessToken("COMMA",     COMMA,     TokenType::Comma,     0); }
"="               { return ProcessToken("ASSIGN",    ASSIGN,    TokenType::Equal,     0); }
";"               { return ProcessToken("SEMICOLON", SEMICOLON, TokenType::Semicolon, 0); }
"::"              { return ProcessToken("DUB_COLON", DUBCOLON,  TokenType::DubColon,  0); }
"~"               { return ProcessToken("TILDE",     TILDE,     TokenType::Tilde,     0); }
":"               { return ProcessToken("COLON",     COLON,     TokenType::Colon,     0); } 

  /******* BRACKETS *******/
"(" { return ProcessToken("PAREN_OPEN",    PAREN_OPEN,    TokenType::LeftParen, 0); }
")" { return ProcessToken("PAREN_CLOSED",  PAREN_CLOSED,  TokenType::RightParen, 0); }
"[" { return ProcessToken("SQUARE_OPEN",   SQUARE_OPEN,   TokenType::LeftSquare, 0); }
"]" { return ProcessToken("SQUARE_CLOSED", SQUARE_CLOSED, TokenType::RightSquare, 0); }
"{" { return ProcessToken("BRACE_OPEN",    BRACE_OPEN,    TokenType::LeftBrace, 0); }
"}" { return ProcessToken("BRACE_CLOSED",  BRACE_CLOSED,  TokenType::RightBrace, 0); }
"<" { return ProcessToken("ANGLE_OPEN",    ANGLE_OPEN,    TokenType::LeftAngle, 0); }
">" { return ProcessToken("ANGLE_CLOSED",  ANGLE_CLOSED,  TokenType::RightAngle, 0); }


  /******** VALUES **********/
"true"|"false"         { return ProcessToken("BOOLVAL", BOOLVAL, TokenType::Boolean, 3); }
"0x"([0-9A-F]{2})+     { return ProcessToken("HEXVAL",  HEXVAL,  TokenType::HexCode, 4); }
\-?[0-9]+              { return ProcessToken("INTVAL",  INTVAL,  TokenType::Integer, 1); }
\-?([0-9]*\.[0-9]+|[0-9]+\.[0-9]+)([eE][-+]?[0-9]+)? { return ProcessToken("FLOATVAL", FLOATVAL, TokenType::Float, 2); }

  /*add escaped quotes*/
\$?(\"[^\"]*\"|\'[^\']*\')   { return ProcessToken("STRINGVAL",  STRINGVAL,  TokenType::String, 0); }
[a-zA-Z_][a-zA-Z0-9_]*     { return ProcessToken("IDENTIFIER", IDENTIFIER, TokenType::Identifier, 0); }

. {
  //tdebugf("[%i, %i] Unknown Token %s\n", line_number, column_number, yytext);

  //column_number += yyleng;
  InvalidToken();
  exit(0);

  //maybe we want to create a token for this unknown token and 
  //then store it in a list so we can report all of the unknown tokens at once
  //yylval.lexeme = strdup(yytext);
  //return UNKNOWN;
}
%%

int yywrap(void){
  return 1;
}

void InvalidToken(){
  fprintf(stderr, "ERROR ON LINE %i [%i] : \n Unknown Token %s\n", line_number, column_number, yytext);
}

void Drop()
{
  UpdateLineColumnNumber();
  tdebugf("[%i, %i] DROPPED : %s\n", line_number, column_number, yytext);
}

void Drop(const char* message)
{
  UpdateLineColumnNumber();
  tdebugf("[%i, %i] DROPPED : %s\n", line_number, column_number, message);
}

int ProcessToken(const char* name, int token_parser_id, TokenType token_type, int value_type)
{
  UpdateLineColumnNumber();

  tdebugf("[%i, %i] %s %s\n", line_number, column_number, name, yytext);

  Token* token;
  if(value_type == 0){
    token = new Token(token_type, strdup(yytext), line_number, column_number);
  }else if(value_type == 1){
    token = new Token(token_type, atoi(yytext), line_number, column_number);
  }else if(value_type == 2){
    token = new Token(token_type, (float)atof(yytext), line_number, column_number);
  }else if(value_type == 3){
    token = new Token(token_type, strcmp(yytext, "true") == 0, line_number, column_number);
  }else if(value_type == 4) { //hex value
    token = new Token(token_type, strdup(yytext) + 2, line_number, column_number);
  }

  yylval.token = token;
  return token_parser_id;
}

void UpdateLineColumnNumber()
{
  line_number = next_line_number;
  column_number = next_column_number;
  
  int off = 1;
  int reset = 0;
  for(int i = 0; i < yyleng; i++)
  {
    if(yytext[i] == '\n') {
      next_line_number = next_line_number + 1;
      reset = 1;
      off = 1;
    } else {
      off = off + 1;
    }
  }
  next_column_number = reset ? off : next_column_number + off - 1;
}

void yyerror(const char* s){
  fprintf(stderr, "\nERROR ON LINE %i [%i] : \n %s\n", line_number, column_number, s);
  exit(0);
}
