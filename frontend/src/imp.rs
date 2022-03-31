use imp_backend::{e::Interp};

pub fn process(interp: &mut Interp, input: &str) {
    let result = imp_backend::process(
        input,
        imp_backend::Callbacks {
            a: Some(|out| {
                println!("{:#?}", out);
            }),
            b: Some(|out| {
                println!("{:#?}", out);
            }),
            c: Some(|out| {
                println!("{:#?}", out);
            }),
        },
    );

    match result {
        Ok(op) => {
            // print_result(op);
        }
        Err(e) => {
            print_span(&e.range);
            crate::print_error(e);
        }
    }
}

// fn print_result(op: Operand) {
//     println!(
//         "  {} {}",
//         crate::color(
//             supports_color::Stream::Stdout,
//             "=".to_string(),
//             ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow)
//         ),
//         op,
//     );
// }

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
