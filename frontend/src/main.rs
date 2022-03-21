use std::io::{self, Write as _};

fn main() {
    println!("Hello, world!");

    loop {
        print!("> ");

        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        let lexed = oracle_backend::lexer::lex(input.as_str()).unwrap();
        println!("{:?}", lexed);
        let parsed = oracle_backend::parser::parse(lexed.into_iter()).unwrap();
        println!("{:#?}", parsed);
    }
}
