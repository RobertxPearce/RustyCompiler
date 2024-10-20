/// Robert Pearce
/// Lexer Implementation in Rust

///----------------------------------
///     TOKEN

/// Enumeration for the tokens
#[derive(Debug, Clone, PartialEq)]
enum TokenTypes {
    TokStart,   // start
    TokEnd,     // end
    TokSemi,    // ;
    TokAssign,  // =
    TokPlus,    // +
    TokMinus,   // -
    TokA,       // a
    TokB,       // b
    TokC,       // c
    TokInvalid, // Error
    TokEOF,     // End of File
}

/// Struct for token type
pub struct Token {
    kind: TokenTypes,   // Variable for token type
    line: usize,        // Variable for line of token
}

impl Token {
    /// @Brief Function to create and return token
    /// @Param Reference to lexer
    /// @Param Token Type
    pub fn make_token(curr_line: usize, curr_type: TokenTypes) -> Token {
        Token {
            kind: curr_type,
            line: curr_line,
        }
    }
}  // End of Token methods

///----------------------------------
///     LEXER

/// Struct for the lexical analysis
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
    pub fn get_line(&self) -> usize {
        self.line
    }

    /// @Brief Function to skip whitespace
    /// @Param Mutable reference to self to update position
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.next_char() {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.position += 1;
            } else {
                break;
            }
        }
    }

    /// @Brief Function to "consume" current lexeme
    /// @Param Mutable reference to self
    fn consume(&mut self) {
        self.position += 1;
    }

    /// @Brief Function to "consume" keywords
    /// @Param Mutable reference to self
    fn consume_keywords(&mut self) {

    }

    /// @Brief Function to return the current token
    /// @Param Reference to self mutable to update position
    /// @Return Current Token
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace
        self.skip_whitespace();

        // Set char variable to next token
        let char = self.next_char();

        // Switch statement to determine token type
        let curr_type = match char {
            Some('s') => {
                self.consume();
                TokenTypes::TokStart
            },
            Some('e') => {
                self.consume();
                TokenTypes::TokEnd
            },
            Some(';') => {
                self.consume();
                TokenTypes::TokSemi
            },
            Some('=') => {
                self.consume();
                TokenTypes::TokAssign
            },
            Some('+') => {
                self.consume();
                TokenTypes::TokPlus
            },
            Some('-') => {
                self.consume();
                TokenTypes::TokMinus
            },
            Some('a') => {
                self.consume();
                TokenTypes::TokA
            },
            Some('b') => {
                self.consume();
                TokenTypes::TokB
            },
            Some('c') => {
                self.consume();
                TokenTypes::TokC
            },
            Some(_) => {
                self.consume();
                TokenTypes::TokInvalid
            },
            None => {
                TokenTypes::TokEOF
            },
        };

        Token::make_token(self.line, curr_type)
    }

}  // End of Lexer methods
