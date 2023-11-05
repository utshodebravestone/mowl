use crate::types::{AtomicExpression, Error, Expression};

pub fn parse(tokens: &[String]) -> Result<(Expression, &[String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or(Error::Reason("couldn't get enough token".to_string()))?;
    match &token[..] {
        "(" => parse_sequence(rest),
        ")" => Err(Error::Reason("unexpected end of list".to_string())),
        _ => Ok((Expression::Atom(parse_atom(token)), rest)),
    }
}

#[test]
fn test_parse() {
    let expectation: (Expression, &[String]) = (
        Expression::List(vec![
            Expression::Atom(AtomicExpression::Symbol("+".to_string())),
            Expression::Atom(AtomicExpression::Number(6.9)),
            Expression::Atom(AtomicExpression::Number(4.20)),
        ]),
        &[],
    );
    let tokens = &[
        "(".to_string(),
        "+".to_string(),
        "6.9".to_string(),
        "4.20".to_string(),
        ")".to_string(),
    ];
    let reality = parse(tokens).unwrap();
    assert_eq!(expectation, reality);

    let tokens = &[")".to_string()];
    assert!(parse(tokens).is_err());
}

fn parse_sequence(tokens: &[String]) -> Result<(Expression, &[String]), Error> {
    let mut list = vec![];
    let mut tokens = tokens;
    loop {
        let (token, rest) = tokens.split_first().ok_or(Error::Reason(
            "couldn't find closing parenthesis".to_string(),
        ))?;

        if token == ")" {
            return Ok((Expression::List(list), rest));
        }

        let (expression, rest) = parse(tokens)?;
        list.push(expression);
        tokens = rest;
    }
}

#[test]
fn test_parse_sequence() {
    let expectation: (Expression, &[String]) = (
        Expression::List(vec![
            Expression::Atom(AtomicExpression::Symbol("+".to_string())),
            Expression::Atom(AtomicExpression::Number(6.9)),
            Expression::Atom(AtomicExpression::Number(4.20)),
        ]),
        &[],
    );
    let tokens = &[
        "+".to_string(),
        "6.9".to_string(),
        "4.20".to_string(),
        ")".to_string(),
    ];
    let reality = parse_sequence(tokens).unwrap();
    assert_eq!(expectation, reality);

    let tokens = &["+".to_string(), "6.9".to_string(), "4.20".to_string()];
    assert!(parse_sequence(tokens).is_err());
}

fn parse_atom(token: &str) -> AtomicExpression {
    match token.parse() {
        Ok(number) => AtomicExpression::Number(number),
        Err(_) => AtomicExpression::Symbol(token.to_string()),
    }
}

#[test]
fn test_parse_atom() {
    let expectation = AtomicExpression::Number(6.9);
    let reality = parse_atom("6.9");
    assert_eq!(expectation, reality);

    let expectation = AtomicExpression::Number(420.);
    let reality = parse_atom("420");
    assert_eq!(expectation, reality);

    let expectation = AtomicExpression::Symbol("x".to_string());
    let reality = parse_atom("x");
    assert_eq!(expectation, reality);
}
