#![feature(process_exitcode_placeholder)]

use ansi_term::{Color, Style};
use std::{io::{self, Write as _}, process::ExitCode};
use supports_color::Stream;

fn main() -> ExitCode {
    if let Err(_) = main_impl() {
        eprintln!("fatal error");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_impl() -> Result<(), ()> {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let mut interp = oracle_backend::interp::Interp::default();

    loop {
        print!("{} ", color(
            Stream::Stdout,
            ">".to_string(),
            Style::new().bold().fg(Color::Green),
        ));

        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        let input = input.trim();

        if let Some(cmd) = process_cmd(input) {
            match cmd {
                Cmd::GetHelp => {
                    print_usage();
                }
                Cmd::ListAliases => {
                    println!(
                        "{}",
                        interp.aliases
                            .iter()
                            .map(|(symbol, operand)| {
                                format!("{} -> {}", symbol, operand)
                            })
                            .collect::<Vec<String>>()
                            .join("\n")
                    )
                }
                Cmd::Quit => {
                    return Ok(());
                }
            }

            continue;
        }

        match oracle_backend::process(input) {
            Ok(ast) => {
                if let Err(e) = interp.eval_ast(ast) {
                    print_error(e);
                }
            }
            Err(e) => {
                print_error(e);
            }
        }
    }
}

fn print_error(e: impl std::error::Error) {
    eprintln!(
        "{}: {}",
        color(
            Stream::Stderr,
            "error".to_string(),
            Style::new().bold().fg(Color::Red),
        ),
        e,
    );
}

fn color(stream: Stream, input: String, style: Style) -> String {
    if supports_ansi(stream) {
        format!("{}", style.paint(input))
    } else {
        input
    }
}

fn supports_ansi(stream: Stream) -> bool {
    supports_color::on_cached(stream).map_or(false, |it| it.has_basic)
}

fn process_cmd(input: &str) -> Option<Cmd> {
    let (nothing, cmd) = input.split_once(':')?;
    if !nothing.is_empty() {
        return None;
    }

    match cmd {
        "a" | "list-aliases" => {
            Some(Cmd::ListAliases)
        }
        "h" | "get-help" => {
            Some(Cmd::GetHelp)
        }
        "q" | "quit" => {
            Some(Cmd::Quit)
        }
        _ => None,
    }
}

enum Cmd {
    GetHelp,
    ListAliases,
    Quit,
}

fn print_usage() {
    todo!()
}
