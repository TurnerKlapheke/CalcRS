// src/stack.rs
//! A generic stack implementation used throughout the calculator

/// A LIFO stack data structure implemented using a Vec
/// In accordence with the ADT operations of a stack
/// All methods of the stack or O(1) Amortized cost
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack
    pub fn new() ->  Self {
        Stack { elements: Vec::new() }
    }
    /// Pushes an item onto the top of the stack
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }
    /// Removes and returns the item that was on top of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
    // Takes a look at the item on top of the stack
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
}