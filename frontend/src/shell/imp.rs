pub fn process(input: &str) {
    let result = imp_backend::process(
        input,
        imp_backend::Callbacks {
            a: Some(|out| {
                // println!("A: {}", out);
            }),
            b: Some(|out| {
                // println!("B: {}", out);
            }),
            c: Some(|out| {
                // println!("C: {}", out);
            }),
        },
    );

    match result {
        Ok(op) => {
            for output in op {
                print_output(output);
            }
        }
        Err(e) => {
            use crate::err::{self, Stage};
            use imp_backend::Error;

            print_span(&e.range);

            eprintln!(
                "{}",
                err::BackendError {
                    stage: match e.inner {
                        Error::A(_) => Stage::A,
                        Error::C(_) => Stage::C,
                        Error::D(_) => Stage::D,
                    },
                    inner: e.inner,
                },
            )
        }
    }
}

fn print_output(output: imp_backend::d::Output) {
    print!(
        "  {} ",
        crate::color(
            supports_color::Stream::Stdout,
            "=".to_string(),
            ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow)
        )
    );

    match output {
        imp_backend::d::Output::Text(text) => {
            print!("{}", text);
        }
        imp_backend::d::Output::Graphic => {
            todo!()
        }
    }

    println!()
}

fn print_span(range: &std::ops::Range<usize>) {
    // Match the shell prompt (` >`).
    // TODO: automatically update this value based on the shell prompt; doing so establishes, e.g.,
    // the [`print_shell_prompt`] function, as the single source of truth.
    eprint!("  ");
    // Print leading whitespace.
    eprint!("{0:<1$}", "", range.start);
    // Print the span.
    eprintln!(
        "{0:<^1$}",
        crate::color(
            supports_color::Stream::Stderr,
            "^".to_string(),
            ansi_term::Style::new().bold().fg(ansi_term::Color::Cyan),
        ),
        range
            .end
            .checked_sub(range.start)
            .expect("span range is inverted"),
    );
}
