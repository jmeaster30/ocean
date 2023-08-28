# Hydro Programming Language Documentation

## Table of Contents
1. [Bytecode Binary File Spec](#bytecode-binary-file-spec)
   1. [File Bytes Layout](#file-bytes-layout)
   2. [Module Bytes Layout](#module-bytes-layout)
   3. [Using Bytes Layout](#using-bytes-layout)
   4. [Layout Bytes Layout](#layout-bytes-layout)
   5. [Function Bytes Layout](#function-bytes-layout)

## Bytecode Binary File Spec

Everything is in big endian format so if we need to read the bytes  `68 79 64 72 6F` and covert to ascii you will get the string `hydro` and reading `00 01` will produce `1`

### File Bytes Layout

|                | Byte Offset | Byte Length | Data Type | Notes                                    |
|----------------|-------------|-------------|-----------|------------------------------------------|
| Magic Number   | 0           | 5           | string    | `68 79 64 72 6F` or `hydro` in UTF8      |
| Num of Modules | 5           | 2           | u16       |                                          |
| Modules Array  | 7           | varies      | Module[]  | Length is determined by 'Num of Modules' |

### Module Bytes Layout

|                          | Byte Offset | Byte Length | Data Type | Notes                              |
|--------------------------|-------------|-------------|-----------|------------------------------------|
| Module Marker            | 0           | 1           | byte      | Single byte `(hex of m)` or `m`    |
| Module Name Length (mnl) | 1           | 2           | u16       |                                    |
| Module Name              | 3           | mnl         | string    | UTF8 encoding of the module's name | 
| Usings Offset            | 3 + mnl     | 4           | u32       |                                    |
| Usings Length            | 7 + mnl     | 4           | u32       |                                    |
| Layouts Offset           | 13 + mnl    | 4           | u32       |                                    |
| Layouts Length           | 17 + mnl    | 4           | u32       |                                    |
| Functions Offset         | 21 + mnl    | 4           | u32       |                                    |
| Functions Length         | 25 + mnl    | 4           | u32       |                                    |

### Using Bytes Layout

|                         | Byte Offset | Byte Length | Data Type | Notes                                       |
|-------------------------|-------------|-------------|-----------|---------------------------------------------|
| Using Marker            | 0           | 1           | byte      | Single byte `(hex of u)` or `u`             |
| Using Name Length (unl) | 1           | 2           | u16       |                                             |
| Using Name              | 3           | unl         | string    | UTF8 encoding of the referenced module name |

### Layout Bytes Layout

| | Byte Offset | Byte Length | Data Type | Notes |
|-|-------------|-------------|-----------|-------|
| |             |             |           |       |

### Function Bytes Layout

| | Byte Offset | Byte Length | Data Type | Notes |
|-|-------------|-------------|-----------|-------|
| |             |             |           |       |
