use imp_backend::{parser::Operand, Interp};

pub fn process(interp: &mut Interp, input: &str) {
    match imp_backend::process(interp, input) {
        Ok(op) => {
            print_result(op);

            // demo_plot();
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
            ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow)
        ),
        op,
    );
}

// fn demo_plot() {
//     use plotters::prelude::*;

//     let mut svg = String::new();
//     let root = plotters_svg::SVGBackend::with_string(&mut svg, (640, 480)).into_drawing_area();

//     let mut chart = ChartBuilder::on(&root)
//         .caption("y=x^2", ("sans-serif", 50).into_font())
//         .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
//         .unwrap();

//     chart.configure_mesh().draw().unwrap();

//     chart
//         .draw_series(LineSeries::new(
//             (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
//             &RED,
//         ))
//         .unwrap()
//         .label("y = x^2")
//         .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

//     chart
//         .configure_series_labels()
//         .background_style(&WHITE.mix(0.8))
//         .border_style(&BLACK)
//         .draw()
//         .unwrap();

//     drop(chart);
//     drop(root);

//     let img = nsvg::parse_str(svg.as_str(), nsvg::Units::Pixel, 96.0)
//         .unwrap()
//         .rasterize(1.0)
//         .unwrap();

//     let img = image::load_from_memory_with_format(&img, image::ImageFormat::Bmp).unwrap();

//     viuer::print(&img, &viuer::Config::default()).unwrap();
// }

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
        range
            .end
            .checked_sub(range.start)
            .expect("span range is inverted"),
    );
}
