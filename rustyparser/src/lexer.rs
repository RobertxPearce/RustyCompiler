/// Robert Pearce
/// Lexer Implementation

use crate::token;

///----------------------------------
///     LEXER

/// Struct for the lexer
pub struct Lexer {
    input: String,      // Variable for source code
    position: usize,    // Variable for current position
    line: usize,        // Variable for current line #
}

/// Implement methods for the Lexer
impl Lexer {
    /// @Brief Constructor
    /// @Param Source code as string
    /// @Return New instance of Lexer
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
        }
    }

    /// @Brief Function to return the next character in the input
    /// @Param Reference to self
    /// @Return A char or None with Option
    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// @Brief Function to return the current line #
    /// @Param Reference to self
    /// @Return The current line #
    #[allow(dead_code)]
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// @Brief Function to skip whitespace
    /// @Param Mutable reference to self to update position
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.next_char() {
            if c == ' ' || c == '\t' {
                self.position += 1;
            } else if c == '\n' || c == '\r' {
                self.position += 1;
                self.line += 1;
            } else {
                break;
            }
        }
    }

    /// @Brief Function to "consume" current lexeme
    /// @Param Mutable reference to self
    fn consume(&mut self) {
        // println!("Consuming Lexeme: {:?}", self.next_char());
        self.position += 1;
    }

    /// @Brief Function to "consume" keywords
    /// @Param Mutable reference to self
    fn consume_keyword(&mut self) {
        // Check for "start"
        if self.input[self.position..].starts_with("start") {
            // println!("Consuming 'start' keyword.");
            self.position += 5;
        }
        // Check for "end"
        else if self.input[self.position..].starts_with("end") {
            // println!("Consuming 'end' keyword.");
            self.position += 3;
        }
        // Check for "int"
        else if self.input[self.position..].starts_with("int") {
            // println!("Consuming 'end' keyword.");
            self.position += 3;
        }
    }

    /// @Brief Function to return the current token
    /// @Param Reference to self mutable to update position
    /// @Return Current Token
    pub fn next_token(&mut self) -> token::Token {

        // Skip whitespace
        self.skip_whitespace();

        // println!("Current Position: {}", self.position);
        // println!("Current Lexeme: {:?}", self.next_char());

        // Set char variable to next token
        let char = self.next_char();

        // Switch statement to determine token type
        let curr_type = match char {
            Some('s') => {
                self.consume_keyword();
                token::TokenTypes::TokStart
            },
            Some('e') => {
                self.consume_keyword();
                token::TokenTypes::TokEnd
            },
            Some(';') => {
                self.consume();
                token::TokenTypes::TokSemi
            },
            Some('=') => {
                self.consume();
                token::TokenTypes::TokAssign
            },
            Some('i') => {
                self.consume_keyword();
                token::TokenTypes::TokInt
            },
            Some('a') => {
                self.consume();
                token::TokenTypes::TokVar
            },
            Some('b') => {
                self.consume();
                token::TokenTypes::TokVar
            },
            Some('c') => {
                self.consume();
                token::TokenTypes::TokVar
            },
            Some('+') => {
                self.consume();
                token::TokenTypes::TokPlus
            },
            Some('-') => {
                self.consume();
                token::TokenTypes::TokMinus
            },
            Some('*') => {
                self.consume();
                token::TokenTypes::TokMul
            },
            Some('/') => {
                self.consume();
                token::TokenTypes::TokDiv
            },
            Some(_) => {
                self.consume();
                token::TokenTypes::TokInvalid
            },
            None => {
                token::TokenTypes::TokInvalid
            },
        };

        token::Token::make_token(self.line, curr_type)
    }

}  // End of Lexer methods
