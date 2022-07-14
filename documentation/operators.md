# Operators

## Precedence Table
| Symbols               | Name               | Precedence | Associativity |
|:---------------------:|:------------------:|:----------:|:-------------:|
| `()`                  | subexpression      | 0          | Left          |
| `.`, `[]`, `()`, `as` | access, call, cast | 1          | Left          |
| `?`                   | postfix            | 2          | Right         |
| `~`, `!`, `-`         | prefix             | 3          | Right         |
| `??`                  | default            | 4          | Left          |
| `..`, `..<`, `..=`    | range              | 5          | Left          |
| `++`, `--`, `>.`      | array              | 6          | Left          |
| `*`, `/`, `%`, `//`   | multiplicative     | 7          | Left          |
| `+`, `-`              | additive           | 8          | Left          |
| `^`, `\|`, `&`        | bitwise            | 9          | Left          |
| `^^`, `\|\|`, `&&`    | logical            | 10         | Right         |
| `<<`, `>>`            | shift              | 11         | Left          |
| `<`, `>`, `<=`, `>=`  | comparison         | 12         | Left          |
| `==`, `!=`            | equality           | 13         | Left          |
| `=`, `{op}=`          | assignment         | 14         | Left          |

## Member

### Member Access - `.`

Gets or calls the rhs identifier on the lhs expression.

| Left Hand Type | Right Hand Type   | Resulting Type |
|----------------|-------------------|----------------|
| X              | func(X, ..., Y)   | Y              |
| array(X)       | "length"          | i64            |
| string         | "length"          | i64            |
| pack           | \<identifier\> X* | X              |
| lazy X         | "eval"            | X              |
| ref X          | func(X, ..., Y)   | Y              |
| ref array(X)   | "length"          | i64            |
| ref string     | "length"          | i64            |
| ref pack       | \<identifier\> X* | X              |

### Array Access - `[]`

Gets the value of the lhs expression from the rhs expression

| Left Hand Type | Right Hand Type | Resulting Type |
|----------------|-----------------|----------------|
| array(X, Y)    | X               | Y              |
| array(X)       | i64             | X              |
| string         | i64             | char           |

## Postfix

### HasValue - `?`

Returns if an optional type has a value

| Left Hand Type          | Resulting Type |
|-------------------------|----------------|
| optional X              | bool           |

## Prefix

### Negative - `-`

Returns the number * -1

| Right Hand Type         | Resulting Type |
|-------------------------|----------------|
| uXX                     | iXX            |
| iXX                     | iXX            |
| fXX                     | fXX            |

### Logical Not - `!`

Returns the number * -1

| Right Hand Type         | Resulting Type |
|-------------------------|----------------|
| bool                    | bool           |

### Bitwise Not - `~`

Returns inverted bit string casted back to original data type. String maintains original length. 

| Right Hand Type         | Resulting Type |
|-------------------------|----------------|
| X                       | X              |

