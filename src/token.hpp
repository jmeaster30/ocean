#pragma once

#include <string>
#include <iostream>

union TokenLexeme
{
  bool bool_lex;
  int int_lex;
  float float_lex;
  char* string_lex;
};

enum class TokenType {
	//Tokens
	Unrecognized,
  
  Macro,
	Newline,
	//Keywords
	Type,
	Auto,
	Func,
	Void,
	Op,

	Const,
	Enum,
	Pack,
	Variant,

	If,
	Else,
	For,
	In,
	By,
	While,
	Break,
	Continue,
	Stop,
	Switch,
	Default,

	//variables and constants
	Identifier,
	HexCode,
	String,
	Boolean,
	Float,
	Integer,

	//operators and symbols
	OpAssign,
	Shift,
	Question,
	Range,
	Equal,
	EqOp,
	RelOp,
	LogOp,
	BitOp,
	AddOp,
	MultOp,
	Apply,
	Access,
	Not,
	
	LeftParen,
	RightParen,
	LeftSquare,
	RightSquare,
	LeftBrace,
	RightBrace,
	LeftAngle,
	RightAngle,

	Comma,
	Colon,
  Semicolon,
	DubColon,
	Carrot,
	Tilde,
};

class Token
{
public:
	
	TokenType type;
	TokenLexeme lexeme;
	int linenum;
	int colnum;

	Token(TokenType t, char* l, int lnum, int cnum) {
		type = t;
		lexeme.string_lex = l;
		linenum = lnum;
		colnum = cnum;
	};

  Token(TokenType t, bool l, int lnum, int cnum) {
		type = t;
		lexeme.bool_lex = l;
		linenum = lnum;
		colnum = cnum;
	};

  Token(TokenType t, int l, int lnum, int cnum) {
		type = t;
		lexeme.int_lex = l;
		linenum = lnum;
		colnum = cnum;
	};

  Token(TokenType t, float l, int lnum, int cnum) {
		type = t;
		lexeme.float_lex = l;
		linenum = lnum;
		colnum = cnum;
	};

  std::string toString() {
    //std::cout << "in toString()" << std::endl;
    std::string s = "{[";
    s += std::to_string(linenum) + ":" + std::to_string(colnum) + "]";
    s += "(" + std::to_string((int)type) + ") ";
    if(type == TokenType::Integer)
      s += std::to_string(lexeme.int_lex);
    else if(type == TokenType::Float)
      s += std::to_string(lexeme.float_lex);
    else if(type == TokenType::Boolean)
      s += std::to_string(lexeme.bool_lex);
    else
      s.append(lexeme.string_lex);
    s += "}";
    //std::cout << "done toString()" << std::endl;
    return s;
  }
};

