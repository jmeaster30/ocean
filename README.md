# ocean
A C-like programming language (get it like sea-like like an ocean lol)

This is a hobby language for myself so I can learn more about how compilers work and how to design programming languages.

I am really interested in static analysis and optimizations so I will be building those kinds of features into this language. Also, when I use this language I want to feel like I am in control of the language without having to modify the compiler at all. Tooling is also super important to me so I would like to build a bunch of really nice tooling. 

My inspiration comes from all the programming languages I have used and all the pain points I have run into. Some languages that I really like but have weird nitpicks with are:

- Rust
- C++
- D
- C#
- Likely loads of others since I have used a bunch but I didn't consciously pull features from those.

It is unimportant what the exact nitpicks are because I don't understand any of these languages in their entirety and likely just don't know how to properly use the things I don't like.

## Key Features
### Tooling
I like the dotnet and cargo system and want to provide a similar all-in-one kind of experience with this compiler. Some ideas I had were:
- Testing
  - Unit
  - Fuzz
- Performance analysis
- Memory layout reader / CPU register viewer
### Simple Build Systems
I want to be able to give the compiler the file that contains the start of the program and be able to compile it even if the codebase is spread across many files. I also would like to integrate a decent package manager and library loader into the build system so you don't have to wrestle the compiler to find your dependencies.
### Broad Compile Targets
It would be cool if we can have this language compile to several targets such as:
- Native machine code
- LLVM
- C++
- CLR (dotnet)
- Java bytecode

### Static Analysis
I would like to be able to have the compiler understand what values certain variables can be at any given point in the program so we can check for things like unhandled cases, dead code, and out-of-bounds accesses at compile time. There are many more things I would want to check for but I don't know what I don't know.
### Neat Generic System
I want to have generics determined by the operators and functions that are being used on the generic variables. This would provide some pretty neat functionality I feel but I haven't gotten to that point yet to really play with it.
### Generic Operator Overloading
I don't think I would be able to implement this but I would like to add in the ability to override any symbol and provide the user with the ability to add their own operators and define the precedence and associativity of those operators. I tried writing out the parse for this and it proved to be really difficult so I am not sure if I will be able to get to this. Some things I noted was (especially with the type system I wanted (see Neat Generics System)) it would require a parser pass without parsing the expressions and then a type checking pass and then another parser pass to parse the expressions. Also, dealing with parsing conflicts when the user could potentially add an operator using the same symbol but the operator works on different types and have a different precedence order. Maybe this will be a different project.

