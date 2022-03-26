use imp_backend::{Interp, op::Operand};

pub fn process(interp: &mut Interp, input: &str) {
    match imp_backend::process(input).and_then(|ast| interp.eval_ast(ast)) {
        Ok(ops) => {
            for op in ops {
                print_result(op);
            }
        }
        Err(e) => {
            print_span(&e.range);
            crate::print_error(e);
        }
    }
}

fn print_result(op: Operand) {
    println!(
        "  {} {}",
        crate::color(
            supports_color::Stream::Stdout,
            "=".to_string(),
            ansi_term::Style::new().bold().fg(ansi_term::Color::Purple)
        ),
        op,
    );
}

fn print_span(range: &std::ops::Range<usize>) {
    // Match the shell prompt (` >`).
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
        range.end
            .checked_sub(range.start)
            .expect("span range is inverted"),
    );
}
