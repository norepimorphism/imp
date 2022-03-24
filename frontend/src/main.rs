//! Interactive Mathematical Calculator (IMC).
//!
//!

#![feature(process_exitcode_placeholder)]

use ansi_term::{Color, Style};
use std::{io::{self, Write as _}, process::ExitCode};
use supports_color::Stream;

fn main() -> ExitCode {
    if let Err(_) = main_impl() {
        todo!();
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn main_impl() -> Result<(), ()> {
    // let mut interp = oracle_backend::interp::Interp::default();

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
                Cmd::Help => {
                    print_usage();
                }
                Cmd::PrintAliases => {
                    println!(
                        // "{}",
                        // interp.aliases
                        //     .iter()
                        //     .map(|(symbol, operand)| {
                        //         format!("{} -> {}", symbol, operand)
                        //     })
                        //     .collect::<Vec<String>>()
                        //     .join("\n")
                    );
                }
                Cmd::PrintVersion => {
                    println!(env!("CARGO_PKG_VERSION"));
                }
                Cmd::Quit => {
                    return Ok(());
                }
            }

            continue;
        }

        match oracle_backend::process(input) {
            Ok(ast) => {
                // if let Err(e) = interp.eval_ast(ast) {
                //     print_error(e);
                // }
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
        "h" | "help" => {
            Some(Cmd::Help)
        }
        "a" | "print-aliases" => {
            Some(Cmd::PrintAliases)
        }
        "v" | "print-version" => {
            Some(Cmd::PrintVersion)
        }
        "q" | "quit" => {
            Some(Cmd::Quit)
        }
        _ => None,
    }
}

enum Cmd {
    Help,
    PrintAliases,
    PrintVersion,
    Quit,
}

fn print_usage() {
    println!("Commands:");
    println!("  :h, :help               Displays this usage information.");
    println!("  :a, :print-aliases      Prints all defined aliases.");
    println!("  :v, :print-version      Prints the version number.");
    println!("  :q, :quit               Exits the program.");
}
