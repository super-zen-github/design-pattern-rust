/// Interpreter design pattern
/// The Interpreter pattern is particularly useful when you need to :
/// - Parse and evaluate domain-specific languages
/// - Process structured text input
/// - Implement rule engines or expression evaluators
/// - Build compilers or interpreters for simple languages
/// The pattern helps break down complex grammar rules into a set of
/// simple objects that can be composed to handle more complex expressions.

pub trait Expression {
    fn interpret(&self) -> i32;
}

pub struct NumberExpression {
    number: i32,
}

impl NumberExpression {
    pub fn new(number: i32) -> Self {
        NumberExpression { number }
    }
}

impl Expression for NumberExpression {
    fn interpret(&self) -> i32 {
        self.number
    }
}

pub struct AddExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl AddExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        AddExpression { left, right }
    }
}

impl Expression for AddExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() + self.right.interpret()
    }
}

pub struct MultiplyExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl MultiplyExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        MultiplyExpression { left, right }
    }
}

impl Expression for MultiplyExpression {
    fn interpret(&self) -> i32 {
        self.left.interpret() * self.right.interpret()
    }
}
