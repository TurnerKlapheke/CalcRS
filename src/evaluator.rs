// src/evaluator.rs
//! Evaluates postfix notation expressions using a stack machine

use crate::stack::Stack;
use crate::tokenizer::Token;

/// Evaluates postfix tokens to a final numerical result
///
/// # Arguments
/// * `tokens` - Slice of tokens in postfix notation
///
/// # Returns
/// Result containing either:
/// - f64 result of evaluation
/// - Error message string for invalid expressions
///
/// # Evaluation Process:
/// 1. Initialize operand stack
/// 2. For each token:
///    - Numbers are pushed to stack
///    - Operators pop two operands and push result
/// 3. Final stack should contain exactly one value
pub fn evaluate_postfix(tokens: &[Token]) -> Result<f64, &'static str> {
    let mut stack = Stack::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            _ => {
                let b = stack.pop().ok_or("Missing operand")?;
                let a = stack.pop().ok_or("Missing operand")?;
                let result = match token {
                    Token::Plus => a + b,
                    Token::Minus => a - b,
                    Token::Multiply => a * b,
                    Token::Divide => {
                        if b == 0.0 {
                            return Err("Division by zero");
                        }
                        a / b
                    }
                    _ => return Err("Invalid token in postfix evaluation"),
                };
                stack.push(result);
            }
        }
    }

    stack.pop().ok_or("No result found")
}