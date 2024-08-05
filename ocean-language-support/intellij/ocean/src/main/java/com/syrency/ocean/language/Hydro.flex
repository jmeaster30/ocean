package com.syrency.ocean.language;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;

import static com.intellij.psi.TokenType.BAD_CHARACTER;
import static com.intellij.psi.TokenType.WHITE_SPACE;
import static com.syrency.ocean.language.psi.HydroTypes.*;

%%

%{
  public HydroLexer() {
    this((java.io.Reader)null);
  }
%}

%public
%class HydroLexer
%implements FlexLexer
%function advance
%type IElementType
%unicode

WHITE_SPACE=\s+

NUMBER=-?(([0-9]+)|([0-9]*\.[0-9]+))
IDENTIFIER=[0-9a-zA-Z.\-_\\/]+
STRING=(['\"].*['\"])
COMMENT=%.*
BASETYPE=u8|u16|u32|u64|u128|s8|s16|s32|s64|s128|string|bool|any|f32|f64
BOOLEAN=true|false
KEYWORD=module|using|layout|function|intrinsic|target|main|this|array|body|cast|funcp|vref|iref|label|alloc|push|pop|duplicate|rotate|swap|add|subtract|multiply|divide|modulo|leftshift|rightshift|bitwiseand|bitwiseor|bitwisexor|bitwisenot|and|or|xor|not|equal|notequal|lessthan|greaterthan|lessthanequal|greaterthanequal|jump|branch|call|return|load|store|getindex|setindex

%%
<YYINITIAL> {
  {WHITE_SPACE}       { return WHITE_SPACE; }

  {NUMBER}            { return NUMBER; }
  {BOOLEAN}           { return BOOLEAN; }
  {KEYWORD}           { return KEYWORD; }
  {STRING}            { return STRING; }
  {COMMENT}           { return COMMENT; }
  {BASETYPE}          { return BASETYPE; }
  {IDENTIFIER}        { return IDENTIFIER; }
}

[^] { return BAD_CHARACTER; }
