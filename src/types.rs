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
    Function(fn(&[Expression]) -> Result<Expression, Error>),
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
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[1] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[2] {
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
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[1] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[2] {
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
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[1] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[2] {
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
                        if let Expression::Atom(AtomicExpression::Number(x)) = args[1] {
                            if let Expression::Atom(AtomicExpression::Number(y)) = args[2] {
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
