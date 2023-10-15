# Hydro Syntax

# Grammar

```
IDENTIFIER -> ^[0-9a-zA-Z.\-_\\/]+$
NUMBER -> ^-?(([0-9]+)|([0-9]*\.[0-9]+))$
STRING -> ^['"].*['"]$
COMMENT -> ^%.*$
TYPE -> u8
      | u16
      | u32
      | u64
      | u128
      | s8
      | s16
      | s32
      | s64
      | s128
      | string
      | bool
      | any
      | NUMBER TYPE
      | IDENTIFIER IDENTIFIER
      .
PROGRAM -> module IDENTIFIER FUNCTIONS PROGRAM
         | 
         .
FUNCTIONS -> FUNCTION FUNCTIONS
           |
           .
FUNCTION -> main PARAMETERS body INSTRUCTIONS
          | function IDENTIFIER PARAMETERS body INSTRUCTIONS
          .
INSTRUCTIONS -> INSTRUCTION INSTRUCTIONS
              |
              .
PARAMETERS -> TYPE PARAMETERS
            |
            .
INSTRUCTION -> alloc TYPE
             | push TYPE NUMBER
             | push TYPE STRING
             | push TYPE true
             | push TYPE false
             | push funcp IDENTIFIER IDENTIFIER
             | push REFERENCE
             | pop
             | duplicate
             | swap
             | add
             | subtract
             | multiply
             | divide
             | modulo
             | leftshift
             | rightshift
             | bitwiseand
             | bitwiseor
             | bitwisexor
             | bitwisenot
             | and
             | or
             | xor
             | not
             | equal
             | notequal
             | lessthan
             | greaterthan
             | lessthanequal
             | greaterthanequal
             | jump IDENTIFIER
             | jump NUMBER
             | branch IDENTIFIER IDENTIFIER
             | branch IDENTIFIER NUMBER
             | branch NUMBER IDENTIFIER
             | branch NUMBER NUMBER
             | call
             | return
             | load
             | store
             | getindex
             | getindex IDENTIFIER
             | setindex
             | setindex IDENTIFIER
             .
REFERENCE -> vref IDENTIFIER
           | iref REFERENCE IDENTIFIER
           .
```