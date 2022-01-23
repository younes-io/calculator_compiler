# Calculator Compiler

## Description

I started this mini-project during my Rust learning journey (which is still ongoing).
The goal is very simple : to implement a mini-compiler in Rust that _interprets_ statements like `2 + 5` or `(6 * 9) / 7 + 20`

Generally, what an interpreter does is:

- Lexical analysis (lexing) : transform the source code (text) into tokens
- Syntax analysis (parsing) : tranform the tokens into an AST (Abstract Syntax Tree)

### Next steps

- Developing the parser:
  - Use a parser combinator ? (like https://github.com/Geal/nom)
  - Use a parser generator ? (like https://github.com/lalrpop/lalrpop)

[if you have any recommendations for me, do not hesitate to let me know]
