use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum AtomicExpression {
    Symbol(String),
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Atom(AtomicExpression),
    List(Vec<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Reason(String),
}

#[derive(Debug)]
pub struct Environment {
    pub bindings: HashMap<String, Expression>,
}
