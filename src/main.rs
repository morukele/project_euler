use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{BLUE, WHITE},
};

fn collatz_sequence(mut n: u64) -> Vec<u64> {
    let mut seq = vec![n];
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        seq.push(n);
    }
    seq
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = 1_000_000;
    let sequence = collatz_sequence(start);
    let path = format!("collatz_sequence_{num}.png", num = start);

    let root = BitMapBackend::new(&path, (1280, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_value = *sequence.iter().max().unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Collatz Sequence for {}", start),
            ("sans-serif", 30),
        )
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..sequence.len(), 0u64..max_value)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        sequence.iter().enumerate().map(|(i, &v)| (i, v)),
        &BLUE,
    ))?;

    Ok(())
}
