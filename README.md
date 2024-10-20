# Rusty Parser

### Description
A simple lexer and parser written in the Rust programming language using the LL(1) grammar below. This grammar represents a small language with semicolon delimited statements and assignment, addition, and subtraction operations. 

### Grammar
```               
<program> → start <stmt_list> end 
<stmt_list> → <assign> ; <stmt_list>
            | ϵ
<assign> → <var> = <expression>
<expression> → <var> <arith>
<arith> → + <expression>
        | - <expression>
        | ϵ
<var> → A | B | C
```

## Files
```
rustyparser/
├─ src/
│  ├─ lexer.rs
│  ├─ parser.rs
│  ├─ main.rs
├─ target/
├─ Cargo.lock
├─ Cargo.toml
```
### lexer.rs

### parser.rs

### main.rs

## Usage

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