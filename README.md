# Rusty Compiler

### Description
A simple compiler written in the Rust programming language using the LL(1) grammar below. This grammar represents a small language with semicolon delimited statements, arithmetic operations, and simple control structures. 

### Grammar
```               
<program> → start <stmt_list> end 
<stmt_list> → <stmt> ; <stmt_list>
            | ϵ
<stmt> → <assign>
       | <expr>
<assign> → <type> <var> = <expr>
<type> → int
<var> → A | B | C
<expr> → <var> <sum>
<sum> → + <expr>
      | - <expr>
      | <factor>
<factor> → * <expr>
         | / <expr>
         | ϵ
```

## Files
```
rustyparser/
├─ src/
│  ├─ lexer.rs
│  ├─ parser.rs
│  ├─ main.rs
├─ target/
├─ Cargo.toml
```
### token.rs
* `enum TokenTypes`: Enumeration for the types of tokens in language.
* `struct Token`: Struct to encapsulate token data such as type and line # for tokens made.
* `make_token(curr_line: usize, curr_type: TokenTypes) -> Token`: Function to make token given line # and type.

### lexer.rs
* `struct Lexer`: Struct to represent the Lexer with members for input, position, and line #.
* `new(input: String) -> Self`: Constructor to initialize Lexer given input string.
* `next_char(&self) -> Option<char>`: Function to get char at the current position.
* `get_line(&self) -> usize`: Function to get the current line #.
* `skip_whitespace(&mut self)`: Function to skip whitespace (' ', '\t', '\n', '\r').
* `consume(&mut self)`: Function to "consume" the current lexeme (inc position).
* `consume_keywords(&mut self)`: Function to "consume" the keywords (start & end).
* `next_token(&mut self) -> Token`: Function to create and return the current token.

### parser.rs

### main.rs
* **LEXICAL ANALYSIS** - Initializes a lexer with an input string. It loops through generating tokens and pushing them into a vector terminating when TokEnd is encountered. The vector is then printed for debugging.

## Usage
Update parameter: `let mut lexer = lexer::Lexer::new("start \n a = a + b + c; \n end".to_string());` 
```bash
cargo build
```
```bash
cargo run
```

## Resources
1. Sebesta, Robert W. Concepts of Programming Languages - 10th Edition. Pearson Addison Wesley, 2012.
    ```
    Figure 3.1 A Grammar for a Small Language

    <program> → begin <stmt_list> end 
    <stmt_list> → <stmt>
                  | <stmt> ; <stmt_list>
    <stmt> → <var> = <expression>
    <var> → A | B | C
    <expression> → <var> + <var>
                   | <var> – <var>
                   | <var>
    ```
    ```
    Figure 3.4 An Unambiguous Grammar for Expressions

    <assign> → <id> = <expr>
    <id> → A | B | C 
    <expr> → <expr> + <term>
             | <term>
    <term> → <term> * <factor>
             | <factor>
    <factor> → ( <expr> )
               | <id>
    ```
