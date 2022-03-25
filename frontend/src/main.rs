//! Interactive Mathematical Calculator (IMC).
//!
//!

#![feature(process_exitcode_placeholder)]

use ansi_term::{Color, Style};
use oracle_backend::{Error, Interp};
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
    let mut interp = Interp::default();

    loop {
        // Print shell prompt.
        print!("{} ", color(
            Stream::Stdout,
            ">".to_string(),
            Style::new().bold().fg(Color::Green),
        ));

        // Read user input.
        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        if input.starts_with(':') {
            let cmd = input
                // Remove leading colon.
                .trim_start_matches(':')
                // Remove line feed.
                // TODO: This is only necessary because [`process_cmd`] is not smart enough to ignore
                // whitespace, and so command matching fails. Ideally, [`process_cmd`] will have its own
                // lexer and parser to manage these quirks.
                .trim_end();

            process_cmd(&interp, cmd);
        } else {
            if let Err(e) = oracle_backend::process(input.as_str())
                .map(|ast| interp.eval_ast(ast))
            {
                print_error(e);
            }
        }
    }
}

fn print_error(e: Error) {
    // Match the shell prompt (` >`).
    eprint!("  ");
    // Print leading whitespace.
    eprint!("{0:<1$}", "", e.range.start);
    // Print the span.
    eprintln!(
        "{0:<^1$}",
        color(
            Stream::Stderr,
            "^".to_string(),
            Style::new().bold().fg(Color::Cyan),
        ),
        e.range.end
            .checked_sub(e.range.start)
            .expect("span range is inverted"),
    );
    // Print the error message.
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

fn process_cmd(interp: &Interp, cmd: &str) {
    match cmd {
        "h" | "help" => {
            print_usage()
        }
        "a" | "print-aliases" => {
            print_aliases(interp)
        }
        "v" | "print-version" => {
            print_version()
        }
        // TODO: Handle invalid commands.
        _ => ()
    }
}

fn print_usage() {
    println!("Commands:");
    println!("  :h, :help               Displays this usage information.");
    println!("  :a, :print-aliases      Prints all defined aliases.");
    println!("  :v, :print-version      Prints the version number.");
}

fn print_aliases(interp: &Interp) {
    println!(
        "{}",
        interp.aliases()
            .map(|(symbol, operand)| {
                format!("{} -> {}", symbol, operand)
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}
