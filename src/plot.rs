use crate::types::*;
use plotly::common::{Mode, Title};
use plotly::layout::Axis;
use plotly::{Layout, Plot, Scatter};
use plotters::coord::Shift;
use plotters::prelude::*;
use std::f64::consts::PI;

fn draw_chart<B: DrawingBackend>(area: DrawingArea<B, Shift>, view: &PlotView, points: &[(f64, f64)]) {
	let _ = area.fill(&WHITE);
	if let Ok(mut chart) = ChartBuilder::on(&area)
		.margin(20)
		.caption(view.title.clone(), ("sans-serif", 20))
		.build_cartesian_2d(-PI..PI, view.omega_min..view.omega_max)
	{
		let _ = chart.configure_mesh().disable_mesh().draw();
		let series = points.iter().map(|(x, y)| Circle::new((*x, *y), 2, RED.filled()));
		let _ = chart.draw_series(series);
	}
	let _ = area.present();
}

pub fn save_static(points: &[(f64, f64)], view: &PlotView, out_png: &str, out_svg: &str) {
	let png_backend = BitMapBackend::new(out_png, (view.width_px as u32, view.height_px as u32));
	let svg_backend = SVGBackend::new(out_svg, (view.width_px, view.height_px));
	draw_chart(png_backend.into_drawing_area(), view, points);
	draw_chart(svg_backend.into_drawing_area(), view, points);
}

pub fn save_html(points: &[(f64, f64)], view: &PlotView, out_html: &str) {
	let xs: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
	let ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
	let trace = Scatter::new(xs, ys).mode(Mode::Markers);
	let layout = Layout::new()
		.title(Title::new(&view.title))
		.x_axis(Axis::new().range(vec![-PI, PI]))
		.y_axis(Axis::new().range(vec![view.omega_min, view.omega_max]));
	let mut plot = Plot::new();
	plot.add_trace(trace);
	plot.set_layout(layout);
	plot.write_html(out_html);
}

pub fn save_all(points: &[(f64, f64)], view: &PlotView, out_base: &str) {
	let out_png = format!("{}.png", out_base);
	let out_svg = format!("{}.svg", out_base);
	let out_html = format!("{}.html", out_base);
	save_static(points, view, &out_png, &out_svg);
	save_html(points, view, &out_html);
}
