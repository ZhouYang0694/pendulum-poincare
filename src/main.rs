use pendulum_poincare::{load_run_spec, run, save_all, save_all_x};

fn main() {
	let mut args = std::env::args();
	let _ = args.next();
	let path = args.next().unwrap_or_else(|| "run.json".to_string());
	let spec = load_run_spec(&path);
	let samples = run(&spec);
	let points: Vec<(f64, f64)> = samples.iter().map(|s| (s.theta, s.omega)).collect();
	save_all(&points, &spec.plot, &spec.output.out_base);
	let points_roi: Vec<(f64, f64)> = points.iter().copied().filter(|(theta, _)| *theta > 2.0).collect();
	let roi_base = format!("{}__theta_gt_2", spec.output.out_base);
	save_all_x(&points_roi, &spec.plot, &roi_base, 1.9, 3.3);
}
