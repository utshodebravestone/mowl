use std::collections::HashMap;

#[derive(Debug)]
enum Atom {
    Symbol(String),
    Number(f64),
}

#[derive(Debug)]
enum Expression {
    Atom(Atom),
    List(Vec<Expression>),
}

#[derive(Debug)]
enum Error {
    Reason(String),
}

#[derive(Debug)]
struct Environment {
    bindings: HashMap<String, Expression>,
}

fn main() {
    println!("Hello, My Own Lisp!");
}
