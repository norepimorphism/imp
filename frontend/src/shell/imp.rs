// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use imp_backend::span::Span;
use super::Shell;

pub fn process(this: &Shell, input: &str) {
    let result = process_through_backend(input);

    handle_backend_result(this, result);
}

fn process_through_backend(input: &str) -> Result<Vec<imp_backend::evaluator::Output>, Span<imp_backend::Error>> {
    imp_backend::process(
        input,
        imp_backend::Callbacks {
            inspect_lexer_output: Some(|out| {

            }),
            inspect_parser_output: Some(|out| {

            }),
        },
    )
}

fn handle_backend_result(
    this: &Shell,
    result: Result<Vec<imp_backend::evaluator::Output>, Span<imp_backend::Error>>,
) {
    match result {
        Ok(outputs) => {
            handle_backend_success(this, outputs);
        }
        Err(e) => {
            handle_backend_error(this, e);
        }
    }
}

fn handle_backend_success(this: &Shell, outputs: Vec<imp_backend::evaluator::Output>) {
    for output in outputs {
        print_output(this, output);
    }
}

fn print_output(this: &Shell, output: imp_backend::evaluator::Output) {
    print_output_eq_sign(this);
    print_output_value(output);
    println!()
}

fn print_output_eq_sign(this: &Shell) {
    print!(
        "  {} ",
        crate::color(
            supports_color::Stream::Stdout,
            "=".to_string(),
            ansi_term::Style::new().bold().fg(this.config.output.color)
        )
    );
}

fn print_output_value(output: imp_backend::evaluator::Output) {
    match output {
        imp_backend::evaluator::Output::Text(text) => {
            print_output_text(text.as_str());
        }
        imp_backend::evaluator::Output::Graphic => {
            print_output_graph(());
        }
    }
}

fn print_output_text(text: &str) {
    print!("{}", text);
}

fn print_output_graph(_: ()) {
    todo!()
}

fn handle_backend_error(this: &Shell, e: Span<imp_backend::Error>) {
    use crate::err::{self, BackendStage};
        use imp_backend::Error;

        print_span(this, &e.range);

        eprintln!(
            "{}",
            err::BackendError {
                stage: match e.inner {
                    Error::Lexer(_) => BackendStage::Lexer,
                    Error::Parser(_) => BackendStage::Parser,
                    Error::Evaluator(_) => BackendStage::Evaluator,
                },
                inner: e.inner,
            },
        )
}

fn print_span(this: &Shell, range: &std::ops::Range<usize>) {
    print_span_whitespace(this, range);
    print_span_underline(this, range);
}

fn print_span_whitespace(this: &Shell, range: &std::ops::Range<usize>) {
    // Match the shell prompt (`>`).
    eprint!(" {}", this.prompt_padding());
    // Print leading whitespace.
    eprint!("{0:<1$}", "", range.start);
}

fn print_span_underline(this: &Shell, range: &std::ops::Range<usize>) {
    eprintln!(
        "{}",
        crate::color(
            supports_color::Stream::Stderr,
            create_span_underline(range),
            ansi_term::Style::new().bold().fg(this.config.spans.color),
        ),
    );
}

fn create_span_underline(range: &std::ops::Range<usize>) -> String {
    std::iter::repeat('^')
        .take({
            range
                .end
                .checked_sub(range.start)
                .expect("span range is inverted")
        })
        .collect()
}
