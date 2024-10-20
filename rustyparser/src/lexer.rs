/// Robert Pearce
/// Lexer Implementation in Rust

///----------------------------------
///     TOKEN

/// Enumeration for the tokens
#[derive(Debug, Clone, PartialEq)]
pub enum TokenTypes {
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
}

/// Struct for token type
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenTypes,   // Variable for token type
    pub line: usize,        // Variable for line of token
}

/// Implement methods for use with Token
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
    }

    /// @Brief Function to return the current token
    /// @Param Reference to self mutable to update position
    /// @Return Current Token
    pub fn next_token(&mut self) -> Token {

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
                TokenTypes::TokStart
            },
            Some('e') => {
                self.consume_keyword();
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
                TokenTypes::TokInvalid
            },
        };

        Token::make_token(self.line, curr_type)
    }

}  // End of Lexer methods
