//! Interactive Mathematical Processor (IMP).
//!
//! This is the reference implementation of an IMP frontend. It interprets IMP expressions
//! line-by-line in a shell-like, command-line interface.

#![feature(process_exitcode_placeholder)]

mod cmd;
mod imp;

use std::{
    io::{self, Write as _},
    process::ExitCode,
};

fn main() -> ExitCode {
    main_impl();

    ExitCode::SUCCESS
}

fn main_impl() {
    let mut interp = imp_backend::Interp::default();
    loop {
        do_shell(&mut interp);
    }
}

fn do_shell(interp: &mut imp_backend::Interp) {
    print_shell_prompt();
    let user_input = read_user_input();

    if cmd::is_cmd(user_input.as_str()) {
        cmd::process(interp, user_input.as_str());
    } else {
        imp::process(interp, user_input.as_str());
    }
}

fn print_shell_prompt() {
    print!(
        "{} ",
        color(
            supports_color::Stream::Stdout,
            ">".to_string(),
            ansi_term::Style::new().bold().fg(ansi_term::Color::Green),
        )
    );
}

fn read_user_input() -> String {
    let _ = io::stdout().lock().flush();
    let mut input = String::new();
    // TODO: Handle error.
    let _ = io::stdin().read_line(&mut input);

    input
}

// Prints an error message.
fn print_error(e: impl std::error::Error) {
    eprintln!(
        "{}: {}",
        color(
            supports_color::Stream::Stderr,
            "error".to_string(),
            ansi_term::Style::new().bold().fg(ansi_term::Color::Red),
        ),
        e,
    );
}

fn color(stream: supports_color::Stream, input: String, style: ansi_term::Style) -> String {
    if supports_ansi_color(stream) {
        format!("{}", style.paint(input))
    } else {
        input
    }
}

fn supports_ansi_color(stream: supports_color::Stream) -> bool {
    supports_color::on_cached(stream).map_or(false, |it| it.has_basic)
}
