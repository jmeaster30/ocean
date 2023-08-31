# Hydro Programming Language Documentation

## Table of Contents
1. [Bytecode Binary File Spec](#bytecode-binary-file-spec)
   1. [File Bytes Layout](#file-bytes-layout)
   2. [Module Bytes Layout](#module-bytes-layout)
   3. [Using Bytes Layout](#using-bytes-layout)
   4. [Layout Bytes Layout](#layout-bytes-layout)
   5. [Function Bytes Layout](#function-bytes-layout)
   6. [Single Byte Instruction Layout](#single-byte-instruction-layout)
   7. [Multi Byte Instruction Layout](#multi-byte-instruction-layout)
      1. [Push](#push)

## Bytecode Binary File Spec

Everything is in big endian format so if we need to read the bytes  `68 79 64 72 6F` and covert to ascii you will get the string `hydro` and reading `00 01` will produce `1`

### File Bytes Layout

|                | Byte Offset | Byte Length | Data Type | Notes                                    |
|----------------|-------------|-------------|-----------|------------------------------------------|
| Magic Number   | 0           | 5           | string    | `68 79 64 72 6F` or `hydro` in UTF8      |
| Num of Modules | 5           | 2           | u16       |                                          |
| Modules Array  | 7           | varies      | Module[]  | Length is determined by 'Num of Modules' |

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
| Using Marker            | 0           | 1           | byte      | Single byte `(hex of u)` or `u` in UTF8     |
| Using Name Length (unl) | 1           | 2           | u16       |                                             |
| Using Name              | 3           | unl         | string    | UTF8 encoding of the referenced module name |

### Layout Bytes Layout

|               | Byte Offset | Byte Length | Data Type | Notes                                   |
|---------------|-------------|-------------|-----------|-----------------------------------------|
| Layout Marker | 0           | 1           | byte      | Single byte `(hex of l)` or `l` in UTF8 |


### Function Bytes Layout

|                 | Byte Offset | Byte Length | Data Type | Notes                                   |
|-----------------|-------------|-------------|-----------|-----------------------------------------|
| Function Marker | 0           | 1           | byte      | Single byte `(hex of f)` or `f` in UTF8 |

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

