/// Robert Pearce
/// Main Implementation

mod token;
mod lexer;
mod parser;

//------------------
// Test Lexer
fn main() {

    //----------------------------------
    //     LEXICAL ANALYSIS

    // Create a new lexer instance with test string
    let mut lexer = lexer::Lexer::new("start \n a = a + b + c; \n end".to_string());

    // Vector to hold the tokens
    let mut token_vec = Vec::new();

    // Loop through tokens until we reach TokEnd
    loop {
        // Variable for current token
        let token = lexer.next_token();

        // Push current token into vector
        token_vec.push(token.clone());

        // Exit loop if token is TokEnd
        if token.kind == token::TokenTypes::TokEnd {
            break;
        }
    }

    // Loop to print vector of tokens
    for token in token_vec {
        println!("{:?}", token);
    }

    //----------------------------------
    //     SYNTAX ANALYSIS
}
