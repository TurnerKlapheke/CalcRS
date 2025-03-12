# CalcRS: A Rust-Based Calculator with Shunting Yard Algorithm

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg?style=flat&logo=rust)](https://www.rust-lang.org)
[![Algorithm](https://img.shields.io/badge/Algorithm-Shunting_Yard-009688.svg?style=flat)](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
[![AI-Assisted](https://img.shields.io/badge/AI_Collaborator-DeepSeek_R1-430098.svg?style=flat)](https://www.deepseek.com)

## Project Overview

**CalcRS** is a command-line calculator implemented in Rust, featuring:
- **Infix-to-Postfix Conversion**: Using Dijkstra's Shunting Yard algorithm
- **Postfix Evaluation**: Stack-based expression evaluation
- **Robust Error Handling**: Invalid input detection and graceful error messages
- **Interactive CLI**: Supports continuous expression evaluation until exit

This project demonstrates Rust's strengths in memory safety, error handling, and modular design while implementing a classic computer science algorithm.

## Key Features

### Core Functionality
- **Tokenization**:
  - Handles numbers (including negatives and decimals)
  - Supports basic operators (`+`, `-`, `*`, `/`)
  - Manages parentheses for precedence
- **Shunting Yard Algorithm**:
  - Converts infix expressions to postfix notation
  - Maintains operator precedence and associativity
  - Detects mismatched parentheses
- **Postfix Evaluation**:
  - Stack-based computation
  - Precise floating-point results

### Error Handling
- **Invalid Characters**: Rejects non-numeric, non-operator input
- **Malformed Numbers**: Detects invalid number formats
- **Mismatched Parentheses**: Ensures balanced expression structure
- **Missing Operands**: Validates expression completeness

## Development Context

This project was developed to explore:
- **Rust's Type System**: Leveraging enums for token representation
- **Algorithm Implementation**: Translating pseudocode to efficient Rust
- **Error Handling**: Using `Result` and custom error types
- **Modular Design**: Separating concerns into distinct modules

**AI Collaboration**:  
This project was developed with assistance from **DeepSeek R1**, which provided:
- Documentation generation
- Debugging support for edge cases

## Key Components

1. **Tokenizer:**
   - Converts raw input Strings into Token enums
   - Handles whitespace, numbers, and operators
   - Detects invalid characters and malformed numbers
2. **Shunting Yard:**
   - Implements Dijkstra's algorithm
   - Manages operator precedence and parentheses
   - Converts infix tokens to postfix notation
3. **Evaluator:**
   - Processes postfix tokens using a stack
   - Performs arithmetic operations

## Code Structure

```plaintext
src/
├── main.rs          # CLI interface and main loop
├── tokenizer.rs     # Input tokenization logic
├── shunting_yard.rs # Infix-to-postfix conversion
├── evaluator.rs     # Postfix expression evaluation
└── stack.rs         # Generic stack implementation
```
## License

The MIT License (MIT)

Copyright (c) 2025 Turner Klapheke

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
