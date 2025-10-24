use crate::types::*;
use plotly::common::{Font, Marker, Mode, Title};
use plotly::layout::{Axis, Margin};
use plotly::{Layout, Plot, Scatter};
use plotters::coord::Shift;
use plotters::prelude::*;
use std::path::Path;

fn square_side(view: &PlotView) -> (u32, u32) {
	let side = view.width_px.min(view.height_px);
	(side, side)
}

fn data_y_range(points: &[(f64, f64)]) -> (f64, f64) {
	if points.is_empty() {
		return (-4.0, 4.0);
	}
	let mut min_y = points[0].1;
	let mut max_y = points[0].1;
	for &(_, y) in points.iter().skip(1) {
		if y < min_y {
			min_y = y;
		}
		if y > max_y {
			max_y = y;
		}
	}
	if min_y == max_y {
		min_y -= 1.0;
		max_y += 1.0;
	}
	let span = (max_y - min_y).max(0.2);
	let pad = 0.05 * span;
	let y_min = (min_y - pad).floor();
	let y_max = (max_y + pad).ceil();
	(y_min, y_max)
}

fn integer_ticks_in_range(min: f64, max: f64) -> Vec<i32> {
	let start = min.ceil();
	let end = max.floor();
	if start > end {
		return Vec::new();
	}
	let mut ticks = Vec::new();
	let mut value = start as i32;
	let end_value = end as i32;
	while value <= end_value {
		ticks.push(value);
		value += 1;
	}
	ticks
}

fn marker_radius(px: u32) -> i32 {
	if px >= 1000 { 2 } else { 1 }
}

fn draw_static_chart<B: DrawingBackend>(
	area: DrawingArea<B, Shift>,
	view: &PlotView,
	points: &[(f64, f64)],
	y_min: f64,
	y_max: f64,
	rx: &[i32],
	ry: &[i32],
	radius: i32,
) {
	let _ = area.fill(&WHITE);
	let mut builder = ChartBuilder::on(&area);
	builder.margin(20).caption(view.title.clone(), ("sans-serif", 28));
	builder
		.set_label_area_size(LabelAreaPosition::Left, 110)
		.set_label_area_size(LabelAreaPosition::Bottom, 90);
	if let Ok(mut chart) = builder.build_cartesian_2d(-4.0..4.0, y_min..y_max) {
		let mut mesh = chart.configure_mesh();
		let _ = mesh
			.disable_mesh()
			.x_desc("θ (radians)")
			.y_desc("ω (radians/s)")
			.axis_desc_style(("sans-serif", 22))
			.label_style(("sans-serif", 18))
			.x_labels(rx.len())
			.x_label_formatter(&|v| format!("{:.0}", v))
			.y_labels(ry.len().max(2))
			.y_label_formatter(&|v| format!("{:.0}", v))
			.draw();
		let style = BLACK.filled();
		let _ = chart.draw_series(points.iter().map(|(x, y)| Circle::new((*x, *y), radius, style)));
		let frame_style = ShapeStyle::from(&BLACK).stroke_width(2);
		let _ = chart
			.plotting_area()
			.draw(&Rectangle::new([(-4.0, y_min), (4.0, y_max)], frame_style));
	}
	let _ = area.present();
}


pub fn save_static(points: &[(f64, f64)], view: &PlotView, out_png: &str, out_svg: &str) {
	let (w, h) = square_side(view);
	let (y_min, y_max) = data_y_range(points);
	let x_ticks: Vec<i32> = (-4..=4).collect();
	let y_ticks = integer_ticks_in_range(y_min, y_max);
	let r = marker_radius(w);
	let png_backend = BitMapBackend::new(out_png, (w, h));
	let svg_backend = SVGBackend::new(out_svg, (w, h));
	draw_static_chart(png_backend.into_drawing_area(), view, points, y_min, y_max, &x_ticks, &y_ticks, r);
	draw_static_chart(svg_backend.into_drawing_area(), view, points, y_min, y_max, &x_ticks, &y_ticks, r);
}


pub fn save_html(points: &[(f64, f64)], view: &PlotView, out_html: &str) {
	let (w, h) = square_side(view);
	let (y_min, y_max) = data_y_range(points);
	let xs: Vec<f64> = points.iter().map(|(x, _)| *x).collect();
	let ys: Vec<f64> = points.iter().map(|(_, y)| *y).collect();
	let mut x_tick_vals = Vec::new();
	for v in -4..=4 {
		x_tick_vals.push(v as f64);
	}
	let y_tick_vals: Vec<f64> = integer_ticks_in_range(y_min, y_max)
		.into_iter()
		.map(|v| v as f64)
		.collect();
	let trace = Scatter::new(xs, ys)
		.mode(Mode::Markers)
		.marker(Marker::new().size(3).opacity(0.8).color("black"));
	let x_axis = Axis::new()
		.range(vec![-4.0, 4.0])
		.show_grid(false)
		.zero_line(false)
		.show_line(true)
		.mirror(true)
		.line_color("black")
		.tick_values(x_tick_vals.clone())
		.tick_text(x_tick_vals.iter().map(|v| format!("{:.0}", v)).collect())
		.tick_font(Font::new().size(18))
		.title(Title::new("θ (radians)").font(Font::new().size(22)));
	let y_axis = {
		let axis = Axis::new()
			.range(vec![y_min, y_max])
			.show_grid(false)
			.zero_line(false)
			.show_line(true)
			.mirror(true)
			.line_color("black")
			.tick_font(Font::new().size(18))
			.title(Title::new("ω (radians/s)").font(Font::new().size(22)));
		if y_tick_vals.is_empty() {
			axis
		} else {
			axis
				.tick_values(y_tick_vals.clone())
				.tick_text(y_tick_vals.iter().map(|v| format!("{:.0}", v)).collect())
		}
	};
	let layout = Layout::new()
		.title(Title::new(&view.title).font(Font::new().size(28)))
		.width(w as usize)
		.height(h as usize)
		.margin(Margin::new().left(110).right(30).top(60).bottom(90))
		.x_axis(x_axis)
		.y_axis(y_axis);
	let mut plot = Plot::new();
	plot.add_trace(trace);
	plot.set_layout(layout);
	plot.write_html(out_html);
}

pub fn save_all(points: &[(f64, f64)], view: &PlotView, out_base: &str) {
	let output_dir = Path::new("output");
	std::fs::create_dir_all(output_dir).expect("failed to create output directory");
	let base_path = output_dir.join(out_base);
	let base_str = base_path.to_string_lossy().into_owned();
	let out_png = format!("{base_str}.png");
	let out_svg = format!("{base_str}.svg");
	let out_html = format!("{base_str}.html");
	save_static(points, view, &out_png, &out_svg);
	save_html(points, view, &out_html);
}
