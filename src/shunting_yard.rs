// src/shunting_yard.rs
//! Converts infix notation to postfix notation using the Shunting Yard algorithm

use crate::stack::Stack;
use crate::tokenizer::Token;

/// Converts infix tokens to postfix notation (Reverse Polish Notation)
///
/// # Arguments
/// * `tokens` - Slice of tokens in infix notation
///
/// # Returns 
/// Result containing either:
/// - Vec<Token> in postfix notation
/// - Error message string for invalid expressions
///
/// # Algorithm Steps:
/// 1. Initialize empty output queue and operator stack
/// 2. Process each token:
///    - Numbers go directly to output
///    - Left parens go to stack
///    - Right parens pop until left paren is found
///    - Operators pop higher precedence ops first
/// 3. Pop remaining operators to output
pub fn infix_to_postfix(tokens: &[Token]) -> Result<Vec<Token>, &'static str> {
    //variables
    let mut output = Vec::new();
    let mut op_stack: Stack<Token> = Stack::new();

    //analyze tokens with shunting alg
    for token in tokens {
        match token {
            Token::Number(n) => output.push(Token::Number(*n)),
            Token::LParen => op_stack.push(token.clone()),
            Token::RParen => {
                let mut found_paren = false;
                while let Some(top) = op_stack.pop() {
                    if let Token::LParen = top {
                        found_paren = true;
                        break;
                    } else {
                        output.push(top);
                    }
                }
                if !found_paren {
                    return Err("Mismatched parentheses");
                }
            }
            _ => {
                while let Some(top) = op_stack.peek() {
                    if let Token::LParen = top {
                        break;
                    }
                    if precedence(token) <= precedence(top) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(token.clone());
            }
        }
    }

    // Add remaining operators to output
    while let Some(op) = op_stack.pop() {
        if let Token::LParen = op {
            return Err("Mismatched parentheses");
        }
        output.push(op);
    }

    // done :)
    Ok(output)
}

/// Returns operator precedence level
fn precedence(token: &Token) -> u8 {
    match token {
        Token::Multiply | Token::Divide => 3,
        Token::Plus | Token::Minus => 2,
        _ => 0,
    }
}