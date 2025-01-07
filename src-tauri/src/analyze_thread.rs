use plotly::common::Mode;
use plotly::{Plot, Scatter};
// use std::fs;

#[tauri::command]
pub fn generate_plot() -> String {
    let trace = Scatter::new(vec![1, 2, 3], vec![4, 5, 6])
        .mode(Mode::LinesMarkers);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    return plot.to_inline_html(None);
}

// fn main() {
//     let plot_html = generate_plot();
//     fs::write("plot.html", &plot_html).expect("Unable to write file");
// }