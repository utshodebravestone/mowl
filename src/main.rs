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

fn tokenize(src: &str) -> Vec<String> {
    src.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|it| it.to_string())
        .collect()
}

fn main() {
    println!("Hello, My Own Lisp!");
}
