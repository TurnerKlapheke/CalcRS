// src/tokenizer.rs
//! Converts input strings into a sequence of meaningful tokens

/// Represents the different types of tokens in an algebraic expression
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

/// Error types that can occur during tokenization
#[derive(Debug)]
pub enum ParseError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

/// Converts a mathematical expression string into tokens
///
/// # Arguments
/// * `input` - The input string to tokenize
///
/// # Returns
/// Result containing either:
/// - Vec<Token> on successful tokenization
/// - ParseError if invalid input is detected
///
/// # Example
/// ```
/// tokenize("3 + 4*(2 - 1)") → Ok(vec![
///     Token::Number(3.0), Token::Plus, Token::Number(4.0),
///     Token::Multiply, Token::LParen, Token::Number(2.0),
///     Token::Minus, Token::Number(1.0), Token::RParen
/// ])
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>, ParseError> {
    // variables
    let mut tokens = Vec::new(); //the tokens we're gonna return
    let mut chars = input.chars().peekable(); //the input
    let mut allow_number = true; //State flag for negative numbers

    while let Some(&c) = chars.peek() {//has more tokens
        //ignore whitespace between tokens
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        // handle numbers (including negatives and decimals)
        if c.is_ascii_digit() || c =='.' || (c == '-' && allow_number) {
            let mut num_str = String::new();

            //handle negative
            if c == '-' {
                chars.next();
                num_str.push('-');
            }

            //getting the full numerical value of the token
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() || ch == '.' {
                    num_str.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            //push token into num string to be tokenized
            let num = num_str.parse().map_err(|_| ParseError::InvalidNumber(num_str))?;
            tokens.push(Token::Number(num));
            allow_number = false; //finished getting number from token, change state
        } else { //handle operators and parenthesis
            // Handle operators and parentheses
            match chars.next().unwrap() {
                '+' => {
                    tokens.push(Token::Plus);
                    allow_number = true;
                }
                '-' => {
                    tokens.push(Token::Minus);
                    allow_number = true;
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    allow_number = true;
                }
                '/' => {
                    tokens.push(Token::Divide);
                    allow_number = true;
                }
                '(' => {
                    tokens.push(Token::LParen);
                    allow_number = true;
                }
                ')' => {
                    tokens.push(Token::RParen);
                    allow_number = false;
                }
                c => return Err(ParseError::InvalidCharacter(c)),
            }
        }
    }
    //DONE :)
    Ok(tokens)
}