#[macro_use]
mod macro_rules;

mod evaluator;
mod parser;
mod tokenizer;

use anyhow::Result;
use evaluator::Evaluator;
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("bc-rs (commit {})", env!("COMMIT_HASH"));
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut inp_expr = String::new();
        io::stdin().read_line(&mut inp_expr)?;

        println!("{}", Evaluator::new_evaluator(&inp_expr).eval()?);
    }
}
