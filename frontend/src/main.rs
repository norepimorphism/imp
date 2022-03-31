//! Interactive Mathematical Processor (IMP).
//!
//! This is the reference implementation of an [IMP] frontend. It interprets IMP expressions
//! line-by-line in a shell-like, command-line interface. IMP is stateless; however, configuration
//! is possible through a TOML configuration file.
//!
//! By default, output is colored with ANSI color codes if IMP determines that the containing
//! terminal supports them (see the [supports-color] crate). Interpreter errors are visualized with
//! a red error message and cyan span markers (`^`) pointing to the area of concern from the
//! previous line.
//!
//! Colored output may be disabled by defining a `NO_COLOR` environment variable or disabling color
//! in the configuration file. The color of the shell prompt and error messages is customizable
//! through the configuration file.
//!
//! [IMP]: https://crates.io/crates/imp-backend
//! [supports-color]: https://crates.io/crates/supports-color

#![feature(process_exitcode_placeholder)]

mod cmd;
mod imp;

use imp_backend::e::Interp;
use std::{
    io::{self, Write as _},
    process::ExitCode,
};

fn main() -> ExitCode {
    // This interpreter lives for the entire duration of the program. It is continually accessed and
    // mutated by [shell commands](cmd::process) and [IMPL expressions](imp::process).
    let mut interp = Interp::default();
    loop {
        do_shell(&mut interp);
    }

    // Note: don't bother removing this line; it is likely that an error within this scope will be
    // possible at some point.
    ExitCode::SUCCESS
}

/// Prints the shell prompt, reads user input, and executes the appropriate processor function.
fn do_shell(interp: &mut Interp) {
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
