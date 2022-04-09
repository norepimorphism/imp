// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::Shell;

pub fn process(this: &Shell, input: &str) {
    let result = imp_backend::process(
        input,
        imp_backend::Callbacks {
            lexer: Some(|out| {
                // println!("A: {}", out);
            }),
            diet: Some(|out| {
                // println!("B: {}", out);
            }),
            parser: Some(|out| {
                // println!("C: {}", out);
            }),
        },
    );

    match result {
        Ok(op) => {
            for output in op {
                print_output(this, output);
            }
        }
        Err(e) => {
            use crate::err::{self, Stage};
            use imp_backend::Error;

            print_span(this, &e.range);

            eprintln!(
                "{}",
                err::BackendError {
                    stage: match e.inner {
                        Error::Lexer(_) => Stage::A,
                        Error::Parser(_) => Stage::C,
                        Error::Interp(_) => Stage::D,
                    },
                    inner: e.inner,
                },
            )
        }
    }
}

fn print_output(this: &Shell, output: imp_backend::interp::Output) {
    print!(
        "  {} ",
        crate::color(
            supports_color::Stream::Stdout,
            "=".to_string(),
            ansi_term::Style::new().bold().fg(this.config.output.color)
        )
    );

    match output {
        imp_backend::interp::Output::Text(text) => {
            print!("{}", text);
        }
        imp_backend::interp::Output::Graphic => {
            todo!()
        }
    }

    println!()
}

fn print_span(this: &Shell, range: &std::ops::Range<usize>) {
    // Match the shell prompt (` >`).
    eprint!("{}", this.prompt_padding());
    // Print leading whitespace.
    eprint!("{0:<1$}", "", range.start);
    // Print the span.
    eprintln!(
        "{0:<^1$}",
        crate::color(
            supports_color::Stream::Stderr,
            "^".to_string(),
            ansi_term::Style::new().bold().fg(this.config.spans.color),
        ),
        range
            .end
            .checked_sub(range.start)
            .expect("span range is inverted"),
    );
}
