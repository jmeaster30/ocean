# Grammar
```
Program       -> StatementList .
StatementList -> Statement StatementList
              | .
Statement     -> MacroStatement
              |  PackDecStatement
              |  EnumDecStatement
              |  VarDecStatement
              |  CastStatement
              |  MatchStatement
              |  UseStatement
              |  IfStatement
              |  LoopStatement 
              |  ExpressionStatement 
              |  ContinueStatement
              |  ReturnStatement
              |  BreakStatement .
ContinueStatement -> continue .
BreakStatement    -> break .
ReturnStatement   -> return .
MacroStatement    -> macro .
UseStatement      -> use IdChain .
IdChain           -> id dot IdChain 
                  |  id .
LoopStatement -> loop left_curly StatementList right_curly
              |  while Expression left_curly StatementList right_curly
              |  for id in Expression left_curly StatementList right_curly .
IfStatement -> if Expression left_curly StatementList right_curly
            |  if Expression left_curly StatementList right_curly else IfStatement
            |  if Expression left_curly StatementList right_curly else left_curly StatementList .
MatchStatement -> match Expression left_curly MatchCaseList right_curly .
MatchCaseList -> MatchCase comma MatchCaseList
              |  MatchCase .
MatchCase -> Expression arrow left_curly StatementList right_curly
CastStatement -> cast Function .
PackDecStatement -> pack id left_curly PackDecList right_curly .
PackDecList -> VarDecStatement comma PackDecList
            |  VarDecStatement .
EnumDecStatement -> enum id left_curly EnumDecList right_curly .
EnumDecList -> EnumDecEntry comma EnumDecList
            |  EnumDecEntry .
EnumDecEntry -> id left_paren TypeList right_paren .
             |  id .
VarDecStatement -> TypeVar Assignment Expression
                |  TypeVar Assignment Function .
Assignment -> assignment_symbol .
ExpressionStatement -> Expression .
Function -> left_paren ParameterList right_paren arrow left_paren ReturnList right_paren left_curly StatementList right_curly
ParameterList -> Parameter comma RemainingParameters
              | Parameter
              |  .
RemainingParameters -> Parameter comma RemainingParameters
                    |  Parameter .
Parameter -> TypeVar
          |  triple_dots .
ReturnList -> VarDecStatement comma RemainingParameters
           |  VarDecStatement
           |  .
RemainingParameters -> VarDecStatement comma RemainingParameters
                    |  VarDecStatement .
Expression -> ############################################################## .
TypeVar -> id colon Type .
TypeList -> Type comma TypeList
         |  Type .
Type     -> CompType
         |  Type left_square BaseType right_square . 
CompType -> BaseType
         |  comp BaseType .
BaseType -> void
         |  func
         |  func left_paren TypeList right_paren
         |  func left_paren right_paren
         |  auto
         |  auto id
         |  id
         |  lazy BaseType
         |  ref BaseType
         |  optional BaseType
         |  left_paren Type right_paren
         |  i8  | i16 | i32 | i64 
         |  f32 | f64
         |  u8  | u16 | u32 | u64 
         |  string | bool | char
```
