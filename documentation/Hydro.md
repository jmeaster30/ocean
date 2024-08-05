# Hydro Programming Language Documentation

## Table of Contents
1. [Instructions](#instructions-and-what-they-do)
2. [Bytecode Binary File Spec](#bytecode-binary-file-spec)
   1. [File Bytes Layout](#file-bytes-layout)
   2. [Module Bytes Layout](#module-bytes-layout)
   3. [Using Bytes Layout](#using-bytes-layout)
   4. [Layout Bytes Layout](#layout-bytes-layout)
   5. [Function Bytes Layout](#function-bytes-layout)
   6. [Single Byte Instruction Layout](#single-byte-instruction-layout)
   7. [Multi Byte Instruction Layout](#multi-byte-instruction-layout)
      1. [Push](#push)
      2. [Jump](#jump)
      3. [Branch](#branch)
      4. [AllocArray](#allocarray)
      5. [AllocLayout](#alloclayout)
      6. [LayoutIndex](#layoutindex)

## Instructions and What They Do

These are all of the instructions in Hydro and what they do. Arguments for the instructions are symbolized with `$` if it is a value, `@` if it is a label, and `^` for types. 
The way to interpret the stack in the stack before/after is:
1. All values are comma separated
2. Left most value is the top of the stack
3. `...` signifies a variable number of values that are on the stack or values that just don't matter
   1. `...(3)` means the values don't matter but there are 3 of them.
4. `^`, `@`, and `$` symbolize the value of the arguments 

| Instruction Name | Stack Before  | Stack After | Description                                                                                                                                      | Valid Types          |
|------------------|---------------|-------------|--------------------------------------------------------------------------------------------------------------------------------------------------|----------------------|
| Pop              | `[a, ...]`    | `[...]`     | Removes value off of the stack                                                                                                                   |                      |
| Push `^t` `$x`   | `[...]`       | `[$x, ...]` | Pushes value `$x` of type `^t` on to the stack                                                                                                   | `^t` can be any type |
| Add              | `[b, a, ...]` | `[c, ...]`  | Pops values `a` and `b` and pushes the value `c` which is the sum of `a` and `b`. This only works if summation is defined for the supplied types |                      |
| Sub              | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| Multiply         | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| Divide           | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| Modulo           | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| LeftShift        | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| RightShift       | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| BitwiseAnd       | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| BitwiseOr        | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| BitwiseXor       | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| BitwiseNot       | `[b, a, ...]` | `[c, ...]`  |                                                                                                                                                  |                      |
| And              |               |             |                                                                                                                                                  |                      |
| Or               |               |             |                                                                                                                                                  |                      |
| Xor              |               |             |                                                                                                                                                  |                      |
| Not              |               |             |                                                                                                                                                  |                      |
| Equal            |               |             |                                                                                                                                                  |                      |
| NotEqual         |               |             |                                                                                                                                                  |                      |
| LessThan         |               |             |                                                                                                                                                  |                      |
| GreaterThan      |               |             |                                                                                                                                                  |                      |
| LessThanEqual    |               |             |                                                                                                                                                  |                      |
| GreaterThanEqual |               |             |                                                                                                                                                  |                      |
| Jump             |               |             |                                                                                                                                                  |                      |
| Branch           |               |             |                                                                                                                                                  |                      |
| Call             |               |             |                                                                                                                                                  |                      |
| Return           |               |             |                                                                                                                                                  |                      |
| Load             |               |             |                                                                                                                                                  |                      |
| Store            |               |             |                                                                                                                                                  |                      |
| ArrayIndex       |               |             |                                                                                                                                                  |                      |
| LayoutIndex      |               |             |                                                                                                                                                  |                      |
| AllocArray       |               |             |                                                                                                                                                  |                      |
| AllocLayout      |               |             |                                                                                                                                                  |                      |

## Bytecode Binary File Spec

Everything is in big endian format so if we need to read the bytes  `68 79 64 72 6F` and covert to ascii you will get the string `hydro` and reading `00 01` will produce `1`

### File Bytes Layout

|                | Byte Offset | Byte Length | Data Type | Notes                                    |
|----------------|-------------|-------------|-----------|------------------------------------------|
| Magic Number   | 0           | 5           | string    | `68 79 64 72 6F` or `hydro` in UTF8      |
| Num of Modules | 5           | 4           | u32       |                                          |
| Modules Array  | 9           | varies      | Module[]  | Length is determined by 'Num of Modules' |

### Module Bytes Layout

|                          | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------------|-------------|-------------|-----------|-----------------------------------------|
| Module Marker            | 0           | 1           | byte      | Single byte `(hex of M)` or `M` in UTF8 |
| Module Name Length (mnl) | 1           | 2           | u16       |                                         |
| Module Name              | 3           | mnl         | string    | UTF8 encoding of the module's name      | 
| Usings Offset            | 3 + mnl     | 4           | u32       |                                         |
| Usings Length            | 7 + mnl     | 4           | u32       |                                         |
| Layouts Offset           | 13 + mnl    | 4           | u32       |                                         |
| Layouts Length           | 17 + mnl    | 4           | u32       |                                         |
| Functions Offset         | 21 + mnl    | 4           | u32       |                                         |
| Functions Length         | 25 + mnl    | 4           | u32       |                                         |

### Using Bytes Layout

|                         | Byte Offset | Byte Length | Data Type | Notes                                       |
|-------------------------|-------------|-------------|-----------|---------------------------------------------|
| Using Marker            | 0           | 1           | byte      | Single byte `(hex of U)` or `U` in UTF8     |
| Using Name Length (unl) | 1           | 2           | u16       |                                             |
| Using Name              | 3           | unl         | string    | UTF8 encoding of the referenced module name |

### Layout Bytes Layout

|                          | Byte Offset | Byte Length | Data Type | Notes                                       |
|--------------------------|-------------|-------------|-----------|---------------------------------------------|
| Layout Marker            | 0           | 1           | byte      | Single byte `(hex of L)` or `L` in UTF8     |
| Layout Name Length (lnl) | 1           | 2           | u16       |                                             |
| Layout Name              | 3           | lnl         | string    | UTF8 encoding of the layout template's name |
| Member Number            | 3 + lnl     | 2           | u16       ||


### Function Bytes Layout

|                                          | Byte Offset | Byte Length | Data Type     | Notes                                   |
|------------------------------------------|-------------|-------------|---------------|-----------------------------------------|
| Function Marker                          | 0           | 1           | byte          | Single byte `(hex of F)` or `F` in UTF8 |
| Function Name Length (fnl)               | 1           | 2           | u16           |                                         |
| Function Name                            | 3           | fnl         | string        | UTF8 encoding of the function's name    |
| Parameter Number                         | 3 + fnl     | 1           | u8            |                                         |
| (Parameter Name Length + Parameter Name) | varies      | varies      | (u16, string) |                                         |
| Instruction Length                       | varies      | 4           | u32           |                                         | 
| Instruction Array                        | varies      | varies      | Instruction[] |                                         |

### Single-Byte Instruction Layout

|                    | Byte Offset | Byte Length | Data Type | Notes                                       |
|--------------------|-------------|-------------|-----------|---------------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte that identifies the instruction |

| Instruction      | Hex | UTF8 Encoding |
|------------------|-----|---------------|
| Pop              |     | `.`           |
| Add              |     | `+`           |
| Sub              |     | `-`           |
| Multiply         |     | `*`           |
| Divide           |     | `/`           |
| Modulo           |     | `%`           |
| LeftShift        |     | `L`           |
| RightShift       |     | `R`           |
| BitwiseAnd       |     | `&`           |
| BitwiseOr        |     | `\|`          |
| BitwiseXor       |     | `^`           |
| BitwiseNot       |     | `~`           |
| And              |     | `a`           |
| Or               |     | `o`           |
| Xor              |     | `x`           |
| Not              |     | `n`           |
| Equal            |     | `=`           |
| NotEqual         |     | `!`           |
| LessThan         |     | `<`           |
| GreaterThan      |     | `>`           |
| LessThanEqual    |     | `(`           |
| GreaterThanEqual |     | `)`           |
| Call             |     | `c`           |
| Return           |     | `r`           |
| Load             |     | `g`           |
| Store            |     | `s`           |
| ArrayIndex       |     | `i`           |

### Multi-Byte Instruction Layout

#### Push

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of :)` or `:` in UTF8 |

#### Jump

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of j)` or `j` in UTF8 |

#### Branch

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of b)` or `b` in UTF8 |

#### AllocArray

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of [)` or `[` in UTF8 |

#### AllocLayout

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of {)` or `{` in UTF8 |

#### LayoutIndex

|                    | Byte Offset | Byte Length | Data Type | Notes                                   |
|--------------------|-------------|-------------|-----------|-----------------------------------------|
| Instruction Marker | 0           | 1           | byte      | Single byte `(hex of m)` or `m` in UTF8 |

