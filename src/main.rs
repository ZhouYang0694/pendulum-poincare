use pendulum_poincare::{load_run_spec, run, save_all};

fn main() {
	let mut args = std::env::args();
	let _ = args.next();
	let path = args.next().unwrap_or_else(|| "run.json".to_string());
	let spec = load_run_spec(&path);
	let samples = run(&spec);
	let points: Vec<(f64, f64)> = samples.iter().map(|s| (s.theta, s.omega)).collect();
	save_all(&points, &spec.plot, &spec.output.out_base);
}
