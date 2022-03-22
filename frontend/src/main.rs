use std::io::{self, Write as _};

fn main() {
    println!("Hello, world!");

    loop {
        print!("> ");

        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        let tokens = oracle_backend::lexer::lex(input.as_str()).unwrap();
        println!("{:?}", tokens);
        let ast = oracle_backend::parser::parse(tokens.into_iter()).unwrap();
        println!("{:#?}", ast);
        println!("{}", ast);
        let mut interp = oracle_backend::interp::Interp::default();
        interp.eval_ast(ast);
    }
}
