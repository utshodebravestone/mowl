use std::collections::HashMap;

#[derive(Debug)]
pub enum Atom {
    Symbol(String),
    Number(f64),
}

#[derive(Debug)]
pub enum Expression {
    Atom(Atom),
    List(Vec<Expression>),
}

#[derive(Debug)]
pub enum Error {
    Reason(String),
}

#[derive(Debug)]
pub struct Environment {
    pub bindings: HashMap<String, Expression>,
}
