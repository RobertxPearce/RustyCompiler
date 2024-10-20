mod lexer;
mod parser;

//------------------
// Test Lexer
fn main() {

    //----------------------------------
    //     LEXICAL ANALYSIS

    // Create a new lexer instance with test string
    let mut lexer = lexer::Lexer::new("start = a + b + c; end".to_string());

    // Vector to hold the tokens
    let mut token_vec = Vec::new();

    // Loop through tokens until we reach TokEnd
    loop {
        // Variable for current token
        let token = lexer.next_token();

        // Push current token into vector
        token_vec.push(token.clone());

        // Exit loop if token is TokEnd
        if token.kind == lexer::TokenTypes::TokEnd {
            break;
        }
    }

    // Loop to print vector of tokens
    for token in token_vec {
        println!("Token: {:?}", token);
    }

    //----------------------------------
    //     SYNTAX ANALYSIS
}
