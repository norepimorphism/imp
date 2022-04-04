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

#![feature(let_else, process_exitcode_placeholder)]

mod args;
mod config;
mod err;
mod shell;

use args::Args;
use config::Config;
use shell::Shell;
use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = main_impl() {
        // An error has occured in the frontend. Unlike backend errors---which only live for the
        // lifetime of one line of user input---frontend errors are fatal.

        eprintln!("{}", e);

        ExitCode::from(e.exit_code())
    } else {
        ExitCode::SUCCESS
    }
}

fn main_impl() -> Result<(), err::FrontendError> {
    // Windows requires ANSI escape codes support to be explicitly enabled. This is a no-op on non-
    // Windows platforms.
    let _ = enable_ansi_support::enable_ansi_support();

    let args = Args::get().map_err(err::FrontendError::Args)?;

    if args.should_print_vers {
        print_version();

        return Ok(());
    }

    let config = args
        .config_filepath
        .map(|it| Config::read(it).map_err(err::FrontendError::Config))
        .unwrap_or_else(|| {
            // A configuration file was not specified, so defaults will be used.
            Ok(Config::default())
        })?;

    let shell = Shell::new(config);
    loop {
        shell.interpret_line();
    }
}

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}

fn color(stream: supports_color::Stream, input: String, style: ansi_term::Style) -> String {
    if supports_ansi_color(stream) {
        style.paint(input).to_string()
    } else {
        input
    }
}

fn supports_ansi_color(stream: supports_color::Stream) -> bool {
    // TODO: This only checks for basic ANSI color support, but the configuration file can specify
    // 8-bit colors. At the same time, color shouldn't be disabled if the terminal doesn't support
    // 8-bit colors and the user only needs basic colors.
    supports_color::on_cached(stream).map_or(false, |it| it.has_basic)
}
