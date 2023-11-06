use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Clone)]
pub enum AtomicExpression {
    Symbol(String),
    Number(f64),
}

impl fmt::Display for AtomicExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AtomicExpression::Symbol(s) => write!(f, "{s}"),
            AtomicExpression::Number(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Atom(AtomicExpression),
    List(Vec<Expression>),
    Function(fn(&[Expression]) -> Result<Expression, Error>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Atom(a) => write!(f, "{a}"),
            Expression::List(l) => write!(
                f,
                "({})",
                l.iter()
                    .map(|it| format!("{it}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expression::Function(_) => write!(f, "<function>"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Reason(String),
}

#[derive(Debug)]
pub struct Environment {
    pub bindings: HashMap<String, Expression>,
}

impl Default for Environment {
    fn default() -> Self {
        let bindings = HashMap::from([
            (
                "+".to_string(),
                Expression::Function(|args| {
                    if args.len() == 2 {
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[0] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[1] {
                                return Ok(Expression::Atom(AtomicExpression::Number(x + y)));
                            }
                        }
                    }
                    Err(Error::Reason(
                        "`+` only accepts numbers as it's argument".to_string(),
                    ))
                }),
            ),
            (
                "-".to_string(),
                Expression::Function(|args| {
                    if args.len() == 2 {
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[0] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[1] {
                                return Ok(Expression::Atom(AtomicExpression::Number(x - y)));
                            }
                        }
                    }
                    Err(Error::Reason(
                        "`-` only accepts numbers as it's argument".to_string(),
                    ))
                }),
            ),
            (
                "*".to_string(),
                Expression::Function(|args| {
                    if args.len() == 2 {
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[0] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[1] {
                                return Ok(Expression::Atom(AtomicExpression::Number(x * y)));
                            }
                        }
                    }
                    Err(Error::Reason(
                        "`*` only accepts numbers as it's argument".to_string(),
                    ))
                }),
            ),
            (
                "/".to_string(),
                Expression::Function(|args| {
                    if args.len() == 2 {
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[0] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[1] {
                                if y == 0. {
                                    return Err(Error::Reason("can't divide by 0".to_string()));
                                } else {
                                    return Ok(Expression::Atom(AtomicExpression::Number(x / y)));
                                }
                            }
                        }
                    }
                    Err(Error::Reason(
                        "`/` only accepts numbers as it's argument".to_string(),
                    ))
                }),
            ),
        ]);

        Self { bindings }
    }
}
