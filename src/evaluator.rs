use crate::types::{AtomicExpression, Environment, Error, Expression};

pub fn evaluate(environment: &Environment, expression: &Expression) -> Result<Expression, Error> {
    match expression {
        Expression::Atom(atom) => match atom {
            AtomicExpression::Symbol(s) => {
                if let Some(expression) = environment.bindings.get(s) {
                    Ok(expression.clone())
                } else {
                    Err(Error::Reason(format!("undefined symbol {s}")))
                }
            }
            AtomicExpression::Number(_) => Ok(expression.clone()),
        },
        Expression::List(list) => {
            let function = list
                .first()
                .ok_or(Error::Reason("can't have an empty list".to_string()))?;
            let args = &list[1..];
            let function = evaluate(environment, function)?;
            match function {
                Expression::Function(f) => {
                    let args = args
                        .iter()
                        .map(|it| evaluate(environment, it))
                        .collect::<Result<Vec<Expression>, Error>>();
                    f(&args?)
                }
                _ => Err(Error::Reason(format!("`{function}` is not a function"))),
            }
        }
        Expression::Function(_) => unreachable!(),
    }
}
