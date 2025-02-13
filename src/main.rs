// src/main.rs
//! Main CLI interface for the calculator

mod evaluator;
mod shunting_yard;
mod stack;
mod tokenizer;

use std::io::{self, Write};
use evaluator::evaluate_postfix;
use shunting_yard::infix_to_postfix;
use tokenizer::{tokenize, ParseError};

fn main() -> io::Result<()> {
    //print intro stuff
    println!("Basic Algebraic Calculator\nEnter expressions or 'exit' to quit");

    // the loop of death (actually the loop of math but death sounds cooler)
    loop {
        //make sure we're nice and clean
        io::stdout().flush()?;

        //take input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        //exit if input says so
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        //does the mathy bits and stuff
        match process_expression(input) {
            // if we get an ok result, print it
            Ok(result) => println!("Result: {}", result),
            // if not, also print it, but it's an error not an answer
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    //jobs done
    Ok(())
}

/// Handles the complete processing pipeline for an expression
fn process_expression(input: &str) -> Result<f64, String> {
    let tokens = tokenize(input).map_err(|e| match e {
        ParseError::InvalidCharacter(c) => format!("Invalid character: '{}'", c),
        ParseError::InvalidNumber(n) => format!("Invalid number format: '{}'", n),
    })?;

    let postfix = infix_to_postfix(&tokens).map_err(|e| e.to_string())?;

    evaluate_postfix(&postfix).map_err(|e| e.to_string())
}