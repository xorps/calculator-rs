mod nel;
mod token;
mod parser;
mod eval;

use parser::parse;
use eval::eval;

fn main() -> Result<(), std::io::Error> {
    use std::io::Write;
    println!("Calculator, press q to quit");
    loop {
        print!("> ");
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "q" { return Ok(() )}
        let tokens = match parse(input) {
            Ok(tokens) => tokens,
            Err(e) => { println!("Parse Error: {:?}", e); continue }
        };
        match eval(tokens.into_iter()) {
            Ok(n) => println!("{}", n),
            Err(e) => println!("Eval Error: {:?}", e),
        }
    }
}