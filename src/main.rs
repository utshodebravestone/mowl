use std::io::{stdin, stdout, Write};

use mowl::{evaluator::evaluate, parser::parse, tokenizer::tokenize, types::Environment};

fn main() {
    let mut environment = Environment::default();
    let mut src = String::new();

    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("-> ");
        stdout.flush().unwrap();

        stdin.read_line(&mut src).unwrap();

        let tokens = tokenize(&src);
        let (expression, _) = parse(&tokens).unwrap();
        let expression = evaluate(&mut environment, &expression).unwrap();
        println!("{expression}");

        src.clear();
    }
}
