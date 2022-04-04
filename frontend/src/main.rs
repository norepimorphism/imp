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
mod cmd;
mod config;
mod err;
mod imp;

use args::Args;
use config::Config;
use std::{
    io::{self, Write as _},
    process::ExitCode,
};

fn main() -> ExitCode {
    if let Err(e) = main_impl() {
        eprintln!("{}", e);

        ExitCode::from(e.exit_code())
    } else {
        ExitCode::SUCCESS
    }
}

fn main_impl() -> Result<(), err::FrontendError> {
    let args = Args::get()
        .map_err(err::FrontendError::Args)?;

    if args.should_print_vers {
        print_version();
    } else {
        let config = args.config_filepath
            .map(|it| Config::read(it).map_err(err::FrontendError::Config))
            .unwrap_or_else(|| Ok(Config::default()))?;

        let shell = Shell { config };
        loop {
            shell.do_shell();
        }
    }

    Ok(())
}

fn print_version() {
    println!(env!("CARGO_PKG_VERSION"));
}

struct Shell {
    config: Config,
}

impl Shell {
    /// Prints the shell prompt, reads user input, and executes the appropriate processor function.
    fn do_shell(&self) {
        self.print_prompt();
        let user_input = Self::read_user_input();

        if cmd::is_cmd(user_input.as_str()) {
            cmd::process(user_input.as_str());
        } else {
            imp::process(user_input.as_str());
        }
    }

    fn print_prompt(&self) {
        print!(
            "{}{}",
            color(
                supports_color::Stream::Stdout,
                ">".to_string(),
                ansi_term::Style::new().bold().fg(self.config.colors.prompt),
            ),
            std::iter::repeat(' ').take(self.config.prompt.padding).collect::<String>()
        );
    }

    fn read_user_input() -> String {
        let _ = io::stdout().lock().flush();
        let mut input = String::new();
        // TODO: Handle error.
        let _ = io::stdin().read_line(&mut input);

        input
    }
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
